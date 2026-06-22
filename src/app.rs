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

impl RightPanelMode {
    /// Constrói o painel de recursos a partir de um resumo do sistema.
    pub fn resource_from(s: &crate::core::monitor::SystemSummary) -> Self {
        RightPanelMode::ResourceMonitor {
            cpu_usage: s.cpu_usage,
            memory_used: s.memory_used,
            memory_total: s.memory_total,
            process_count: s.process_count,
        }
    }
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

    /// Comando perigoso aguardando confirmação explícita (`sim`). `None` = nenhum.
    pub pending_command: Option<String>,

    /// Monitor de recursos persistente — uma única instância (refresh incremental).
    /// Evita recriar `System::new_all()` a cada tecla/tick e permite ao `sysinfo`
    /// calcular o delta de CPU corretamente entre refreshes.
    pub monitor: crate::core::monitor::SystemMonitor,

    /// Último resumo do sistema (consumido pelos painéis de monitor e de stats).
    pub system_summary: crate::core::monitor::SystemSummary,

    /// Índice selecionado no navegador de arquivos (painel de árvore + Enter/setas).
    pub file_selection: usize,

    /// A partir de qual índice do histórico o "scrollback" é exibido. `clear`/
    /// `Ctrl+L` apontam isto para o fim, limpando a tela **sem** perder o
    /// histórico navegável pelas setas.
    pub history_view_start: usize,
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

/// Palavras que cancelam um modo ativo (tutorial, benchmark). Fonte única.
const CANCEL_WORDS: &[&str] = &["sair", "exit", "stop", "parar", "cancel", "cancelar"];

/// Máximo de entradas exibidas/navegáveis no painel de árvore de arquivos.
pub const MAX_TREE_ENTRIES: usize = 20;

/// Verifica se `word` é uma palavra de cancelamento (case-insensitive).
fn is_cancel_word(word: &str) -> bool {
    let w = word.trim().to_lowercase();
    CANCEL_WORDS.contains(&w.as_str())
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
            pending_command: None,
            monitor: crate::core::monitor::SystemMonitor::new(),
            system_summary: crate::core::monitor::SystemSummary::default(),
            file_selection: 0,
            history_view_start: 0,
        })
    }

    /// Limpa a tela (saída + scrollback visível) e volta ao painel inicial.
    /// Conceito único de "limpar" compartilhado por `clear`/`cls` e `Ctrl+L`.
    /// O histórico navegável (setas) é preservado.
    pub fn clear_screen(&mut self) {
        self.last_output.clear();
        self.history_view_start = self.command_history.len();
        self.right_panel_mode = RightPanelMode::Welcome;
        self.clear_input();
    }

    /// Entradas do diretório atual (mesma lista do painel de árvore, limitada a
    /// [`MAX_TREE_ENTRIES`]). Fonte única para render e navegação.
    pub fn dir_entries(&self) -> Vec<crate::core::filesystem::FileEntry> {
        let mut entries =
            crate::core::filesystem::FileSystemManager::list_directory(&self.current_dir)
                .unwrap_or_default();
        entries.truncate(MAX_TREE_ENTRIES);
        entries
    }

    /// `true` quando o navegador de arquivos está ativo: árvore visível e sem
    /// nada digitado (aí as setas navegam arquivos em vez do histórico).
    pub fn is_browsing_files(&self) -> bool {
        self.input_buffer.is_empty()
            && matches!(self.right_panel_mode, RightPanelMode::FileTree { .. })
    }

    /// Move a seleção do navegador (`delta` negativo = sobe).
    pub fn move_file_selection(&mut self, delta: i32) {
        let len = self.dir_entries().len();
        if len == 0 {
            self.file_selection = 0;
            return;
        }
        self.file_selection = if delta < 0 {
            self.file_selection.saturating_sub(1)
        } else {
            (self.file_selection + 1).min(len - 1)
        };
    }

    /// Abre a entrada selecionada: entra no diretório ou mostra o preview do
    /// arquivo. Usa o campo `FileEntry.path`.
    pub fn open_selected_entry(&mut self) {
        let entries = self.dir_entries();
        let Some(entry) = entries.get(self.file_selection) else { return };
        let path = entry.path.clone();

        if entry.is_dir {
            if let Some(p) = path.to_str() {
                let _ = self.change_directory(p);
            }
            self.file_selection = 0;
        } else {
            let language = crate::core::parser::CommandParser::detect_language(&entry.name);
            let content = crate::core::filesystem::FileSystemManager::read_file_preview(&path)
                .unwrap_or_default();
            self.right_panel_mode = RightPanelMode::FilePreview { path, content, language };
            self.scroll = 0;
        }
    }

    /// Atualiza o monitor de sistema persistente e guarda o resumo. Se o painel
    /// de recursos estiver ativo, reflete os novos valores nele.
    pub fn refresh_monitor(&mut self) {
        self.system_summary = self.monitor.get_system_summary();
        if matches!(self.right_panel_mode, RightPanelMode::ResourceMonitor { .. }) {
            self.right_panel_mode = RightPanelMode::resource_from(&self.system_summary);
        }
    }

    /// Atalho para traduzir uma chave com um único argumento string.
    /// (Casos multi-argumento montam `FluentArgs` inline.)
    fn t1(&self, key: &str, name: &'static str, value: impl Into<String>) -> String {
        let mut args = FluentArgs::new();
        args.set(name, value.into());
        self.i18n.t(key, Some(&args))
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
                self.last_output = self.i18n.tc("sys-alias-none");
            } else {
                let mut lines: Vec<String> =
                    self.aliases.iter().map(|(k, v)| format!("  {} = {}", k, v)).collect();
                lines.sort();
                self.last_output =
                    format!("{}\n{}", self.i18n.tc("sys-alias-list-title"), lines.join("\n"));
            }
            self.clear_input();
            return true;
        }

        // `unalias nome`: remove.
        if let Some(name) = trimmed.strip_prefix("unalias ") {
            let name = name.trim();
            self.last_output = if self.aliases.remove(name).is_some() {
                self.t1("sys-alias-removed", "name", name)
            } else {
                self.t1("sys-alias-missing", "name", name)
            };
            self.clear_input();
            return true;
        }

        // `alias nome=comando` ou `alias nome='comando com args'`.
        if let Some(rest) = trimmed.strip_prefix("alias ") {
            if let Some((name, value)) = rest.split_once('=') {
                let name = name.trim();
                let value = value.trim().trim_matches(|c| c == '\'' || c == '"').trim();
                if name.is_empty() || value.is_empty() {
                    self.last_output = self.i18n.tc("sys-alias-usage");
                } else if name.contains(char::is_whitespace) {
                    self.last_output = self.i18n.tc("sys-alias-no-spaces");
                } else {
                    self.aliases.insert(name.to_string(), value.to_string());
                    let mut args = FluentArgs::new();
                    args.set("name", name);
                    args.set("value", value);
                    self.last_output = self.i18n.t("sys-alias-created", Some(&args));
                }
            } else {
                self.last_output = self.i18n.tc("sys-alias-usage");
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
        if is_cancel_word(arg) {
            if self.tutorial.is_some() {
                self.tutorial = None;
                self.last_output = self.i18n.tc("sys-tutorial-ended");
            } else {
                self.last_output = self.i18n.tc("sys-tutorial-none");
            }
        } else {
            self.tutorial = Some(0);
            self.last_output = self.i18n.tc("sys-tutorial-started");
            self.show_popup(
                self.i18n.tc("sys-tutorial-mode-title"),
                crate::game::tutorial::step_text(0),
                PopupType::Tip,
            );
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
                self.i18n.tc("sys-tutorial-step-done-title"),
                tutorial::step_text(next),
                PopupType::Success,
            );
        } else {
            self.tutorial = None;
            self.game_state.add_xp(tutorial::COMPLETION_XP);
            let mut args = FluentArgs::new();
            args.set("xp", tutorial::COMPLETION_XP);
            self.show_popup(
                self.i18n.tc("sys-tutorial-complete-title"),
                self.i18n.t("sys-tutorial-complete-body", Some(&args)),
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

        if is_cancel_word(trimmed.split_whitespace().nth(1).unwrap_or("")) {
            self.last_output = self.i18n.tc("sys-bench-none");
        } else {
            let state = crate::game::benchmark::BenchmarkState::start();
            self.last_output = self.t1("sys-bench-start", "phrase", state.prompt.clone());
            self.show_popup(
                self.i18n.tc("sys-bench-popup-title"),
                self.t1("sys-bench-popup-body", "phrase", state.prompt.clone()),
                PopupType::Info,
            );
            self.benchmark = Some(state);
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
        let mut args = FluentArgs::new();
        args.set("seconds", format!("{:.1}", result.seconds)); // precisão fixa em Rust
        args.set("wpm", result.wpm);
        args.set("accuracy", result.accuracy);
        args.set("xp", result.xp);
        let summary = self.i18n.t("sys-bench-result", Some(&args));
        self.last_output = summary.clone();
        self.show_popup(self.i18n.tc("sys-bench-result-title"), summary, PopupType::Success);
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

        // Gate de confirmação: se há um comando perigoso pendente, esta entrada é
        // a resposta. Só `sim`/`s`/`yes`/`y` re-injeta o comando para execução;
        // qualquer outra coisa cancela.
        let mut danger_confirmed = false;
        if let Some(pending) = self.pending_command.take() {
            let answer = self.input_buffer.trim().to_lowercase();
            if matches!(answer.as_str(), "sim" | "s" | "yes" | "y") {
                self.input_buffer = pending;
                danger_confirmed = true;
            } else {
                self.clear_input();
                self.last_output = self.i18n.tc("sys-danger-cancelled");
                self.right_panel_mode = RightPanelMode::Welcome;
                return Ok(());
            }
        }

        // Modo benchmark ativo: a entrada é a frase digitada, não um comando.
        if self.benchmark.is_some() {
            let typed = self.input_buffer.clone();
            let mut words = typed.split_whitespace();
            let is_cancel = words.next() == Some("benchmark")
                && words.next().map(is_cancel_word).unwrap_or(false);
            if is_cancel {
                self.benchmark = None;
                self.last_output = self.i18n.tc("sys-bench-cancelled");
            } else {
                self.finish_benchmark(&typed);
            }
            self.clear_input();
            return Ok(());
        }

        // Adiciona ao histórico (o comando confirmado já foi registrado na 1ª tentativa).
        if !danger_confirmed {
            self.command_history.push(self.input_buffer.clone());
        }
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

        // Lógica de Sessão SSH (extraída em handler próprio).
        if self.ssh_session.is_some() {
            self.handle_ssh_session(&command);
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
                    
                    self.last_output = {
                        let mut args = FluentArgs::new();
                        args.set("user", user);
                        args.set("host", host);
                        self.i18n.t("sys-ssh-connecting", Some(&args))
                    };
                    // Renderiza um frame antes de bloquear na conexão (idealmente seria async, mas TUI é sync)
                    // Como não temos async runtime fácil aqui, vai bloquear brevemente

                    match crate::core::ssh::SshSession::connect(user, host) {
                        Ok(session) => {
                            let cwd = session.remote_cwd.clone();
                            self.ssh_session = Some(session);
                            self.last_output = {
                                let mut args = FluentArgs::new();
                                args.set("host", host);
                                args.set("dir", cwd.clone());
                                self.i18n.t("sys-ssh-connected", Some(&args))
                            };
                            self.right_panel_mode = RightPanelMode::Welcome; // Ou algum modo específico SSH

                            let title = self.i18n.tc("sys-ssh-conn-title");
                            let body = {
                                let mut args = FluentArgs::new();
                                args.set("user", user);
                                args.set("host", host);
                                args.set("dir", cwd.clone());
                                self.i18n.t("sys-ssh-conn-body", Some(&args))
                            };
                            self.show_popup(title, body, PopupType::Success);
                        }
                        Err(e) => {
                            self.last_output = {
                                let mut args = FluentArgs::new();
                                args.set("msg", e.to_string());
                                self.i18n.t("sys-ssh-fail", Some(&args))
                            };
                            let title = self.i18n.tc("sys-ssh-fail-title");
                            let body = {
                                let mut args = FluentArgs::new();
                                args.set("target", target);
                                args.set("msg", e.to_string());
                                self.i18n.t("sys-ssh-fail-body", Some(&args))
                            };
                            self.show_popup(title, body, PopupType::Warning);
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
            // Comando perigoso liberado (modo seguro desligado): exige confirmação
            // explícita em vez de executar direto no Enter.
            if !danger_confirmed {
                self.pending_command = Some(command.clone());
                self.clear_input();
                self.last_output = self.i18n.tc("sys-danger-confirm");
                self.right_panel_mode = self.command_to_panel_mode(&command);
                return Ok(());
            }
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
        
        // Comandos especiais do Munux (stats/quests/achievements/tip/help).
        if self.handle_special_command(&command) {
            return Ok(());
        }

        // Comandos internos (não executam via shell)
        if command.starts_with("cd ") {
            let path = Self::parse_cd_arg(&command);
            // Regra de contagem: comandos reais (cd, ls, externos) contam para
            // stats/quests; comandos do app (stats/help/tip/...) não. `cd` antes
            // não incrementava — agora conta, igual ao `ls`.
            match self.change_directory(path) {
                Ok(_) => {
                    self.last_output =
                        self.t1("sys-cd-ok", "dir", self.current_dir.display().to_string());
                    self.game_state.increment_commands();
                    self.game_state.record_success();
                    // Atualiza o painel para mostrar novo diretório
                    self.right_panel_mode = RightPanelMode::FileTree {
                        path: self.current_dir.clone()
                    };
                }
                Err(e) => {
                    self.last_output = self.t1("sys-error", "msg", e.to_string());
                    self.game_state.increment_commands();
                    self.game_state.record_failure();
                }
            }
        } else if cfg!(debug_assertions) && command.starts_with("xp ") {
            // Comando secreto para testar progressão de nível — só em builds de
            // debug. Em release o ramo é eliminado e `xp` cai no shell (sem cheat).
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
            self.clear_screen(); // limpa buffer e tela; histórico navegável preservado
            return Ok(());
        } else if command.starts_with("ls") {
            // ls não mostra output no painel esquerdo, só atualiza o direito
            self.last_output = self.i18n.tc("sys-ls-listed");
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
                        self.add_educational_hints(&command, &output.combined_output())
                    } else {
                        output.combined_output()
                    };
                    
                    self.last_output = if output.success {
                        self.i18n.tc("sys-cmd-ok")
                    } else {
                        self.i18n.tc("sys-cmd-error")
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
                    self.last_output = self.t1("sys-cmd-exec-error", "msg", e.to_string());
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
            let mut args = FluentArgs::new();
            args.set("title", title);
            args.set("xp", xp);
            let quest_msg = self.i18n.t("sys-quest-complete", Some(&args));
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
            anyhow::bail!("{}", self.t1("sys-cd-notfound", "path", path))
        }
    }
    
    /// Trata os comandos especiais do Munux (stats/quests/achievements/tip/help).
    /// Retorna `true` se o comando foi um deles (e já foi processado) — extraído
    /// de `execute_command` para reduzir o God Object.
    fn handle_special_command(&mut self, command: &str) -> bool {
        if command == "stats" {
            self.refresh_monitor(); // snapshot fresco de CPU/RAM ao abrir o painel
            self.right_panel_mode = RightPanelMode::Stats;
            self.last_output = self.i18n.tc("sys-showing-stats");
            self.input_buffer.clear();
            true
        } else if command == "quests" || command == "missions" {
            self.right_panel_mode = RightPanelMode::Quests;
            self.last_output = self.i18n.tc("sys-showing-quests");
            self.input_buffer.clear();
            true
        } else if command == "achievements" {
            self.last_output = format!(
                "{}: {}/100\n\n{}:\n{}",
                self.i18n.tc("ui-achievements"),
                self.game_state.achievements.len(),
                self.i18n.tc("ui-last-unlocked"),
                self.game_state
                    .achievements
                    .iter()
                    .rev()
                    .take(5)
                    .map(|a| format!("  • {} - {}", a.name, a.description))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
            self.input_buffer.clear();
            true
        } else if command == "tip" {
            let title = self.i18n.tc("sys-tip-title");
            let body = self.i18n.tc("sys-tip-body");
            self.show_popup(title, body, PopupType::Tip);
            self.last_output = self.i18n.tc("sys-tip-showing");
            self.input_buffer.clear();
            true
        } else if command.starts_with("help") {
            self.scroll = 0; // começa o guia do topo (PageUp/PageDown rolam a partir daqui)
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
                    self.last_output = self.t1("sys-help-cmd", "topic", topic);
                } else {
                    // Conteúdo e título vêm juntos da fonte única (sempre coerentes).
                    let (content, title) = crate::game::distro_guide::DistroGuide::get(&topic);
                    self.right_panel_mode = RightPanelMode::Help {
                        content,
                        title: title.to_string(),
                    };
                    self.last_output = self.t1("sys-help-showing-title", "title", title);
                }
            } else {
                self.right_panel_mode = RightPanelMode::Help {
                    content: self.i18n.tc("help-system-body"),
                    title: self.i18n.tc("help-system-title"),
                };
                self.last_output = self.i18n.tc("sys-help-showing");
            }
            self.input_buffer.clear();
            true
        } else {
            false
        }
    }

    /// Decide o modo do painel direito a partir do comando digitado.
    ///
    /// Vive na camada `app` (não no `core`): produz `RightPanelMode` (tipo de UI)
    /// e usa `i18n`. O `core::parser` apenas classifica e busca arquivos.
    fn command_to_panel_mode(&self, input: &str) -> RightPanelMode {
        use crate::core::parser::{CommandParser, CommandType};

        let cmd_type = CommandParser::classify_command(input);
        let trimmed = input.trim();
        let current_dir = &self.current_dir;

        match cmd_type {
            CommandType::Dangerous => {
                let key = if trimmed.contains("rm")
                    && (trimmed.contains("-rf") || trimmed.contains("-fr"))
                {
                    if trimmed.contains('/') && (trimmed.contains("/*") || trimmed.ends_with('/')) {
                        "danger-rm-root"
                    } else {
                        "danger-rm-rf"
                    }
                } else if trimmed.contains("rm") {
                    "danger-rm"
                } else if trimmed.starts_with("sudo") {
                    "danger-sudo"
                } else if trimmed.contains("dd") {
                    "danger-dd"
                } else if trimmed.contains("mkfs")
                    || trimmed.contains("fdisk")
                    || trimmed.contains("parted")
                {
                    "danger-fs"
                } else if trimmed.contains("chmod") || trimmed.contains("chown") {
                    "danger-perm"
                } else if trimmed.contains("reboot")
                    || trimmed.contains("shutdown")
                    || trimmed.contains("poweroff")
                {
                    "danger-power"
                } else {
                    "danger-generic"
                };
                RightPanelMode::DangerZone {
                    warning: self.i18n.tc(key),
                    command: trimmed.to_string(),
                }
            }

            CommandType::FileViewing => {
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() >= 2 {
                    let filename = parts[1];
                    let matches = CommandParser::find_matching_files(current_dir, filename);

                    if matches.len() == 1 {
                        RightPanelMode::FilePreview {
                            path: matches[0].clone(),
                            content: String::new(),
                            language: CommandParser::detect_language(filename),
                        }
                    } else if matches.len() > 1 {
                        let suggestions = matches
                            .iter()
                            .filter_map(|p| p.file_name()?.to_str().map(|s| s.to_string()))
                            .collect::<Vec<_>>()
                            .join("\n  → ");
                        RightPanelMode::FilePreview {
                            path: current_dir.join(filename),
                            content: format!(
                                "{}\n\n  → {}",
                                self.i18n.tc("sys-files-found"),
                                suggestions
                            ),
                            language: "text".to_string(),
                        }
                    } else {
                        RightPanelMode::FilePreview {
                            path: current_dir.join(filename),
                            content: self.t1("sys-file-not-found", "name", filename),
                            language: "text".to_string(),
                        }
                    }
                } else {
                    RightPanelMode::FileTree { path: current_dir.clone() }
                }
            }

            CommandType::SystemMonitoring => RightPanelMode::ResourceMonitor {
                cpu_usage: 0.0,
                memory_used: 0,
                memory_total: 0,
                process_count: 0,
            },

            _ => RightPanelMode::FileTree { path: current_dir.clone() },
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
        
        // Determina o modo do painel (camada app, com i18n)
        let mode = self.command_to_panel_mode(input);
        
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
        
        // Se estiver em modo monitor, atualiza as métricas (instância única).
        if matches!(self.right_panel_mode, RightPanelMode::ResourceMonitor { .. }) {
            self.refresh_monitor();
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
        let mut args = FluentArgs::new();
        args.set("old", old_level);
        args.set("new", new_level);
        args.set("rank", rank);
        args.set("msg", message);
        let title = self.i18n.tc("sys-levelup-title");
        let body = self.i18n.t("sys-levelup-body", Some(&args));
        self.show_popup(title, body, PopupType::Success);
    }

    /// Mostra popup de conquista desbloqueada
    fn show_achievement_popup(&mut self, name: &str, description: &str) {
        let title = self.i18n.tc("sys-achievement-title");
        self.show_popup(
            title,
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
            let mut args = FluentArgs::new();
            args.set("name", achievement.name.clone());
            args.set("desc", achievement.description.clone());
            args.set("xp", achievement.xp_reward);
            let announce_msg = self.i18n.t("sys-achievement-announce", Some(&args));
            self.last_output = format!("{}\n\n{}", self.last_output, announce_msg);
        }
    }
    
    /// Anexa uma dica educativa quando a saída de erro casa um padrão conhecido.
    ///
    /// Os *matchers* inspecionam a saída do shell (que segue o locale do SO),
    /// por isso cobrem PT e EN; o **texto** da dica vem dos `.ftl` (locale do app).
    fn add_educational_hints(&self, command: &str, error_output: &str) -> String {
        let mut output = error_output.to_string();
        let lower = error_output.to_lowercase();

        let is_dir = lower.contains("diretório") || lower.contains("is a directory");
        let not_empty = lower.contains("não vazio") || lower.contains("not empty");
        let not_a_dir = lower.contains("não é um diretório") || lower.contains("not a directory");
        let permission = lower.contains("permissão negada") || lower.contains("permission denied");
        let not_found =
            lower.contains("comando não encontrado") || lower.contains("command not found");

        let hint_key = if command.starts_with("rm ") && is_dir {
            Some("hint-err-rm-isdir")
        } else if command.starts_with("rmdir ") && not_empty {
            Some("hint-err-rmdir-notempty")
        } else if command.starts_with("cat ") && is_dir {
            Some("hint-err-cat-isdir")
        } else if command.starts_with("cd ") && not_a_dir {
            Some("hint-err-cd-notdir")
        } else if command.starts_with("mkdir ") && command.contains('.') {
            Some("hint-err-mkdir-dots")
        } else if permission {
            Some("hint-err-permission")
        } else if not_found {
            Some("hint-err-notfound")
        } else {
            None
        };

        if let Some(key) = hint_key {
            output.push_str(&self.i18n.tc(key));
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
    
    /// Extrai o caminho de um comando `cd ...` (sem o prefixo). Fonte única —
    /// antes o parse acontecia em dois lugares (cd local e cd remoto/SSH).
    fn parse_cd_arg(command: &str) -> &str {
        command.trim().strip_prefix("cd ").map(str::trim).unwrap_or("")
    }

    /// Trata comandos dentro de uma sessão SSH ativa (exit/cd/exec remoto).
    /// Extraído de `execute_command` para reduzir o God Object.
    fn handle_ssh_session(&mut self, command: &str) {
        // `exit`/`logout` encerram a sessão (tratado antes do borrow de ssh_session).
        if command == "exit" || command == "logout" {
            self.ssh_session = None;
            self.last_output = self.i18n.tc("sys-ssh-disconnected");
            self.right_panel_mode = RightPanelMode::Welcome;
            self.input_buffer.clear();
            return;
        }

        let Some(ssh_session) = &mut self.ssh_session else { return };

        if command.starts_with("cd ") {
            let path = Self::parse_cd_arg(command);
            match ssh_session.change_dir(path) {
                Ok(_) => {
                    let mut args = FluentArgs::new();
                    args.set("dir", ssh_session.remote_cwd.clone());
                    self.last_output = self.i18n.t("sys-ssh-cd-ok", Some(&args));
                    self.right_panel_mode =
                        RightPanelMode::CommandOutput(self.last_output.clone());
                }
                Err(e) => {
                    let mut args = FluentArgs::new();
                    args.set("msg", e.to_string());
                    self.last_output = self.i18n.t("sys-error", Some(&args));
                }
            }
        } else {
            // Mesma injeção de cores do shell local (git/ls/grep/pacman/yay/tree/ip).
            let cmd_to_run = Self::prepare_color_command(command);
            match ssh_session.execute(&cmd_to_run) {
                Ok((stdout, stderr, _code)) => {
                    let output = if !stdout.is_empty() { stdout } else { stderr };
                    let prompt = format!(
                        "\x1b[1;36m{}@{} \x1b[0m\x1b[1;33m{}\x1b[0m$ \x1b[1m{}\x1b[0m",
                        ssh_session.user, ssh_session.host, ssh_session.remote_cwd, command
                    );
                    self.last_output = Self::sanitize_output(&output);
                    self.right_panel_mode = RightPanelMode::CommandOutput(format!(
                        "{}\n{}",
                        prompt, self.last_output
                    ));
                }
                Err(e) => {
                    let mut args = FluentArgs::new();
                    args.set("msg", e.to_string());
                    self.last_output = self.i18n.t("sys-ssh-exec-error", Some(&args));
                }
            }
        }

        self.input_buffer.clear();
    }

    /// Limpa e sanitiza o output para evitar quebras no TUI
    fn sanitize_output(content: &str) -> String {
        content.replace('\t', "    ") // Expande tabs para espaços para evitar saltos de cursor bugados
               .replace('\r', "")      // Remove carriage returns que podem causar sobreposição de texto
    }

    /// Injeta flags de cor para comandos comuns em ambientes sem TTY
    /// Injeta flags de cor (`--color=always` etc.) em comandos comuns quando
    /// ausentes. Separa base e argumentos com `split_once` (seguro p/ UTF-8, sem
    /// fatiamento por offset). Usada tanto no shell local quanto no SSH.
    fn prepare_color_command(command: &str) -> String {
        let trimmed = command.trim();
        if trimmed.is_empty() {
            return command.to_string();
        }

        let (cmd, args) = match trimmed.split_once(char::is_whitespace) {
            Some((c, a)) => (c, a.trim()),
            None => (trimmed, ""),
        };

        let colorized = match cmd {
            "git" => format!("git -c color.ui=always {}", args),
            "ls" if !trimmed.contains("--color") => format!("ls --color=always {}", args),
            "grep" if !trimmed.contains("--color") => format!("grep --color=always {}", args),
            "pacman" | "yay" if !trimmed.contains("--color") => {
                format!("{} --color=always {}", cmd, args)
            }
            "tree" if !trimmed.contains("-C") => format!("tree -C {}", args),
            "ip" if !trimmed.contains("-c") && !trimmed.contains("-color") => {
                format!("ip -c {}", args)
            }
            _ => return command.to_string(),
        };

        colorized.trim().to_string()
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new().expect("Falha ao criar aplicação")
    }
}
