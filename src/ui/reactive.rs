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
    // O `Clear` global (ui::render) já evita ghosting; um segundo Clear aqui
    // apagaria o background do tema neste painel. Por isso não limpamos de novo.
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
fn render_file_tree(frame: &mut Frame, _path: &std::path::Path, app: &App, area: Rect) {
    use crate::core::filesystem::FileSystemManager;

    let browsing = app.is_browsing_files();
    let mut block =
        crate::ui::panel_block(format!(" 📂 {} ", app.i18n.navigation_title()), Color::Cyan);
    if browsing {
        block = block.title_bottom(format!(" {} ", app.i18n.tc("ui-browse-hint")));
    }

    let mut items = Vec::new();

    // Diretório pai (referência visual; não selecionável)
    items.push(ListItem::new(Line::from(vec![
        Span::raw("  "),
        Span::styled("📁 ", Style::default().fg(Color::Yellow)),
        Span::raw(".."),
    ])));

    // Lista compartilhada com a navegação (mesma ordem e limite).
    let entries = app.dir_entries();
    if entries.is_empty() && FileSystemManager::list_directory(&app.current_dir).is_err() {
        items.push(ListItem::new(Line::from(vec![
            Span::styled("⚠ ", Style::default().fg(Color::Red)),
            Span::raw(app.i18n.tc("ui-err-read-dir")),
        ])));
    } else {
        for (i, entry) in entries.iter().enumerate() {
            let icon = entry.get_icon();
            let color = if entry.is_dir {
                Color::Yellow
            } else {
                crate::core::filetype::classify(&entry.name).color
            };

            let size_str = if !entry.is_dir {
                format!(" ({})", FileSystemManager::format_size(entry.size))
            } else {
                String::new()
            };

            let selected = browsing && i == app.file_selection;
            let name_style = if selected {
                Style::default().fg(color).add_modifier(Modifier::BOLD | Modifier::REVERSED)
            } else {
                Style::default().fg(color)
            };

            items.push(ListItem::new(Line::from(vec![
                Span::styled(if selected { "▶ " } else { "  " }, Style::default().fg(Color::Cyan)),
                Span::styled(format!("{} ", icon), Style::default().fg(color)),
                Span::styled(entry.name.clone(), name_style),
                Span::styled(size_str, Style::default().fg(Color::DarkGray)),
            ])));
        }
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
        let block = crate::ui::panel_block(
            format!(" 📄 {} [{}] ", app.i18n.preview_title(filename), language),
            Color::Green,
        );

        let lines = crate::ui::highlight::highlight(content, language);
        let paragraph = Paragraph::new(lines)
            .block(block)
            .wrap(Wrap { trim: false })
            .scroll((app.scroll, 0));
        frame.render_widget(paragraph, area);
        return;
    }

    let block = crate::ui::panel_block(
        format!(" 📄 {} ", app.i18n.preview_title(filename)),
        Color::Green,
    );

    // Preview com tratamento de erro melhorado e sugestões
    let text = if !content.is_empty() {
        content.to_string()
    } else if path.is_dir() {
        let mut args = FluentArgs::new();
        args.set("name", filename);
        format!("{}\n\n{}", app.i18n.t("ui-err-is-dir", Some(&args)), app.i18n.t("ui-err-is-dir-hint", Some(&args)))
    } else if !path.exists() {
        // Sugestões de arquivos similares são geradas no update (parser, no
        // diretório lógico correto) e chegam prontas em `content`. Aqui é só um
        // fallback simples — sem varrer o diretório do processo (que após `cd`
        // estaria errado).
        format!("❌ Arquivo '{}' não encontrado", filename)
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
    let block = crate::ui::panel_block(
        format!(" 📊 {} ", app.i18n.resource_title()),
        Color::Blue,
    );

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
    
    // CPU Gauge (clamp em 100: Gauge::percent dá panic se passar de 100)
    let cpu_gauge = Gauge::default()
        .block(Block::default().title(app.i18n.cpu_usage_label()))
        .gauge_style(Style::default().fg(Color::Cyan))
        .percent((cpu_usage as u16).min(100));
    frame.render_widget(cpu_gauge, chunks[0]);

    // Memory Gauge (cálculo único e protegido contra divisão por zero)
    let mem_percent = crate::core::monitor::mem_percent(memory_used, memory_total) as u16;
    let mem_gauge = Gauge::default()
        .block(Block::default().title(app.i18n.tc("ui-memory")))
        .gauge_style(Style::default().fg(Color::Yellow))
        .percent(mem_percent.min(100));
    frame.render_widget(mem_gauge, chunks[1]);
    
    // Informações + Top-5 processos por uso de CPU.
    let mut info_lines = vec![
        Line::from(format!(
            "{}: {}    {}: {} / {} MB",
            app.i18n.tc("ui-processes"),
            process_count,
            app.i18n.tc("ui-memory"),
            memory_used / 1024 / 1024,
            memory_total / 1024 / 1024
        )),
        Line::from(""),
        Line::from(Span::styled(
            app.i18n.tc("ui-top-processes"),
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
    ];
    for p in app.system_summary.top_processes.iter().take(5) {
        let name: String = p.name.chars().take(14).collect();
        info_lines.push(Line::from(vec![
            Span::styled(format!("{:>7} ", p.pid), Style::default().fg(Color::DarkGray)),
            Span::styled(format!("{:<14} ", name), Style::default().fg(Color::White)),
            Span::styled(format!("{:>5.1}% ", p.cpu_usage), Style::default().fg(Color::Cyan)),
            Span::styled(
                format!("{:>6} MB", p.memory / 1024 / 1024),
                Style::default().fg(Color::Yellow),
            ),
        ]));
    }
    frame.render_widget(Paragraph::new(info_lines), chunks[2]);
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
                "[ sim+Enter ]",
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
    let block = crate::ui::panel_block(
        format!(" 🎮 {} ", app.i18n.level_up_title()),
        Color::Magenta,
    );

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
    let block = crate::ui::panel_block(title, theme.border);

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
        .title_bottom(format!(" {}  ·  {} ", app.i18n.esc_to_back(), app.i18n.scroll_hint()));

    let paragraph = Paragraph::new(content)
        .block(block)
        .wrap(Wrap { trim: false })
        .style(Style::default().fg(Color::Gray))
        .scroll((app.scroll, 0)); // PageUp/PageDown e roda do mouse controlam app.scroll

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
