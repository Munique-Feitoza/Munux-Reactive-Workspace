// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

use crate::app::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};

/// Renderiza a barra de status (HUD) com XP, Nível e Integridade
pub fn render_hud(frame: &mut Frame, app: &App, area: Rect) {
    use crate::ui::theme::Theme;
    
    let theme = app.game_state.get_theme();
    
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border))
        .style(Style::default().bg(Color::Black));
    
    let inner = block.inner(area);
    frame.render_widget(block, area);
    
    // Divide o HUD em 3 partes: Info | XP Bar | Integridade
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(25),  // Nível e Rank
            Constraint::Min(20),     // Barra de XP
            Constraint::Length(20),  // Integridade
        ])
        .split(inner);
    
    // 1. Informações do Jogador (Nível e Rank)
    let rank = app.game_state.get_rank(&app.i18n);
    let symbol = Theme::get_prompt_symbol(app.game_state.level);
    
    let info_text = Line::from(vec![
        Span::styled(
            format!(" {} ", symbol),
            Style::default()
                .fg(theme.accent)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            format!("[{} {}] ", app.i18n.tc("ui-level"), app.game_state.level),
            Style::default()
                .fg(theme.primary)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            rank,
            Style::default()
                .fg(theme.text)
                .add_modifier(Modifier::ITALIC),
        ),
    ]);
    
    let info = Paragraph::new(info_text);
    frame.render_widget(info, chunks[0]);
    
    // 2. Barra de XP (cálculo único e protegido contra divisão por zero)
    let xp_pct = app.game_state.xp_progress();
    let xp_label = app.i18n.xp_label(
        app.game_state.xp,
        app.game_state.xp_to_next_level,
        xp_pct,
    );

    let xp_gauge = Gauge::default()
        .label(xp_label)
        .gauge_style(
            Style::default()
                .fg(theme.primary)
                .add_modifier(Modifier::BOLD)
        )
        .percent(xp_pct as u16);
    
    frame.render_widget(xp_gauge, chunks[1]);
    
    // 3. Integridade do Sistema (cor pela fonte única)
    let integrity_color = crate::ui::theme::health_color(app.game_state.integrity);
    
    let integrity_text = Line::from(vec![
        Span::styled(
            " ❤️  ",
            Style::default().fg(integrity_color),
        ),
        Span::styled(
            format!("{}%", app.game_state.integrity),
            Style::default()
                .fg(integrity_color)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
        Span::styled(
            if app.game_state.safe_mode { "🔒" } else { "🔓" },
            Style::default().fg(Color::Gray),
        ),
    ]);
    
    let integrity = Paragraph::new(integrity_text);
    frame.render_widget(integrity, chunks[2]);
}
