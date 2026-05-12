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
use fluent::FluentArgs;
use std::fs;

/// Renderiza o painel direito reativo (o "camaleão")
pub fn render_reactive_panel(frame: &mut Frame, app: &App, area: Rect) {
    use ratatui::widgets::Clear;
    
    // Limpa área para evitar ghosting
    frame.render_widget(Clear, area);

    match &app.right_panel_mode {
        RightPanelMode::Welcome => render_welcome_screen(frame, app, area),
        RightPanelMode::FileTree { path } => {
            // Se tiver input, mostra dica + arquivos
            render_file_tree_with_hint(frame, path, &app.input_buffer, app, area)
        }
        RightPanelMode::FilePreview { path, content, language } => {
            render_file_preview(frame, path, content, language, app, area)
        }
        RightPanelMode::ResourceMonitor { cpu_usage, memory_used, memory_total, process_count } => {
            render_resource_monitor(frame, *cpu_usage, *memory_used, *memory_total, *process_count, app, area)
        }
        RightPanelMode::DangerZone { warning, command } => {
            render_danger_zone(frame, warning, command, app, area)
        }
        RightPanelMode::Gamification { message, celebration } => {
            render_gamification(frame, message, *celebration, app, area)
        }
        RightPanelMode::Stats => {
            crate::ui::stats::render_stats_panel(frame, app, area)
        }
        RightPanelMode::Quests => {
            crate::ui::stats::render_quests_panel(frame, app, area)
        }
        RightPanelMode::EasterEgg { content } => {
            render_easter_egg(frame, content, app, area)
        }
        RightPanelMode::Help { content, title } => {
            render_help_panel(frame, content, title, app, area)
        }
        RightPanelMode::CommandHelp { command, description, examples, tip } => {
            render_command_help(frame, command, description, examples, tip, &app.i18n, area)
        }
        RightPanelMode::CommandOutput(content) => {
            render_command_output(frame, content, app.scroll, app, area)
        }
    }
}

fn render_file_tree_with_hint(frame: &mut Frame, path: &std::path::Path, input: &str, app: &App, area: Rect) {
    // Se tem input, divide área em dica + arquivos
    if !input.is_empty() {
        let parts: Vec<&str> = input.split_whitespace().collect();
        if let Some(hint) = get_command_hint(parts.first().copied().unwrap_or(""), app) {
            // Divide área: 30% dica, 70% arquivos
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
                .split(area);
            
            render_hint(frame, &hint, chunks[0]);
            render_file_tree(frame, path, app, chunks[1]);
            return;
        }
    }
    
    // Sem input ou sem dica, mostra só arquivos
    render_file_tree(frame, path, app, area);
}

/// Renderiza a árvore de arquivos
fn render_file_tree(frame: &mut Frame, path: &std::path::Path, app: &App, area: Rect) {
    use crate::core::filesystem::FileSystemManager;
    
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" 📂 {} ", app.i18n.navigation_title()))
        .border_style(Style::default().fg(Color::Cyan))
        .style(Style::default().bg(Color::Black));
    
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
            } else {
                match entry.name.rsplit('.').next() {
                    Some("rs") => Color::LightRed,
                    Some("sh") => Color::Green,
                    Some("toml") | Some("json") => Color::Blue,
                    Some("md") => Color::Cyan,
                    Some("py") => Color::Yellow,
                    _ => Color::White,
                }
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
            Span::raw(app.i18n.tc("ui-err-read-dir")),
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
    language: &str,
    app: &App,
    area: Rect,
) {
    let filename = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("arquivo");

    // Preview com realce de sintaxe para linguagens suportadas.
    if !content.is_empty() && crate::ui::highlight::is_supported(language) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(format!(" 📄 {} [{}] ", app.i18n.preview_title(filename), language))
            .border_style(Style::default().fg(Color::Green))
            .style(Style::default().bg(Color::Black));

        let lines = crate::ui::highlight::highlight(content, language);
        let paragraph = Paragraph::new(lines)
            .block(block)
            .wrap(Wrap { trim: false })
            .scroll((app.scroll, 0));
        frame.render_widget(paragraph, area);
        return;
    }

    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" 📄 {} ", app.i18n.preview_title(filename)))
        .border_style(Style::default().fg(Color::Green))
        .style(Style::default().bg(Color::Black));

    // Preview com tratamento de erro melhorado e sugestões
    let text = if !content.is_empty() {
        content.to_string()
    } else if path.is_dir() {
        let mut args = FluentArgs::new();
        args.set("name", filename);
        format!("{}\n\n{}", app.i18n.t("ui-err-is-dir", Some(&args)), app.i18n.t("ui-err-is-dir-hint", Some(&args)))
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
            format!("[{}]", app.i18n.tc("ui-empty-file"))
        } else {
            lines.join("\n")
        }
    } else {
        app.i18n.tc("ui-err-read-file")
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
    app: &App,
    area: Rect,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" 📊 {} ", app.i18n.resource_title()))
        .border_style(Style::default().fg(Color::Blue))
        .style(Style::default().bg(Color::Black));
    
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
        .block(Block::default().title(app.i18n.cpu_usage_label()))
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
        .block(Block::default().title(app.i18n.tc("ui-memory")))
        .gauge_style(Style::default().fg(Color::Yellow))
        .percent(mem_percent);
    frame.render_widget(mem_gauge, chunks[1]);
    
    // Informações adicionais
    let info = Paragraph::new(format!(
        "{}: {}\n{}: {} / {} MB",
        app.i18n.tc("ui-processes"),
        process_count,
        app.i18n.tc("ui-memory"),
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
    app: &App,
    area: Rect,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(ratatui::widgets::BorderType::Double)
        .title(format!(" ⚠️  {}  ⚠️ ", app.i18n.danger_title()))
        .title_bottom(format!(" 🚨 {} 🚨 ", app.i18n.tc("sys-danger-detected")))
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
                format!("║  ⚠️   {}   ⚠️  ║", app.i18n.tc("ui-attention-max")),
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
                format!("   {}:", app.i18n.tc("ui-command-detected")),
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
                format!("   {}:", app.i18n.tc("ui-risk")),
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
                format!("   ⚡ {}:", app.i18n.tc("sys-consequences")),
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::raw("   • "),
            Span::styled(app.i18n.tc("ui-data-loss"), Style::default().fg(Color::Red)),
        ]),
        Line::from(vec![
            Span::raw("   • "),
            Span::styled(app.i18n.tc("ui-unstable-system"), Style::default().fg(Color::Red)),
        ]),
        Line::from(vec![
            Span::raw("   • "),
            Span::styled(app.i18n.tc("ui-irreversible-damage"), Style::default().fg(Color::Red)),
        ]),
        Line::from(""),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                format!("   {}:", app.i18n.tc("ui-available-actions")),
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
                format!(" {}", app.i18n.tc("ui-cancel-rec")),
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
                format!(" {}", app.i18n.tc("ui-execute-anyway")),
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
                app.i18n.tc("ui-backup-tip"),
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
    app: &App,
    area: Rect,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" 🎮 {} ", app.i18n.level_up_title()))
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
fn get_command_hint(command: &str, app: &App) -> Option<String> {
    app.i18n.command_hint(command)
}

/// Retorna comandos sugeridos baseados no nível do jogador
fn get_level_commands(level: u32, app: &App) -> Vec<(&'static str, String)> {
    app.i18n.level_commands(level)
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
    let rank = app.game_state.get_rank(&app.i18n);
    let message = app.i18n.level_message(level);
    let character_lines = Theme::get_character_art(level, &app.i18n);
    
    let title = format!(" Munux - {} [Nv {}] ", rank, level);
    let block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .border_style(Style::default().fg(theme.border))
        .style(Style::default().bg(Color::Black));
    
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
                format!(" {}", app.i18n.tc("ui-terminal-title")),
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
                app.i18n.tc("ui-reactive-desc"),
                Style::default().fg(theme.text),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                app.i18n.tc("ui-reactive-desc-2"),
                Style::default().fg(theme.text),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                format!("{}:", app.i18n.tc("ui-start-commands")),
                Style::default().fg(theme.success).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
    ]);
    
    // Adiciona comandos dinâmicos baseados no nível
    let commands = get_level_commands(level, app);
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
    i18n: &crate::i18n::I18n,
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
    ];
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled(format!("{}: ", i18n.tc("ui-examples")), Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
    ]));
    
    for example in examples {
        lines.push(Line::from(vec![
            Span::styled("  ", Style::default()),
            Span::styled(example, Style::default().fg(Color::Yellow)),
        ]));
    }
    
    lines.push(Line::from(""));
    lines.push(Line::from(vec![
        Span::styled(format!("{}: ", i18n.tc("ui-tip")), Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD)),
        Span::styled(tip, Style::default().fg(Color::Magenta)),
    ]));
    
    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false });
    
    frame.render_widget(paragraph, area);
}

/// Renderiza easter egg
fn render_easter_egg(frame: &mut Frame, content: &str, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" 🥚 {} ", app.i18n.tc("ui-easter-egg")))
        .border_style(Style::default().fg(Color::Magenta));
    
    let paragraph = Paragraph::new(content)
        .block(block)
        .wrap(Wrap { trim: false })
        .style(Style::default().fg(Color::Cyan));
    
    frame.render_widget(paragraph, area);
}

/// Renderiza painel de ajuda (help)
fn render_help_panel(frame: &mut Frame, content: &str, title: &str, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" 📚 {} ", title))
        .border_style(Style::default().fg(Color::Cyan))
        .title_bottom(format!(" {} ", app.i18n.esc_to_back()));
    
    let paragraph = Paragraph::new(content)
        .block(block)
        .wrap(Wrap { trim: false })
        .style(Style::default().fg(Color::Gray))
        .scroll((0, 0)); // Futuramente pode adicionar scroll
    
    frame.render_widget(paragraph, area);
}

/// Renderiza output de comando com scroll
fn render_command_output(frame: &mut Frame, content: &str, scroll: u16, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title_bottom(format!(" {} ", app.i18n.scroll_hint()))
        .border_style(Style::default().fg(Color::DarkGray)) // Borda discreta estilo terminal
        .style(Style::default().bg(Color::Black)) 
        .padding(ratatui::widgets::Padding::new(1, 1, 0, 0)); // Padding ajustado
    
    // Converte conteúdo com códigos ANSI para Text do Ratatui com cores preservadas
    // Usando into_text() que lida internamente com ANSI se disponível ou Text::from(content)


    use ansi_to_tui::IntoText;
    // Usa ansi-to-tui para converter string com ANSI para Text do Ratatui
    let text = content.into_text().unwrap_or_else(|_| ratatui::text::Text::from(content));
    
    let paragraph = Paragraph::new(text)
        .block(block)
        .wrap(Wrap { trim: false })
        .scroll((scroll, 0));
    
    frame.render_widget(paragraph, area);
}
