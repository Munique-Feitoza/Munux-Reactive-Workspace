// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

use crate::game::state::GameState;
use anyhow::Result;
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
    /// Cria uma nova instância do App com valores padrão
    pub fn new() -> Result<Self> {
        let current_dir = std::env::current_dir()?;
        
        Ok(Self {
            input_buffer: String::new(),
            command_history: Vec::new(),
            history_index: None,
            right_panel_mode: RightPanelMode::Welcome,
            game_state: GameState::new(),
            current_dir,
            last_output: String::new(),
            should_quit: false,
            danger_mode_active: false,
            active_popup: None,
        })
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
        
        if self.input_buffer.trim().is_empty() {
            return Ok(());
        }
        
        // Adiciona ao histórico
        self.command_history.push(self.input_buffer.clone());
        self.history_index = None;
        
        let command = self.input_buffer.clone().trim().to_string();
        
        // Verifica easter eggs primeiro
        if let Some(easter_egg_output) = EasterEggs::check(&command) {
            self.last_output = easter_egg_output.clone();
            self.right_panel_mode = RightPanelMode::EasterEgg {
                content: easter_egg_output,
            };
            
            // Checa achievement de easter egg
            if let Some(achievement) = AchievementChecker::check_easter_egg(&mut self.game_state, &command) {
                self.game_state.last_achievement = Some(achievement.clone());
                self.game_state.add_xp(achievement.xp_reward);
            }
            
            self.input_buffer.clear();
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
                "🏆 Conquistas: {}/100\n\nÚltimas desbloqueadas:\n{}",
                self.game_state.achievements.len(),
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
        } else if command.starts_with("help") {
            let args: Vec<&str> = command.split_whitespace().collect();
            if args.len() > 1 {
                let distro = args[1].to_lowercase();
                let content = crate::game::distro_guide::DistroGuide::get_guide(&distro);
                let title = match distro.as_str() {
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
                let old_level = self.game_state.level;
                let leveled_up = self.game_state.add_xp(amount);
                if leveled_up {
                    self.last_output = format!(
                        "✓ LEVEL UP! {} → {} | Você é agora: {} | {}",
                        old_level,
                        self.game_state.level,
                        self.game_state.get_rank(),
                        crate::ui::theme::Theme::get_level_message(self.game_state.level)
                    );
                    self.game_state.refresh_quests();
                } else {
                    self.last_output = format!("✓ +{} XP | {}/{} até o nível {}", 
                        amount,
                        self.game_state.xp,
                        self.game_state.xp_to_next_level,
                        self.game_state.level + 1
                    );
                }
            } else {
                self.last_output = "✗ Uso: xp <quantidade>".to_string();
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
            self.game_state.add_xp(5);
            self.game_state.increment_commands();
        } else {
            // Executa comandos externos via shell
            use crate::core::shell::ShellExecutor;
            
            match ShellExecutor::execute(&command, &self.current_dir) {
                Ok(output) => {
                    // Adiciona dicas educativas para erros comuns
                    let helpful_output = if !output.success {
                        Self::add_educational_hints(&command, &output.combined_output())
                    } else {
                        output.combined_output()
                    };
                    
                    self.last_output = if output.success {
                        // Para comandos de listagem, formata melhor a saída
                        if command.starts_with("ls") {
                            let formatted = output.combined_output()
                                .split_whitespace()
                                .collect::<Vec<_>>()
                                .join("\n  ");
                            format!("✓ Arquivos e diretórios:\n\n  {}", formatted)
                        } else {
                            format!("✓ Comando executado:\n{}", helpful_output)
                        }
                    } else {
                        format!("✗ Erro na execução:\n{}", helpful_output)
                    };
                    
                    // Atualiza XP se o comando foi bem-sucedido
                    if output.success {
                        self.game_state.add_xp(10);
                        self.game_state.increment_commands();
                        self.game_state.record_success();
                    } else {
                        self.game_state.increment_commands();
                        self.game_state.record_failure();
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
        
        // Verifica achievements
        if let Some(achievement) = AchievementChecker::check_command(
            &mut self.game_state,
            &command,
            !self.last_output.starts_with("✗"),
        ) {
            self.game_state.last_achievement = Some(achievement.clone());
            self.game_state.add_xp(achievement.xp_reward);
            
            // Mostra notificação de achievement
            let achievement_msg = format!(
                "🏆 CONQUISTA DESBLOQUEADA!\n\n{}\n{}\n\n+{} XP",
                achievement.name,
                achievement.description,
                achievement.xp_reward
            );
            self.last_output = format!("{}\n\n{}", self.last_output, achievement_msg);
        }
        
        // Verifica achievement de streak
        if let Some(streak_achievement) = AchievementChecker::check_streak(&mut self.game_state) {
            self.game_state.last_achievement = Some(streak_achievement.clone());
            self.game_state.add_xp(streak_achievement.xp_reward);
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
            self.current_dir = new_path.clone();
            self.right_panel_mode = RightPanelMode::FileTree { path: new_path };
            Ok(())
        } else {
            anyhow::bail!("Diretório não encontrado: {}", path)
        }
    }
    
    /// Analisa o input em tempo real e atualiza o modo do painel direito
    fn analyze_input(&mut self) {
        use crate::core::parser::{CommandParser, CommandType};
        
        let input = self.input_buffer.trim();
        
        if input.is_empty() {
            self.right_panel_mode = RightPanelMode::Welcome;
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
        if matches!(mode, RightPanelMode::FileTree { .. }) {
            if !input.starts_with("ls") && !input.starts_with("ll") && !input.starts_with("la") {
                self.right_panel_mode = RightPanelMode::Welcome;
                self.danger_mode_active = false;
                return;
            }
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
}

impl Default for App {
    fn default() -> Self {
        Self::new().expect("Falha ao criar aplicação")
    }
}
