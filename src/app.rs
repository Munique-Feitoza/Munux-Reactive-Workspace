// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

use crate::game::state::GameState;
use anyhow::Result;
use fluent::FluentArgs;
use std::collections::HashMap;
use std::path::PathBuf;

/// Enum que define os diferentes modos do Painel Direito (Reactive Panel)
#[derive(Debug, Clone, PartialEq)]
pub enum RightPanelMode {
    /// Tela inicial de boas-vindas
    Welcome,
    
    /// Modo padrão: mostra a árvore de arquivos do diretório atual
    FileTree { path: PathBuf },
    
    /// Mostra o preview de um arquivo com syntax highlighting
    FilePreview { 
        path: PathBuf, 
        content: String,
        language: String,
    },
    
    /// Mostra gráficos de recursos do sistema (CPU, RAM, Processos)
    ResourceMonitor { 
        cpu_usage: f32,
        memory_used: u64,
        memory_total: u64,
        process_count: usize,
    },
    
    /// Modo de alerta vermelho para comandos destrutivos
    DangerZone { 
        warning: String,
        command: String,
    },
    
    /// Mostra mensagens de gamificação (Level Up, conquistas)
    Gamification { 
        message: String,
        celebration: bool,
    },
    
    /// Mostra estatísticas do jogador
    Stats,
    
    /// Mostra quests/missões ativas
    Quests,
    
    /// Easter egg ativado
    EasterEgg {
        content: String,
    },
    
    /// Modo Help - mostra guias com scroll
    Help {
        content: String,
        title: String,
    },

    /// Modo de ajuda específica de comando
    CommandHelp {
        command: String,
        description: String,
        examples: Vec<String>,
        tip: String,
    },
    
    /// Output genérico de comando
    CommandOutput(String),
}

/// Estado da aplicação - "Single Source of Truth"
pub struct App {
    /// Buffer de input do usuário (o que ele está digitando)
    pub input_buffer: String,
    
    /// Histórico de comandos executados
    pub command_history: Vec<String>,
    
    /// Índice atual no histórico (para navegação com setas)
    pub history_index: Option<usize>,
    
    /// Modo atual do painel direito
    pub right_panel_mode: RightPanelMode,
    
    /// Estado de gamificação (XP, nível, integridade)
    pub game_state: GameState,
    
    /// Diretório de trabalho atual
    pub current_dir: PathBuf,
    
    /// Output do último comando executado
    pub last_output: String,
    
    /// Flag para sair da aplicação
    pub should_quit: bool,
    
    /// Flag para indicar se o modo de perigo está ativo
    pub danger_mode_active: bool,
    
    /// Popup ativo (Ghost Mentor)
    pub active_popup: Option<PopupMessage>,

    /// Scroll vertical para o painel direito
    pub scroll: u16,

    /// Info do Git no diretório atual
    pub git_status: Option<crate::core::git::GitStatus>,

    /// Internacionalização
    pub i18n: crate::i18n::I18n,

    /// Sessão SSH ativa (se houver)
    pub ssh_session: Option<crate::core::ssh::SshSession>,

    /// Aliases definidos pelo usuário (`nome` -> `comando`)
    pub aliases: HashMap<String, String>,

    /// Passo atual do tutorial interativo (None = tutorial inativo)
    pub tutorial: Option<usize>,

    /// Teste de digitação em andamento (None = benchmark inativo)
    pub benchmark: Option<crate::game::benchmark::BenchmarkState>,
}

/// Mensagem de popup para o Ghost Mentor
#[derive(Debug, Clone)]
pub struct PopupMessage {
    pub title: String,
    pub content: String,
    pub popup_type: PopupType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PopupType {
    Info,
    Warning,
    Success,
    Tip,
}

impl App {
    /// Cria uma nova instância do App, carregando o progresso salvo se existir
    pub fn new() -> Result<Self> {
        let current_dir = std::env::current_dir()?;

        let git_status = crate::core::git::GitManager::get_status(&current_dir);

        let i18n = crate::i18n::I18n::new(crate::i18n::Language::detect());

        // Tenta carregar o progresso salvo; em caso de save ausente ou inválido,
        // começa um estado novo.
        let (mut game_state, command_history, aliases, restored) =
            match crate::core::persistence::load() {
                Ok(Some(data)) => (data.game_state, data.command_history, data.aliases, true),
                _ => (GameState::new(&i18n), Vec::new(), HashMap::new(), false),
            };

        if restored {
            game_state.update_daily_streak();
        }

        Ok(Self {
            input_buffer: String::new(),
            command_history,
            history_index: None,
            right_panel_mode: RightPanelMode::Welcome,
            game_state,
            current_dir,
            last_output: String::new(),
            should_quit: false,
            danger_mode_active: false,
            active_popup: None,
            scroll: 0,
            git_status,
            i18n,
            ssh_session: None,
            aliases,
            tutorial: None,
            benchmark: None,
        })
    }

    /// Grava o progresso atual em disco (chamado após cada comando e ao sair).
    /// Falhas de I/O são silenciosas para não interromper a sessão do usuário.
    pub fn save_progress(&self) {
        let _ = crate::core::persistence::save(&self.game_state, &self.command_history, &self.aliases);
    }

    /// Expande recursivamente o alias da primeira palavra de `command`.
    ///
    /// Ex.: com `gs` -> `git status`, a entrada `gs -s` vira `git status -s`.
    /// A profundidade é limitada para evitar loops (`a=b`, `b=a`).
    fn expand_alias(&self, command: &str) -> String {
        let mut current = command.trim().to_string();
        for _ in 0..10 {
            let (head, rest) = match current.split_once(char::is_whitespace) {
                Some((h, r)) => (h.to_string(), r.to_string()),
                None => (current.clone(), String::new()),
            };
            match self.aliases.get(&head) {
                Some(expansion) => {
                    current = if rest.is_empty() {
                        expansion.clone()
                    } else {
                        format!("{} {}", expansion, rest)
                    };
                }
                None => break,
            }
        }
        current
    }

    /// Trata os comandos `alias` / `unalias`. Retorna `true` se o comando foi
    /// um comando de alias (e já foi processado).
    fn handle_alias_command(&mut self, command: &str) -> bool {
        let trimmed = command.trim();

        // `alias` ou `aliases`: lista todos.
        if trimmed == "alias" || trimmed == "aliases" {
            if self.aliases.is_empty() {
                self.last_output = "Nenhum alias definido. Use: alias nome='comando'".to_string();
            } else {
                let mut lines: Vec<String> =
                    self.aliases.iter().map(|(k, v)| format!("  {} = {}", k, v)).collect();
                lines.sort();
                self.last_output = format!("📎 Aliases definidos:\n{}", lines.join("\n"));
            }
            self.clear_input();
            return true;
        }

        // `unalias nome`: remove.
        if let Some(name) = trimmed.strip_prefix("unalias ") {
            let name = name.trim();
            if self.aliases.remove(name).is_some() {
                self.last_output = format!("✓ Alias '{}' removido.", name);
            } else {
                self.last_output = format!("✗ Alias '{}' não existe.", name);
            }
            self.clear_input();
            return true;
        }

        // `alias nome=comando` ou `alias nome='comando com args'`.
        if let Some(rest) = trimmed.strip_prefix("alias ") {
            if let Some((name, value)) = rest.split_once('=') {
                let name = name.trim();
                let value = value.trim().trim_matches(|c| c == '\'' || c == '"').trim();
                if name.is_empty() || value.is_empty() {
                    self.last_output = "Uso: alias nome='comando'".to_string();
                } else if name.contains(char::is_whitespace) {
                    self.last_output = "✗ O nome do alias não pode conter espaços.".to_string();
                } else {
                    self.aliases.insert(name.to_string(), value.to_string());
                    self.last_output = format!("✓ Alias criado: {} = {}", name, value);
                }
            } else {
                self.last_output = "Uso: alias nome='comando'".to_string();
            }
            self.clear_input();
            return true;
        }

        false
    }

    /// Trata os comandos `tutorial` / `tutorial sair`. Retorna `true` se o
    /// comando foi de tutorial (e já foi processado).
    fn handle_tutorial_command(&mut self, command: &str) -> bool {
        let trimmed = command.trim();
        let first = trimmed.split_whitespace().next().unwrap_or("");
        if first != "tutorial" {
            return false;
        }

        let arg = trimmed.split_whitespace().nth(1).unwrap_or("");
        match arg {
            "sair" | "exit" | "stop" | "parar" => {
                if self.tutorial.is_some() {
                    self.tutorial = None;
                    self.last_output =
                        "🎓 Tutorial encerrado. Volte quando quiser: 'tutorial'.".to_string();
                } else {
                    self.last_output = "Nenhum tutorial em andamento.".to_string();
                }
            }
            _ => {
                self.tutorial = Some(0);
                self.last_output = "🎓 Tutorial iniciado! Siga as instruções no quadro.".to_string();
                self.show_popup(
                    "🎓 Modo Tutorial".to_string(),
                    crate::game::tutorial::step_text(0),
                    PopupType::Tip,
                );
            }
        }

        self.clear_input();
        true
    }

    /// Verifica se `command` completa o passo atual do tutorial e, em caso
    /// afirmativo, avança (ou conclui o tutorial com bônus de XP).
    fn advance_tutorial(&mut self, command: &str) {
        use crate::game::tutorial;

        let Some(step_idx) = self.tutorial else { return };
        let Some(step) = tutorial::STEPS.get(step_idx) else {
            self.tutorial = None;
            return;
        };
        if !step.matches(command) {
            return;
        }

        let next = step_idx + 1;
        if next < tutorial::STEPS.len() {
            self.tutorial = Some(next);
            self.show_popup(
                "✅ Passo concluído!".to_string(),
                tutorial::step_text(next),
                PopupType::Success,
            );
        } else {
            self.tutorial = None;
            self.game_state.add_xp(tutorial::COMPLETION_XP);
            self.show_popup(
                "🎉 Tutorial concluído!".to_string(),
                format!(
                    "Parabéns! Você dominou o básico do Munux.\n\n+{} XP de bônus!\n\nAgora explore à vontade — use 'help' sempre que precisar.",
                    tutorial::COMPLETION_XP
                ),
                PopupType::Success,
            );
        }
    }

    /// Trata o comando `benchmark` (iniciar) e `benchmark sair`. Retorna `true`
    /// se o comando foi de benchmark e já foi processado. A *avaliação* da frase
    /// digitada acontece em `finish_benchmark`, chamada no início de
    /// `execute_command` quando um benchmark está ativo.
    fn handle_benchmark_command(&mut self, command: &str) -> bool {
        let trimmed = command.trim();
        if trimmed.split_whitespace().next() != Some("benchmark") {
            return false;
        }

        match trimmed.split_whitespace().nth(1).unwrap_or("") {
            "sair" | "exit" | "stop" | "parar" => {
                self.last_output = "Nenhum benchmark em andamento.".to_string();
            }
            _ => {
                let state = crate::game::benchmark::BenchmarkState::start();
                self.last_output = format!(
                    "⏱️ BENCHMARK DE DIGITAÇÃO\n\nDigite a frase abaixo e pressione Enter:\n\n  {}\n\n('benchmark sair' cancela)",
                    state.prompt
                );
                self.show_popup(
                    "⏱️ Benchmark de Digitação".to_string(),
                    format!(
                        "Digite exatamente esta frase e pressione Enter:\n\n{}\n\nO cronômetro já começou! ('benchmark sair' para cancelar)",
                        state.prompt
                    ),
                    PopupType::Info,
                );
                self.benchmark = Some(state);
            }
        }

        self.clear_input();
        true
    }

    /// Avalia o texto digitado contra a frase do benchmark ativo, mostra o
    /// resultado e concede XP.
    fn finish_benchmark(&mut self, typed: &str) {
        let Some(state) = self.benchmark.take() else { return };
        let seconds = state.started_at.elapsed().as_secs_f64();
        let result = crate::game::benchmark::score(&state.prompt, typed.trim(), seconds);

        self.game_state.add_xp(result.xp);
        let summary = format!(
            "⏱️ {:.1}s  •  {} WPM  •  {}% de precisão  •  +{} XP",
            result.seconds, result.wpm, result.accuracy, result.xp
        );
        self.last_output = summary.clone();
        self.show_popup("⏱️ Resultado do Benchmark".to_string(), summary, PopupType::Success);
    }

    /// Atualiza o buffer de input (chamado a cada tecla)
    pub fn update_input(&mut self, character: char) {
        self.input_buffer.push(character);
        // Trigger de análise reativa do parser
        self.analyze_input();
    }
    
    /// Remove o último caractere do buffer
    pub fn delete_char(&mut self) {
        self.input_buffer.pop();
        self.analyze_input();
    }
    
    /// Limpa o buffer de input
    pub fn clear_input(&mut self) {
        self.input_buffer.clear();
        self.analyze_input();
    }
    
    /// Completa o input atual (Tab). Quando há mais de um candidato, estende o
    /// buffer até o maior prefixo comum e lista as opções no output.
    pub fn autocomplete(&mut self) {
        // Em sessão SSH a completação de caminhos seria contra o disco local
        // (errado), então só ajuda com nomes de comando — o módulo lida bem com
        // isso quando o diretório informado não tem entradas correspondentes.
        let result = crate::core::completion::complete(&self.input_buffer, &self.current_dir);
        self.input_buffer = result.new_input;

        if !result.suggestions.is_empty() {
            self.last_output = format!("⇥ {}", result.suggestions.join("   "));
        }

        self.history_index = None;
        self.analyze_input();
    }

    /// Navega para o comando anterior no histórico
    pub fn history_previous(&mut self) {
        if self.command_history.is_empty() {
            return;
        }
        
        let new_index = match self.history_index {
            None => Some(self.command_history.len() - 1),
            Some(0) => Some(0),
            Some(idx) => Some(idx - 1),
        };
        
        self.history_index = new_index;
        if let Some(idx) = new_index {
            self.input_buffer = self.command_history[idx].clone();
        }
    }
    
    /// Navega para o próximo comando no histórico
    pub fn history_next(&mut self) {
        if self.command_history.is_empty() {
            return;
        }
        
        match self.history_index {
            None => {},
            Some(idx) if idx >= self.command_history.len() - 1 => {
                self.history_index = None;
                self.input_buffer.clear();
            },
            Some(idx) => {
                self.history_index = Some(idx + 1);
                self.input_buffer = self.command_history[idx + 1].clone();
            },
        }
    }
    
    /// Executa o comando atual
    pub fn execute_command(&mut self) -> Result<()> {
        use crate::game::achievements::AchievementChecker;
        use crate::game::easter_eggs::EasterEggs;
        use crate::game::logic;
        use crate::core::parser::CommandParser;
        
        if self.input_buffer.trim().is_empty() {
            return Ok(());
        }

        // Modo benchmark ativo: a entrada é a frase digitada, não um comando.
        if self.benchmark.is_some() {
            let typed = self.input_buffer.clone();
            let t = typed.trim();
            if t == "benchmark sair" || t == "benchmark stop" || t == "benchmark exit" {
                self.benchmark = None;
                self.last_output = "⏱️ Benchmark cancelado.".to_string();
            } else {
                self.finish_benchmark(&typed);
            }
            self.clear_input();
            return Ok(());
        }

        // Adiciona ao histórico
        self.command_history.push(self.input_buffer.clone());
        self.history_index = None;
        
        let command = self.input_buffer.clone().trim().to_string();

        // Comandos de alias e expansão de aliases (não se aplicam em sessões SSH)
        if self.ssh_session.is_none() && self.handle_alias_command(&command) {
            return Ok(());
        }
        let command = if self.ssh_session.is_none() {
            self.expand_alias(&command)
        } else {
            command
        };

        // Benchmark: comando para iniciar o teste de digitação
        if self.ssh_session.is_none() && self.handle_benchmark_command(&command) {
            return Ok(());
        }

        // Tutorial interativo: comando de controle ou avanço de passo
        if self.ssh_session.is_none() {
            if self.handle_tutorial_command(&command) {
                return Ok(());
            }
            self.advance_tutorial(&command);
        }

        // Lógica de Sessão SSH
        if let Some(ssh_session) = &mut self.ssh_session {
            // Comandos locais dentro da sessão SSH
            if command == "exit" || command == "logout" {
                self.ssh_session = None;
                self.last_output = "🔌 Desconectado do servidor remoto.".to_string();
                self.right_panel_mode = RightPanelMode::Welcome;
                self.input_buffer.clear();
                return Ok(());
            }

            // Comandos remotos
            if command.starts_with("cd ") {
                let path = command.trim_start_matches("cd ").trim();
                match ssh_session.change_dir(path) {
                    Ok(_) => {
                        self.last_output = format!("✓ Diretório remoto alterado para: {}", ssh_session.remote_cwd);
                        self.right_panel_mode = RightPanelMode::CommandOutput(self.last_output.clone());
                    }
                    Err(e) => {
                        self.last_output = format!("✗ Erro: {}", e);
                    }
                }
            } else {
                // Executa qualquer outro comando no servidor
                // Injeta --color=always para ls e grep se não tiver
                let cmd_to_run = if (command.starts_with("ls ") || command == "ls") && !command.contains("--color") {
                    format!("ls --color=always {}", command.trim_start_matches("ls").trim())
                } else if command.starts_with("grep ") && !command.contains("--color") {
                    format!("grep --color=always {}", command.trim_start_matches("grep").trim())
                } else {
                    command.clone()
                };

                match ssh_session.execute(&cmd_to_run) {
                    Ok((stdout, stderr, _code)) => {
                        let output = if !stdout.is_empty() { stdout } else { stderr };
                        
                        // Formata o output com prompt remoto
                        let prompt = format!(
                            "\x1b[1;36m{}@{} \x1b[0m\x1b[1;33m{}\x1b[0m$ \x1b[1m{}\x1b[0m", 
                            ssh_session.user, ssh_session.host, ssh_session.remote_cwd, command
                        );
                        
                        self.last_output = Self::sanitize_output(&output);
                        self.right_panel_mode = RightPanelMode::CommandOutput(format!("{}\n{}", prompt, self.last_output));
                    }
                    Err(e) => {
                        self.last_output = format!("✗ Erro de execução remota: {}", e);
                    }
                }
            }

            self.input_buffer.clear();
            return Ok(());
        }

        // Comando para iniciar conexão SSH
        if command.starts_with("ssh ") {
            let parts: Vec<&str> = command.split_whitespace().collect();
            if parts.len() >= 2 {
                let target = parts[1]; // user@host
                if target.contains('@') {
                    let auth_parts: Vec<&str> = target.split('@').collect();
                    let user = auth_parts[0];
                    let host = auth_parts[1];
                    
                    self.last_output = format!("🔄 Conectando a {}@{}...", user, host);
                    // Renderiza um frame antes de bloquear na conexão (idealmente seria async, mas TUI é sync)
                    // Como não temos async runtime fácil aqui, vai bloquear brevemente
                    
                    match crate::core::ssh::SshSession::connect(user, host) {
                        Ok(session) => {
                            let cwd = session.remote_cwd.clone();
                            self.ssh_session = Some(session);
                            self.last_output = format!("✓ Conectado a {} em {}", host, cwd);
                            self.right_panel_mode = RightPanelMode::Welcome; // Ou algum modo específico SSH
                            
                            self.show_popup(
                                "Conexão Estabelecida".to_string(),
                                format!("Conectado com sucesso a {}@{}\n\nDiretório: {}", user, host, cwd),
                                PopupType::Success
                            );
                        }
                        Err(e) => {
                             self.last_output = format!("✗ Falha na conexão: {}", e);
                             self.show_popup(
                                "Erro de Conexão".to_string(),
                                format!("Não foi possível conectar a {}:\n{}", target, e),
                                PopupType::Warning
                            );
                        }
                    }
                    self.input_buffer.clear();
                    return Ok(());
                }
            }
        }
        let cmd_type = CommandParser::classify_command(&command);
        
        // Verifica modo de segurança
        if !CommandParser::is_safe_command(&command, self.game_state.safe_mode) {
            self.last_output = self.i18n.tc("sys-access-denied");
            // Mostra popup de aviso (usa PopupType::Warning)
            self.show_popup(
                self.i18n.tc("sys-access-denied-title"), 
                self.i18n.tc("sys-access-denied-body"), 
                PopupType::Warning
            );
            self.game_state.record_failure();
            return Ok(());
        }

        if matches!(cmd_type, crate::core::parser::CommandType::Dangerous) {
            self.game_state.damage_integrity(10);
        }
        
        // Verifica easter eggs primeiro
        if let Some(easter_egg_output) = EasterEggs::check(&command) {
            self.last_output = easter_egg_output.clone();
            self.right_panel_mode = RightPanelMode::EasterEgg {
                content: easter_egg_output,
            };
            
            // Checa achievement de easter egg
            if let Some(achievement) = AchievementChecker::check_easter_egg(&mut self.game_state, &command, &self.i18n) {
                self.award_achievement(achievement, false);
            }

            self.clear_input();
            return Ok(());
        }
        
        // Comandos especiais do Munux
        if command == "stats" {
            self.right_panel_mode = RightPanelMode::Stats;
            self.last_output = "✓ Mostrando estatísticas".to_string();
            self.input_buffer.clear();
            return Ok(());
        } else if command == "quests" || command == "missions" {
            self.right_panel_mode = RightPanelMode::Quests;
            self.last_output = "✓ Mostrando missões ativas".to_string();
            self.input_buffer.clear();
            return Ok(());
        } else if command == "achievements" {
            self.last_output = format!(
                "{}: {}/100\n\n{}:\n{}",
                self.i18n.tc("ui-achievements"),
                self.game_state.achievements.len(),
                self.i18n.tc("ui-last-unlocked"),
                self.game_state.achievements
                    .iter()
                    .rev()
                    .take(5)
                    .map(|a| format!("  • {} - {}", a.name, a.description))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
            self.input_buffer.clear();
            return Ok(());
        } else if command == "tip" {
            self.show_popup(
                "💡 Dica do Dia".to_string(),
                "Use o comando 'help' para listar todos os comandos disponíveis.\n\nExperimente 'stats' para ver seu progresso!".to_string(),
                PopupType::Tip,
            );
            self.last_output = "Mostrando dica...".to_string();
            self.input_buffer.clear();
            return Ok(());
        } else if command.starts_with("help") {
            let args: Vec<&str> = command.split_whitespace().collect();
            if args.len() > 1 {
                let topic = args[1].to_lowercase();
                
                if let Some((cmd, desc, examples, tip)) = self.get_command_help(&topic) {
                     self.right_panel_mode = RightPanelMode::CommandHelp {
                        command: cmd,
                        description: desc,
                        examples,
                        tip,
                    };
                    self.last_output = format!("📚 Ajuda do comando: {}", topic);
                } else {
                    let content = crate::game::distro_guide::DistroGuide::get_guide(&topic);
                    let title = match topic.as_str() {
                        "arch" => "Guia Manjaro/Arch Linux",
                        "debian" => "Guia Ubuntu/Debian",
                        "fedora" => "Guia Fedora/RHEL",
                        "opensuse" => "Guia openSUSE",
                        "linux" => "Guia Linux Universal",
                        _ => "Guia de Ajuda",
                    };
                self.right_panel_mode = RightPanelMode::Help {
                    content,
                    title: title.to_string(),
                };
                self.last_output = format!("📚 Mostrando: {} (Pressione ESC para voltar)", title);
                }
            } else {
                self.right_panel_mode = RightPanelMode::Help {
                    content: r#"📚 MUNUX HELP SYSTEM

Use: help <distro>

Distribuições suportadas:
  help arch     - Manjaro, Arch Linux (pacman, yay, paru)
  help debian   - Ubuntu, Debian, Mint (apt, dpkg, snap)
  help fedora   - Fedora, RHEL, CentOS (dnf, rpm)
  help opensuse - openSUSE (zypper)
  help linux    - Comandos universais Linux

Exemplos:
  help arch     → Mostra comandos pacman, yay
  help debian   → Mostra comandos apt, dpkg
  help linux    → Mostra comandos básicos

Comandos especiais Munux:
  stats         → Estatísticas e progresso
  quests        → Missões ativas
  achievements  → Conquistas desbloqueadas
  xp            → XP e nível atual
  tutorial      → Tutorial interativo para iniciantes
  benchmark     → Teste de velocidade de digitação
  alias n='cmd' → Cria um atalho de comando (unalias n remove)

Pressione ESC para voltar ao modo normal.
"#.to_string(),
                    title: "Sistema de Ajuda Munux".to_string(),
                };
                self.last_output = "📚 Mostrando ajuda (Pressione ESC para voltar)".to_string();
            }
            self.input_buffer.clear();
            return Ok(());
        }
        
        // Comandos internos (não executam via shell)
        if command.starts_with("cd ") {
            let path = command.trim_start_matches("cd ").trim();
            match self.change_directory(path) {
                Ok(_) => {
                    self.last_output = format!("✓ Diretório alterado para: {}", 
                        self.current_dir.display());
                    self.game_state.record_success();
                    // Atualiza o painel para mostrar novo diretório
                    self.right_panel_mode = RightPanelMode::FileTree { 
                        path: self.current_dir.clone() 
                    };
                }
                Err(e) => {
                    self.last_output = format!("✗ Erro: {}", e);
                    self.game_state.record_failure();
                }
            }
        } else if command.starts_with("xp ") {
            // Comando secreto para testar progressão de nível
            if let Ok(amount) = command.trim_start_matches("xp ").trim().parse::<u32>() {
                let _old_level = self.game_state.level;
                let leveled_up = self.game_state.add_xp(amount);
                if leveled_up {
                    self.last_output = format!(
                        "{}: {} | {}: {} | XP: {} | {}",
                        self.i18n.tc("ui-level"),
                        self.game_state.level,
                        self.i18n.tc("ui-rank"),
                        self.game_state.get_rank(&self.i18n),
                        self.game_state.xp,
                        self.i18n.level_message(self.game_state.level)
                    );
                    self.game_state.refresh_quests(&self.i18n);
                } else {
                    let mut args = FluentArgs::new();
                    args.set("amount", amount);
                    args.set("current", self.game_state.xp);
                    args.set("total", self.game_state.xp_to_next_level);
                    args.set("next", self.game_state.level + 1);
                    self.last_output = self.i18n.t("sys-xp-gain", Some(&args));
                }
            } else {
                self.last_output = self.i18n.tc("sys-xp-usage");
            }
        } else if command == "exit" || command == "quit" {
            self.should_quit = true;
        } else if command == "clear" || command == "cls" {
            self.last_output.clear();
        } else if command.starts_with("ls") {
            // ls não mostra output no painel esquerdo, só atualiza o direito
            self.last_output = "📂 Arquivos listados no painel direito →".to_string();
            self.right_panel_mode = RightPanelMode::FileTree { 
                path: self.current_dir.clone() 
            };
            
            // Sistema de XP dinâmico
            let xp_reward = logic::calculate_xp_reward(&command, &cmd_type, true);
            self.game_state.add_xp(xp_reward);
            self.game_state.increment_commands();
            self.game_state.record_success();
            
            // Verifica conquistas (Removido daqui pois já é checado no final do execute_command via AchievementChecker)
        } else {
            // Executa comandos externos via shell
            use crate::core::shell::ShellExecutor;
            
            // Injeta flags de cor para comandos comuns se não estiverem presentes
            let color_command = Self::prepare_color_command(&command);
            
            match ShellExecutor::execute(&color_command, &self.current_dir) {
                Ok(output) => {
                    // Adiciona dicas educativas para erros comuns
                    let helpful_output = if !output.success {
                        Self::add_educational_hints(&command, &output.combined_output())
                    } else {
                        output.combined_output()
                    };
                    
                    self.last_output = if output.success {
                        "✓ Comando executado com sucesso".to_string()
                    } else {
                        "✗ Erro na execução do comando".to_string()
                    };
                    
                    // Define o conteúdo do painel direito com o output do comando com um prompt estilizado
                    let display_output = {
                        use crate::ui::theme::Theme;
                        let symbol = Theme::get_prompt_symbol(self.game_state.level);
                        let rank = self.game_state.get_rank(&self.i18n);
                        // Prompt verde neon com o comando em ciano brilhante (ANSI)
                        let prompt = format!("\x1b[1;32m{} [{}@munux]$ \x1b[0m\x1b[1;36m{}\x1b[0m", symbol, rank, command);
                        format!("{}\n{}", prompt, Self::sanitize_output(&helpful_output))
                    };
                    
                    self.right_panel_mode = RightPanelMode::CommandOutput(display_output);
                    self.scroll = 0; // Reset scroll
                    
                    // Sistema de XP dinâmico baseado no comando
                    let xp_reward = logic::calculate_xp_reward(&command, &cmd_type, output.success);
                    if output.success {
                        let old_level = self.game_state.level;
                        let leveled_up = self.game_state.add_xp(xp_reward);
                        self.game_state.increment_commands();
                        self.game_state.record_success();
                        self.game_state.restore_integrity(5); // Recupera integridade em comandos bem-sucedidos
                        
                        // Notificação de Level Up
                        if leveled_up {
                            self.show_level_up_popup(old_level, self.game_state.level);
                            self.game_state.refresh_quests(&self.i18n);
                            
                            // Muda para painel de gamificação
                            self.right_panel_mode = RightPanelMode::Gamification {
                                message: {
                                    let mut args = fluent::FluentArgs::new();
                                    args.set("level", fluent::FluentValue::from(self.game_state.level));
                                    self.i18n.t("sys-level-up-msg", Some(&args))
                                },
                                celebration: true,
                            };
                        }
                        
                        // Verifica conquistas (Removido daqui pois já é checado no final do execute_command via AchievementChecker)
                    } else {
                        self.game_state.increment_commands();
                        self.game_state.record_failure();
                        self.game_state.damage_integrity(3); // Perde integridade em erros
                    }
                    
                    // Atualiza visualização da árvore de arquivos após comandos que podem modificar
                    if command.starts_with("mkdir") || command.starts_with("touch") || 
                       command.starts_with("rm ") || command.starts_with("mv ") || 
                       command.starts_with("cp ") || command.starts_with("ls") {
                        self.right_panel_mode = RightPanelMode::FileTree { 
                            path: self.current_dir.clone() 
                        };
                    }
                }
                Err(e) => {
                    self.last_output = format!("✗ Erro ao executar comando: {}", e);
                    self.game_state.increment_commands();
                    self.game_state.record_failure();
                }
            }
        }
        
        // Verifica achievements (comando, streak e nível)
        let success = !self.last_output.starts_with("✗");
        if let Some(achievement) =
            AchievementChecker::check_command(&mut self.game_state, &command, success, &self.i18n)
        {
            self.award_achievement(achievement, true);
        }
        if let Some(achievement) = AchievementChecker::check_streak(&mut self.game_state, &self.i18n) {
            self.award_achievement(achievement, false);
        }
        if let Some(achievement) = AchievementChecker::check_level(&mut self.game_state, &self.i18n) {
            self.award_achievement(achievement, false);
        }
        
        // Atualiza progresso das quests
        let current_level = self.game_state.level;
        let mut completed_quests = Vec::new();
        
        for quest in &mut self.game_state.active_quests {
            if quest.update_progress(&command, current_level) {
                completed_quests.push((quest.title.clone(), quest.xp_reward));
            }
        }
        
        // Adiciona XP e mensagens das quests completadas
        for (title, xp) in completed_quests {
            let quest_msg = format!(
                "\n📋 MISSÃO COMPLETA!\n{}\n+{} XP",
                title,
                xp
            );
            self.last_output = format!("{}{}", self.last_output, quest_msg);
            self.game_state.add_xp(xp);
        }

        self.clear_input();
        Ok(())
    }
    
    /// Muda o diretório atual
    fn change_directory(&mut self, path: &str) -> Result<()> {
        let new_path = if path.starts_with('/') {
            PathBuf::from(path)
        } else {
            self.current_dir.join(path)
        };
        
        if new_path.exists() && new_path.is_dir() {
            // Tenta canonicalizar o caminho para resolver .. e .
            if let Ok(canon_path) = std::fs::canonicalize(&new_path) {
                self.current_dir = canon_path.clone();
                self.right_panel_mode = RightPanelMode::FileTree { path: canon_path };
            } else {
                // Fallback se falhar
                self.current_dir = new_path.clone();
                self.right_panel_mode = RightPanelMode::FileTree { path: new_path };
            }
            Ok(())
        } else {
            anyhow::bail!("Diretório não encontrado: {}", path)
        }
    }
    
    /// Analisa o input em tempo real e atualiza o modo do painel direito
    fn analyze_input(&mut self) {
        use crate::core::parser::{CommandParser, CommandType};
        
        let input = self.input_buffer.trim();
        
        if self.ssh_session.is_some() {
            // Em modo SSH, desabilita a análise reativa de arquivos locais
            return;
        }

        if input.is_empty() {
            // Se o input estiver vazio, só volta para Welcome se NÂO estiver mostrando output de comando
            if !matches!(self.right_panel_mode, RightPanelMode::CommandOutput(_)) {
                self.right_panel_mode = RightPanelMode::Welcome;
            }
            self.danger_mode_active = false;
            return;
        }
        
        // Classifica o tipo de comando
        let cmd_type = CommandParser::classify_command(input);
        
        // Comandos especiais do Munux mantém o painel atual ou mudam conforme necessário
        match cmd_type {
            CommandType::MunuxSpecial => {
                // Não muda o painel aqui - será mudado no execute_command
                return;
            }
            CommandType::EasterEgg => {
                // Easter eggs serão processados no execute_command
                return;
            }
            _ => {}
        }
        
        // Usa o parser para determinar o modo
        let mode = CommandParser::command_to_panel_mode(input, &self.current_dir);
        
        // Se o parser retornar FileTree mas não for comando de listagem, volta para Welcome
        let is_listing = ["ls", "ll", "la"].iter().any(|c| input.starts_with(c));
        if matches!(mode, RightPanelMode::FileTree { .. }) && !is_listing {
            self.right_panel_mode = RightPanelMode::Welcome;
            self.danger_mode_active = false;
            return;
        }
        
        self.right_panel_mode = mode;
        
        // Atualiza o flag de perigo
        self.danger_mode_active = matches!(
            self.right_panel_mode,
            RightPanelMode::DangerZone { .. }
        );
        
        // Se estiver em modo monitor, atualiza as métricas
        if matches!(self.right_panel_mode, RightPanelMode::ResourceMonitor { .. }) {
            use crate::core::monitor::SystemMonitor;
            let mut monitor = SystemMonitor::new();
            let summary = monitor.get_system_summary();
            
            self.right_panel_mode = RightPanelMode::ResourceMonitor {
                cpu_usage: summary.cpu_usage,
                memory_used: summary.memory_used,
                memory_total: summary.memory_total,
                process_count: summary.process_count,
            };
        }
        
        // Se for preview de arquivo, tenta carregar o conteúdo
        if let RightPanelMode::FilePreview { ref path, .. } = self.right_panel_mode {
            use crate::core::filesystem::FileSystemManager;
            if let Ok(content) = FileSystemManager::read_file_preview(path) {
                let language = crate::core::parser::CommandParser::detect_language(
                    path.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("")
                );
                self.right_panel_mode = RightPanelMode::FilePreview {
                    path: path.clone(),
                    content,
                    language,
                };
            }
        }
    }
    
    /// Mostra um popup (Ghost Mentor)
    pub fn show_popup(&mut self, title: String, content: String, popup_type: PopupType) {
        self.active_popup = Some(PopupMessage {
            title,
            content,
            popup_type,
        });
    }
    
    /// Fecha o popup ativo
    pub fn close_popup(&mut self) {
        self.active_popup = None;
    }
    
    /// Mostra popup de Level Up
    fn show_level_up_popup(&mut self, old_level: u32, new_level: u32) {
        let rank = self.game_state.get_rank(&self.i18n);
        let message = self.i18n.level_message(new_level);
        self.show_popup(
            "🎉 LEVEL UP!".to_string(),
            format!(
                "Nível {} → {}\n\n{}\n\n{}",
                old_level, new_level, rank, message
            ),
            PopupType::Success,
        );
    }
    
    /// Mostra popup de conquista desbloqueada
    fn show_achievement_popup(&mut self, name: &str, description: &str) {
        self.show_popup(
            "🏆 Conquista Desbloqueada!".to_string(),
            format!("{}\n\n{}", name, description),
            PopupType::Success,
        );
    }

    /// Aplica os efeitos de desbloquear uma conquista: marca como última,
    /// concede o XP, abre o popup e — se `announce` — anexa a notificação ao
    /// output do terminal.
    fn award_achievement(&mut self, achievement: crate::game::state::Achievement, announce: bool) {
        self.game_state.last_achievement = Some(achievement.clone());
        self.game_state.add_xp(achievement.xp_reward);
        self.show_achievement_popup(&achievement.name, &achievement.description);
        if announce {
            self.last_output = format!(
                "{}\n\n🏆 CONQUISTA DESBLOQUEADA!\n\n{}\n{}\n\n+{} XP",
                self.last_output, achievement.name, achievement.description, achievement.xp_reward
            );
        }
    }
    
    /// Adiciona dicas educativas baseadas em erros comuns
    fn add_educational_hints(command: &str, error_output: &str) -> String {
        let mut output = error_output.to_string();
        
        // Detecta erros comuns e adiciona dicas
        if command.starts_with("rm ") && error_output.contains("diretório") {
            output.push_str("\n\n💡 DICA: 'rm' remove ARQUIVOS.");
            output.push_str("\n   Para remover diretórios use:");
            output.push_str("\n   - 'rmdir nome'     (diretório vazio)");
            output.push_str("\n   - 'rm -r nome'     (diretório com conteúdo)");
            output.push_str("\n   - 'rm -rf nome'    (força remoção - CUIDADO!)");
        } else if command.starts_with("rmdir ") && error_output.contains("não vazio") {
            output.push_str("\n\n💡 DICA: 'rmdir' só remove diretórios VAZIOS.");
            output.push_str("\n   Para remover com conteúdo use: 'rm -r nome'");
        } else if command.starts_with("cat ") && error_output.contains("diretório") {
            output.push_str("\n\n💡 DICA: 'cat' mostra conteúdo de ARQUIVOS.");
            output.push_str("\n   Para listar diretórios use: 'ls nome'");
        } else if command.starts_with("cd ") && error_output.contains("Não é um diretório") {
            output.push_str("\n\n💡 DICA: 'cd' navega para DIRETÓRIOS.");
            output.push_str("\n   Para abrir arquivos use: 'cat nome' ou 'nano nome'");
        } else if command.starts_with("mkdir ") && command.contains(".") {
            output.push_str("\n\n💡 DICA: 'mkdir' cria DIRETÓRIOS (pastas).");
            output.push_str("\n   Para criar arquivos use:");
            output.push_str("\n   - 'touch arquivo.txt'          (arquivo vazio)");
            output.push_str("\n   - 'echo \"texto\" > arquivo.txt'  (arquivo com conteúdo)");
        } else if error_output.contains("Permissão negada") {
            output.push_str("\n\n💡 DICA: Você não tem permissão.");
            output.push_str("\n   Tente com 'sudo' antes do comando (cuidado!)");
        } else if error_output.contains("comando não encontrado") || error_output.contains("command not found") {
            output.push_str("\n\n💡 DICA: Comando não existe ou não está instalado.");
            output.push_str("\n   - Verifique se digitou corretamente");
            output.push_str("\n   - Use 'which comando' para verificar se existe");
        }
        
        output
    }
    /// Retorna dados estruturados de ajuda para um comando
    fn get_command_help(&self, command: &str) -> Option<(String, String, Vec<String>, String)> {
        match command {
            "ls" => Some((
                "ls".to_string(),
                self.i18n.tc("help-ls-desc"),
                vec!["ls".to_string(), "ls -la".to_string(), "ls /home".to_string()],
                self.i18n.tc("help-ls-hint")
            )),
            "cd" => Some((
                "cd".to_string(),
                self.i18n.tc("help-cd-desc"),
                vec!["cd Documentos".to_string(), "cd ..".to_string(), "cd ~".to_string()],
                self.i18n.tc("help-cd-hint")
            )),
             "grep" => Some((
                "grep".to_string(),
                self.i18n.tc("help-grep-desc"),
                vec!["grep 'texto' arquivo.txt".to_string(), "cat arquivo | grep 'erro'".to_string()],
                self.i18n.tc("help-grep-hint")
            )),
            "cat" => Some((
                "cat".to_string(),
                self.i18n.tc("help-cat-desc"),
                vec!["cat arquivo.txt".to_string()],
                self.i18n.tc("help-cat-hint")
            )),
            "sudo" => Some((
                "sudo".to_string(),
                self.i18n.tc("help-sudo-desc"),
                vec!["sudo pacman -Syu".to_string(), "sudo reboot".to_string()],
                self.i18n.tc("help-sudo-hint")
            )),
            _ => None
        }
    }
    
    /// Limpa e sanitiza o output para evitar quebras no TUI
    fn sanitize_output(content: &str) -> String {
        content.replace('\t', "    ") // Expande tabs para espaços para evitar saltos de cursor bugados
               .replace('\r', "")      // Remove carriage returns que podem causar sobreposição de texto
    }

    /// Injeta flags de cor para comandos comuns em ambientes sem TTY
    fn prepare_color_command(command: &str) -> String {
        let trimmed = command.trim();
        if trimmed.is_empty() { return command.to_string(); }
        
        let parts: Vec<&str> = trimmed.split_whitespace().collect();
        let cmd = parts[0];
        
        match cmd {
            "git" => {
                // Força cores no git: git -c color.ui=always <resto>
                let mut new_parts = vec!["git", "-c", "color.ui=always"];
                new_parts.extend_from_slice(&parts[1..]);
                new_parts.join(" ")
            },
            "ls" => {
                // Força cores no ls (Linux)
                if !trimmed.contains("--color") {
                    format!("ls --color=always {}", &trimmed[2..].trim())
                } else {
                    trimmed.to_string()
                }
            },
            "grep" => {
                // Força cores no grep
                if !trimmed.contains("--color") {
                    format!("grep --color=always {}", &trimmed[4..].trim())
                } else {
                    trimmed.to_string()
                }
            },
            "pacman" | "yay" => {
                // Força cores no gerenciador de pacotes
                if !trimmed.contains("--color") {
                    format!("{} --color=always {}", cmd, &trimmed[cmd.len()..].trim())
                } else {
                    trimmed.to_owned()
                }
            },
            "tree" => {
                // Força cores no tree
                if !trimmed.contains("-C") {
                    format!("tree -C {}", &trimmed[4..].trim())
                } else {
                    trimmed.to_owned()
                }
            },
            "ip" => {
                // ip route, ip addr, etc
                if !trimmed.contains("-c") && !trimmed.contains("-color") {
                    format!("ip -c {}", &trimmed[2..].trim())
                } else {
                    trimmed.to_string()
                }
            },
            _ => command.to_string()
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new().expect("Falha ao criar aplicação")
    }
}
