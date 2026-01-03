// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

use crate::app::{App, PopupType};
use crate::ui::layout;
use ratatui::{
    backend::Backend,
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

/// Renderiza o popup flutuante (Ghost Mentor)
pub fn render_popup(frame: &mut Frame, app: &App) {
    if let Some(popup) = &app.active_popup {
        let area = frame.size();
        let popup_area = layout::create_popup_layout(area, 60, 40);
        
        // Limpa a área do popup
        frame.render_widget(Clear, popup_area);
        
        // Cor baseada no tipo
        let (border_color, icon) = match popup.popup_type {
            PopupType::Info => (Color::Cyan, "ℹ️"),
            PopupType::Warning => (Color::Yellow, "⚠️"),
            PopupType::Success => (Color::Green, "✅"),
            PopupType::Tip => (Color::Magenta, "💡"),
        };
        
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color))
            .title(format!(" {} {} ", icon, popup.title))
            .style(Style::default().bg(Color::Black));
        
        let text = vec![
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    &popup.content,
                    Style::default().fg(Color::White),
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled(
                    "Pressione ESC para fechar",
                    Style::default()
                        .fg(Color::DarkGray)
                        .add_modifier(Modifier::ITALIC),
                ),
            ]),
        ];
        
        let paragraph = Paragraph::new(text)
            .block(block)
            .wrap(Wrap { trim: false })
            .alignment(Alignment::Center);
        
        frame.render_widget(paragraph, popup_area);
    }
}
