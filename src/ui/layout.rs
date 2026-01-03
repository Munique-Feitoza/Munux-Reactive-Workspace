// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
};

/// Cria o layout principal: Split Screen + HUD
pub fn create_main_layout(area: Rect) -> Vec<Rect> {
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),      // Área principal (Terminal + Reactive)
            Constraint::Length(3),   // HUD (Barra de status)
        ])
        .split(area);
    
    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(60), // Terminal Ativo (esquerda)
            Constraint::Percentage(40), // Painel Reativo (direita)
        ])
        .split(vertical_chunks[0]);
    
    vec![
        horizontal_chunks[0], // Terminal
        horizontal_chunks[1], // Reactive Panel
        vertical_chunks[1],   // HUD
    ]
}

/// Cria um popup centralizado
pub fn create_popup_layout(area: Rect, width_percent: u16, height_percent: u16) -> Rect {
    let popup_width = (area.width * width_percent) / 100;
    let popup_height = (area.height * height_percent) / 100;
    
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length((area.height - popup_height) / 2),
            Constraint::Length(popup_height),
            Constraint::Length((area.height - popup_height) / 2),
        ])
        .split(area);
    
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length((area.width - popup_width) / 2),
            Constraint::Length(popup_width),
            Constraint::Length((area.width - popup_width) / 2),
        ])
        .split(vertical[1])[1]
}
