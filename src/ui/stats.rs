// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

use crate::app::App;
use crate::game::state::GameState;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

/// Renderiza painel de estatísticas do jogador
pub fn render_stats_panel(frame: &mut Frame, app: &App, area: Rect) {
    
    let theme = app.game_state.get_theme();
    
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" 📊 Estatísticas - {} ", app.game_state.get_rank()))
        .border_style(Style::default().fg(theme.border));
    
    let inner = block.inner(area);
    frame.render_widget(block, area);
    
    // Divide em seções
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10), // Estatísticas gerais
            Constraint::Min(8),      // Conquistas recentes
        ])
        .split(inner);
    
    // Estatísticas gerais
    render_general_stats(frame, &app.game_state, &theme, chunks[0]);
    
    // Conquistas recentes
    render_recent_achievements(frame, &app.game_state, &theme, chunks[1]);
}

fn render_general_stats(frame: &mut Frame, game_state: &GameState, theme: &crate::ui::theme::Theme, area: Rect) {
    use crate::core::monitor::SystemMonitor;
    
    let success_rate = game_state.success_rate();
    
    // Captura informações do sistema
    let mut monitor = SystemMonitor::new();
    let cpu = monitor.get_cpu_usage();
    let (mem_used, mem_total) = monitor.get_memory_info();
    let mem_percent = (mem_used as f64 / mem_total as f64 * 100.0) as f32;
    
    let lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("⚡ Comandos Totais: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{}", game_state.total_commands),
                Style::default().fg(theme.primary).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("✓ Bem-sucedidos: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{}", game_state.successful_commands),
                Style::default().fg(theme.success),
            ),
            Span::styled(" | ", Style::default().fg(Color::DarkGray)),
            Span::styled("✗ Falhas: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{}", game_state.failed_commands),
                Style::default().fg(theme.danger),
            ),
        ]),
        Line::from(vec![
            Span::styled("📈 Taxa de Acerto: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{:.1}%", success_rate),
                Style::default().fg(
                    if success_rate >= 80.0 { theme.success }
                    else if success_rate >= 50.0 { theme.warning }
                    else { theme.danger }
                ).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("🔥 Streak Atual: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{} comandos", game_state.command_streak),
                Style::default().fg(theme.accent).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("💚 Integridade: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{}%", game_state.integrity),
                Style::default().fg(
                    if game_state.integrity >= 80 { Color::Green }
                    else if game_state.integrity >= 50 { Color::Yellow }
                    else { Color::Red }
                ).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("💻 CPU: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{:.1}%", cpu),
                Style::default().fg(if cpu > 80.0 { Color::Red } else { Color::Cyan }),
            ),
            Span::styled(" | ", Style::default().fg(Color::DarkGray)),
            Span::styled("🧠 RAM: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{:.1}%", mem_percent),
                Style::default().fg(if mem_percent > 80.0 { Color::Red } else { Color::Cyan }),
            ),
        ]),
        Line::from(vec![
            Span::styled("🏆 Conquistas: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{}", game_state.achievements.len()),
                Style::default().fg(theme.warning).add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(vec![
            Span::styled("📋 Quests Ativas: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{}", game_state.active_quests.iter().filter(|q| !q.completed).count()),
                Style::default().fg(theme.secondary).add_modifier(Modifier::BOLD),
            ),
        ]),
    ];
    
    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, area);
}

fn render_recent_achievements(frame: &mut Frame, game_state: &GameState, theme: &crate::ui::theme::Theme, area: Rect) {
    let block = Block::default()
        .borders(Borders::TOP)
        .title(" Conquistas Recentes ")
        .border_style(Style::default().fg(Color::DarkGray));
    
    let inner = block.inner(area);
    frame.render_widget(block, area);
    
    let recent: Vec<ListItem> = game_state
        .achievements
        .iter()
        .rev()
        .take(5)
        .map(|achievement| {
            ListItem::new(Line::from(vec![
                Span::styled("🏆 ", Style::default().fg(theme.warning)),
                Span::styled(
                    &achievement.name,
                    Style::default().fg(theme.primary).add_modifier(Modifier::BOLD),
                ),
                Span::styled(" - ", Style::default().fg(Color::DarkGray)),
                Span::styled(
                    &achievement.description,
                    Style::default().fg(Color::Gray),
                ),
            ]))
        })
        .collect();
    
    if recent.is_empty() {
        let text = Paragraph::new(Line::from(vec![
            Span::styled(
                "Nenhuma conquista ainda. Execute comandos para desbloquear!",
                Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC),
            ),
        ]));
        frame.render_widget(text, inner);
    } else {
        let list = List::new(recent);
        frame.render_widget(list, inner);
    }
}

/// Renderiza painel de quests
pub fn render_quests_panel(frame: &mut Frame, app: &App, area: Rect) {
    let theme = app.game_state.get_theme();
    
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" 📋 Missões Ativas ")
        .border_style(Style::default().fg(theme.border));
    
    let inner = block.inner(area);
    frame.render_widget(block, area);
    
    let quests: Vec<ListItem> = app
        .game_state
        .active_quests
        .iter()
        .filter(|q| !q.completed)
        .map(|quest| {
            let status = if quest.is_complete() { "✓" } else { "○" };
            let status_color = if quest.is_complete() { theme.success } else { Color::Gray };
            
            ListItem::new(vec![
                Line::from(vec![
                    Span::styled(format!("{} ", status), Style::default().fg(status_color)),
                    Span::styled(
                        &quest.title,
                        Style::default().fg(theme.primary).add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!(" (+{} XP)", quest.xp_reward),
                        Style::default().fg(theme.warning),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("  ", Style::default()),
                    Span::styled(
                        &quest.description,
                        Style::default().fg(Color::Gray).add_modifier(Modifier::ITALIC),
                    ),
                ]),
                Line::from(vec![
                    Span::styled("  ", Style::default()),
                    Span::styled(
                        quest.get_progress_text(),
                        Style::default().fg(theme.secondary),
                    ),
                ]),
                Line::from(""),
            ])
        })
        .collect();
    
    if quests.is_empty() {
        let text = Paragraph::new(vec![
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "🎉 Todas as missões concluídas!",
                    Style::default().fg(theme.success).add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "Novas missões serão desbloqueadas ao subir de nível.",
                    Style::default().fg(Color::Gray).add_modifier(Modifier::ITALIC),
                ),
            ]),
        ]);
        frame.render_widget(text, inner);
    } else {
        let list = List::new(quests);
        frame.render_widget(list, inner);
    }
}
