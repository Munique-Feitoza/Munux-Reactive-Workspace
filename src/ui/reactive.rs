// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

use crate::app::{App, RightPanelMode};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Wrap},
    Frame,
};
use std::fs;

/// Renderiza o painel direito reativo (o "camaleão")
pub fn render_reactive_panel(frame: &mut Frame, app: &App, area: Rect) {
    match &app.right_panel_mode {
        RightPanelMode::Welcome => render_welcome_screen(frame, app, area),
        RightPanelMode::FileTree { path } => {
            // Se tiver input, mostra dica + arquivos
            render_file_tree_with_hint(frame, path, &app.input_buffer, area)
        }
        RightPanelMode::FilePreview { path, content, language } => {
            render_file_preview(frame, path, content, language, area)
        }
        RightPanelMode::ResourceMonitor { cpu_usage, memory_used, memory_total, process_count } => {
            render_resource_monitor(frame, *cpu_usage, *memory_used, *memory_total, *process_count, area)
        }
        RightPanelMode::DangerZone { warning, command } => {
            render_danger_zone(frame, warning, command, area)
        }
        RightPanelMode::Gamification { message, celebration } => {
            render_gamification(frame, message, *celebration, area)
        }
        RightPanelMode::Stats => {
            crate::ui::stats::render_stats_panel(frame, app, area)
        }
        RightPanelMode::Quests => {
            crate::ui::stats::render_quests_panel(frame, app, area)
        }
        RightPanelMode::EasterEgg { content } => {
            render_easter_egg(frame, content, area)
        }
        RightPanelMode::Help { content, title } => {
            render_help_panel(frame, content, title, area)
        }
    }
}

/// Renderiza árvore de arquivos com dica sobre comando
fn render_file_tree_with_hint(frame: &mut Frame, path: &std::path::Path, input: &str, area: Rect) {
    // Se tem input, divide área em dica + arquivos
    if !input.is_empty() {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if let Some(hint) = get_command_hint(parts.get(0).copied().unwrap_or("")) {
            // Divide área: 30% dica, 70% arquivos
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
                .split(area);
            
            render_hint(frame, &hint, chunks[0]);
            render_file_tree(frame, path, chunks[1]);
            return;
        }
    }
    
    // Sem input ou sem dica, mostra só arquivos
    render_file_tree(frame, path, area);
}

/// Renderiza a árvore de arquivos
fn render_file_tree(frame: &mut Frame, path: &std::path::Path, area: Rect) {
    use crate::core::filesystem::FileSystemManager;
    
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" 📂 Navegação ")
        .border_style(Style::default().fg(Color::Cyan));
    
    let mut items = Vec::new();
    
    // Adiciona o diretório pai
    items.push(ListItem::new(Line::from(vec![
        Span::styled("📁 ", Style::default().fg(Color::Yellow)),
        Span::raw(".."),
    ])));
    
    // Lista arquivos e diretórios usando FileSystemManager
    if let Ok(entries) = FileSystemManager::list_directory(path) {
        for entry in entries.iter().take(20) { // Limita a 20 itens
            let icon = entry.get_icon();
            let color = if entry.is_dir {
                Color::Yellow
            } else if entry.name.ends_with(".rs") {
                Color::LightRed
            } else if entry.name.ends_with(".sh") {
                Color::Green
            } else if entry.name.ends_with(".toml") || entry.name.ends_with(".json") {
                Color::Blue
            } else if entry.name.ends_with(".md") {
                Color::Cyan
            } else if entry.name.ends_with(".py") {
                Color::Yellow
            } else {
                Color::White
            };
            
            let size_str = if !entry.is_dir {
                format!(" ({})", FileSystemManager::format_size(entry.size))
            } else {
                String::new()
            };
            
            items.push(ListItem::new(Line::from(vec![
                Span::styled(format!("{} ", icon), Style::default().fg(color)),
                Span::raw(entry.name.clone()),
                Span::styled(size_str, Style::default().fg(Color::DarkGray)),
            ])));
        }
    } else {
        items.push(ListItem::new(Line::from(vec![
            Span::styled("⚠ ", Style::default().fg(Color::Red)),
            Span::raw("Erro ao ler diretório"),
        ])));
    }
    
    let list = List::new(items).block(block);
    frame.render_widget(list, area);
}

/// Renderiza o preview de um arquivo
fn render_file_preview(
    frame: &mut Frame,
    path: &std::path::Path,
    content: &str,
    _language: &str,
    area: Rect,
) {
    let filename = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("arquivo");
    
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" 📄 Preview: {} ", filename))
        .border_style(Style::default().fg(Color::Green));
    
    // Preview com tratamento de erro melhorado e sugestões
    let text = if !content.is_empty() {
        content.to_string()
    } else if path.is_dir() {
        format!("❌ Erro: '{}' é um diretório, não um arquivo!\n\n💡 Use 'ls {}' para listar o conteúdo.", filename, filename)
    } else if !path.exists() {
        // Busca arquivos similares
        let mut suggestions = Vec::new();
        
        if let Ok(entries) = std::fs::read_dir(".") {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.starts_with(filename) || name.contains(filename) {
                        suggestions.push(name.to_string());
                    }
                }
            }
        }
        
        let mut msg = format!("❌ Arquivo '{}' não encontrado\n\n", filename);
        if !suggestions.is_empty() {
            msg.push_str("💡 Você quis dizer:\n\n");
            for sugg in suggestions.iter().take(5) {
                msg.push_str(&format!("  → {}\n", sugg));
            }
        }
        msg
    } else if let Ok(file_content) = fs::read_to_string(path) {
        let lines: Vec<&str> = file_content.lines().take(30).collect();
        if lines.is_empty() {
            "[Arquivo vazio]".to_string()
        } else {
            lines.join("\n")
        }
    } else {
        "❌ Erro ao ler arquivo (muito grande ou sem permissão)".to_string()
    };
    
    let paragraph = Paragraph::new(text)
        .block(block)
        .wrap(Wrap { trim: false })
        .style(Style::default().fg(Color::White));
    
    frame.render_widget(paragraph, area);
}

/// Renderiza o monitor de recursos
fn render_resource_monitor(
    frame: &mut Frame,
    cpu_usage: f32,
    memory_used: u64,
    memory_total: u64,
    process_count: usize,
    area: Rect,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" 📊 Monitor de Recursos ")
        .border_style(Style::default().fg(Color::Blue));
    
    // Divide em seções
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(block.inner(area));
    
    frame.render_widget(block, area);
    
    // CPU Gauge
    let cpu_gauge = Gauge::default()
        .block(Block::default().title("CPU Usage"))
        .gauge_style(Style::default().fg(Color::Cyan))
        .percent(cpu_usage as u16);
    frame.render_widget(cpu_gauge, chunks[0]);
    
    // Memory Gauge
    let mem_percent = if memory_total > 0 {
        ((memory_used as f64 / memory_total as f64) * 100.0) as u16
    } else {
        0
    };
    let mem_gauge = Gauge::default()
        .block(Block::default().title("Memory"))
        .gauge_style(Style::default().fg(Color::Yellow))
        .percent(mem_percent);
    frame.render_widget(mem_gauge, chunks[1]);
    
    // Informações adicionais
    let info = Paragraph::new(format!(
        "Processos: {}\nMemória: {} / {} MB",
        process_count,
        memory_used / 1024 / 1024,
        memory_total / 1024 / 1024
    ));
    frame.render_widget(info, chunks[2]);
}

/// Renderiza a zona de perigo
fn render_danger_zone(
    frame: &mut Frame,
    warning: &str,
    command: &str,
    area: Rect,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Double)
        .title(" ⚠️  ZONA DE PERIGO  ⚠️ ")
        .title_bottom(" 🚨 COMANDO DESTRUTIVO DETECTADO 🚨 ")
        .border_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
        .style(Style::default().bg(Color::Black));
    
    let warning_text = vec![
        Line::from(""),
        Line::from(vec![
            Span::raw("                    "),
            Span::styled(
                "╔═══════════════════════════╗",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("                    "),
            Span::styled(
                "║  ⚠️   ATENÇÃO MÁXIMA   ⚠️  ║",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD | Modifier::SLOW_BLINK),
            ),
        ]),
        Line::from(vec![
            Span::raw("                    "),
            Span::styled(
                "╚═══════════════════════════╝",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "   COMANDO DETECTADO:",
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("   ➜ "),
            Span::styled(
                command,
                Style::default()
                    .fg(Color::Red)
                    .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
            ),
        ]),
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "   RISCO:",
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("   "),
            Span::styled(
                warning,
                Style::default()
                    .fg(Color::LightRed)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "   ⚡ CONSEQUÊNCIAS POSSÍVEIS:",
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("   • "),
            Span::styled("Perda permanente de dados", Style::default().fg(Color::Red)),
        ]),
        Line::from(vec![
            Span::raw("   • "),
            Span::styled("Sistema pode ficar instável", Style::default().fg(Color::Red)),
        ]),
        Line::from(vec![
            Span::raw("   • "),
            Span::styled("Danos irreversíveis ao sistema", Style::default().fg(Color::Red)),
        ]),
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "   AÇÕES DISPONÍVEIS:",
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("   "),
            Span::styled(
                "[ ESC ]",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD | Modifier::REVERSED),
            ),
            Span::styled(
                " Cancelar (Recomendado)",
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("   "),
            Span::styled(
                "[ Enter ]",
                Style::default()
                    .fg(Color::Red)
                    .add_modifier(Modifier::BOLD | Modifier::REVERSED),
            ),
            Span::styled(
                " Executar MESMO ASSIM (Perigoso!)",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "   💡 DICA: ",
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "Sempre faça backup antes de executar comandos destrutivos!",
                Style::default().fg(Color::Gray),
            ),
        ]),
    ];
    
    let paragraph = Paragraph::new(warning_text)
        .block(block)
        .wrap(Wrap { trim: false })
        .alignment(ratatui::layout::Alignment::Left);
    
    frame.render_widget(paragraph, area);
}

/// Renderiza mensagens de gamificação
fn render_gamification(
    frame: &mut Frame,
    message: &str,
    celebration: bool,
    area: Rect,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" 🎮 Level Up! ")
        .border_style(Style::default().fg(Color::Magenta))
        .style(Style::default().bg(Color::Black));
    
    let icon = if celebration { "🎉" } else { "⭐" };
    
    let text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                format!("{} {} {}", icon, message, icon),
                Style::default()
                    .fg(Color::LightMagenta)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
    ];
    
    let paragraph = Paragraph::new(text)
        .block(block)
        .wrap(Wrap { trim: false });
    
    frame.render_widget(paragraph, area);
}

/// Retorna dica sobre comando
fn get_command_hint(command: &str) -> Option<String> {
    match command {
        // Navegação básica
        "ls" => Some("💡 'ls' lista arquivos\nFato: vem de 'LiSt'".to_string()),
        "cd" => Some("💡 'cd' muda diretório\nDica: use TAB!".to_string()),
        "pwd" => Some("💡 pwd mostra local\nPrint Working Dir".to_string()),
        
        // Arquivos
        "cat" => Some("💡 'cat' mostra arquivo\nCurioso: concatena!".to_string()),
        "mkdir" => Some("⚠️ mkdir cria PASTA\nPara arquivo: 'touch'".to_string()),
        "touch" => Some("💡 touch cria ARQUIVO\nPara pasta: 'mkdir'".to_string()),
        "rm" => Some("🚨 rm remove ARQUIVO\nPasta: 'rm -r'".to_string()),
        "cp" => Some("💡 cp copia arquivos\ncp origem destino".to_string()),
        "mv" => Some("💡 mv move/renomeia\nmv antigo novo".to_string()),
        
        // Editores
        "nano" | "vim" => Some("💡 Editor de texto\nNano: Ctrl+X sai".to_string()),
        
        // Sistema
        "sudo" => Some("🚨 SUPER USUÁRIO\nCuidado! Poder total".to_string()),
        "top" | "htop" => Some("💡 Monitor de processos\nq para sair".to_string()),
        
        // Pacman (Arch/Manjaro)
        "pacman" => Some("📦 Gerenciador Arch/Manjaro\n-S instala | -R remove | -Syu atualiza".to_string()),
        "yay" | "paru" => Some("📦 AUR Helper (Manjaro)\nMesmo uso do pacman + AUR".to_string()),
        
        // APT (Debian/Ubuntu)
        "apt" | "apt-get" => Some("📦 Gerenciador Debian/Ubuntu\ninstall | remove | update | upgrade".to_string()),
        
        // DNF/YUM (Fedora/RHEL)
        "dnf" | "yum" => Some("📦 Gerenciador Fedora/RHEL\ninstall | remove | update".to_string()),
        
        // Zypper (openSUSE)
        "zypper" => Some("📦 Gerenciador openSUSE\nin instala | rm remove | up atualiza".to_string()),
        
        // Universal
        "snap" => Some("📦 Snap (Universal)\ninstall | remove | refresh".to_string()),
        "flatpak" => Some("📦 Flatpak (Universal)\ninstall | uninstall | update".to_string()),
        
        // Rede
        "ping" => Some("🌐 Testa conexão\nping google.com".to_string()),
        "curl" | "wget" => Some("🌐 Baixa da internet\ncurl/wget URL".to_string()),
        "ssh" => Some("🌐 Acesso remoto\nssh user@host".to_string()),
        
        // Git
        "git" => Some("📚 Controle de versão\nclone | pull | push | commit".to_string()),
        
        // Compressão
        "tar" => Some("📦 Arquivamento\n-czf compacta | -xzf extrai".to_string()),
        "zip" | "unzip" => Some("📦 Compressão ZIP\nzip arquivo.zip | unzip arquivo.zip".to_string()),
        
        // Systemd
        "systemctl" => Some("⚙️ Gerencia serviços\nstart | stop | restart | status".to_string()),
        
        _ => None,
    }
}

/// Retorna comandos sugeridos baseados no nível do jogador
fn get_level_commands(level: u32) -> Vec<(&'static str, &'static str)> {
    if level < 5 {
        // Nível 1-4: Iniciante - Comandos básicos
        vec![
            ("ls", "lista arquivos"),
            ("cd", "muda diretório"),
            ("pwd", "mostra local atual"),
            ("mkdir", "cria pasta"),
            ("touch", "cria arquivo"),
        ]
    } else if level < 10 {
        // Nível 5-9: Aprendiz - Manipulação de arquivos
        vec![
            ("cat", "mostra conteúdo"),
            ("cp", "copia arquivos"),
            ("mv", "move/renomeia"),
            ("rm", "remove (cuidado!)"),
            ("grep", "busca em texto"),
        ]
    } else if level < 20 {
        // Nível 10-19: Terminal - Comandos intermediários
        vec![
            ("nano", "editor de texto"),
            ("find", "busca arquivos"),
            ("chmod", "muda permissões"),
            ("tar", "compacta/extrai"),
            ("ps", "lista processos"),
        ]
    } else if level < 30 {
        // Nível 20-29: Hacker - Rede e sistema
        vec![
            ("ssh", "acesso remoto"),
            ("git", "versionamento"),
            ("curl", "requisições HTTP"),
            ("netstat", "conexões de rede"),
            ("systemctl", "gerencia serviços"),
        ]
    } else if level < 40 {
        // Nível 30-39: Cyberpunk - Package managers
        vec![
            ("pacman -Syu", "atualiza sistema"),
            ("yay -S", "instala do AUR"),
            ("apt update", "atualiza repos"),
            ("dnf install", "instala pacote"),
            ("docker", "containers"),
        ]
    } else {
        // Nível 40+: Elite/Legend - Avançado
        vec![
            ("sudo su", "super usuário"),
            ("fdisk", "partições"),
            ("iptables", "firewall"),
            ("cron", "tarefas agendadas"),
            ("make", "compilação"),
        ]
    }
}

/// Renderiza dica compacta
fn render_hint(frame: &mut Frame, hint: &str, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));
    
    let text = Paragraph::new(hint)
        .block(block)
        .style(Style::default().fg(Color::Yellow))
        .wrap(Wrap { trim: false });
    
    frame.render_widget(text, area);
}

/// Renderiza tela de boas-vindas com personagem
fn render_welcome_screen(frame: &mut Frame, app: &App, area: Rect) {
    use crate::ui::theme::Theme;
    
    let theme = app.game_state.get_theme();
    let level = app.game_state.level;
    let rank = app.game_state.get_rank();
    let message = Theme::get_level_message(level);
    let character_lines = Theme::get_character_art(level);
    
    let title = format!(" Munux - {} [Nv {}] ", rank, level);
    let block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .border_style(Style::default().fg(theme.border));
    
    let mut text = vec![Line::from("")];
    
    // Renderiza o personagem com as cores do tema
    for line in character_lines {
        text.push(Line::from(vec![
            Span::styled(
                line,
                Style::default().fg(theme.primary),
            ),
        ]));
    }
    
    text.extend(vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                " Terminal Educacional Reativo",
                Style::default()
                    .fg(theme.accent)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "  Powered by Linux & Rust",
                Style::default()
                    .fg(Color::Gray)
                    .add_modifier(Modifier::ITALIC),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                message,
                Style::default()
                    .fg(theme.success)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "O painel da direita muda conforme",
                Style::default().fg(theme.text),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "você digita comandos!",
                Style::default().fg(theme.text),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "Comandos para começar:",
                Style::default().fg(theme.success).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
    ]);
    
    // Adiciona comandos dinâmicos baseados no nível
    let commands = get_level_commands(level);
    for (cmd, desc) in commands {
        text.push(Line::from(vec![
            Span::styled(format!("  {}", cmd), Style::default().fg(theme.accent)),
            Span::styled(format!("  - {}", desc), Style::default().fg(Color::Gray)),
        ]));
    }
    
    text.extend(vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "\"Talk is cheap. Show me the code.\"",
                Style::default()
                    .fg(theme.secondary)
                    .add_modifier(Modifier::ITALIC),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "            - Linus Torvalds",
                Style::default()
                    .fg(Color::Gray)
                    .add_modifier(Modifier::ITALIC),
            ),
        ]),
    ]);
    
    let paragraph = Paragraph::new(text)
        .block(block)
        .wrap(Wrap { trim: false });
    
    frame.render_widget(paragraph, area);
}

/// Renderiza ajuda sobre comando
fn render_command_help(
    frame: &mut Frame,
    command: &str,
    description: &str,
    examples: &[String],
    tip: &str,
    area: Rect,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" Comando: {} ", command))
        .border_style(Style::default().fg(Color::Green));
    
    let mut lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled(
                description,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "Exemplos:",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
    ];
    
    for example in examples {
        lines.push(Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled(example, Style::default().fg(Color::Yellow)),
        ]));
    }
    
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled(tip, Style::default().fg(Color::Magenta)),
    ]));
    
    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false });
    
    frame.render_widget(paragraph, area);
}

/// Renderiza easter egg
fn render_easter_egg(frame: &mut Frame, content: &str, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" 🥚 Easter Egg! ")
        .border_style(Style::default().fg(Color::Magenta));
    
    let paragraph = Paragraph::new(content)
        .block(block)
        .wrap(Wrap { trim: false })
        .style(Style::default().fg(Color::Cyan));
    
    frame.render_widget(paragraph, area);
}

/// Renderiza painel de ajuda (help)
fn render_help_panel(frame: &mut Frame, content: &str, title: &str, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" 📚 {} ", title))
        .border_style(Style::default().fg(Color::Cyan))
        .title_bottom(" ESC para voltar ");
    
    let paragraph = Paragraph::new(content)
        .block(block)
        .wrap(Wrap { trim: false })
        .style(Style::default().fg(Color::Gray))
        .scroll((0, 0)); // Futuramente pode adicionar scroll
    
    frame.render_widget(paragraph, area);
}
