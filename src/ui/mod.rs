// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

pub mod layout;
pub mod terminal;
pub mod reactive;
pub mod hud;
pub mod popup;
pub mod theme;
pub mod stats;
pub mod highlight;

use crate::app::App;
use ratatui::Frame;

/// Renderiza toda a interface do usuário
pub fn render(frame: &mut Frame, app: &App) {
    // Renderiza background global
    use ratatui::widgets::Block;
    use ratatui::style::Style;
    
    let theme = app.game_state.get_theme();
    
    // Limpa a tela inteira para evitar qualquer ghosting ou bleed-through entre painéis
    use ratatui::widgets::Clear;
    frame.render_widget(Clear, frame.area());
    
    let bg_block = Block::default().style(Style::default().bg(theme.get_background()));
    frame.render_widget(bg_block, frame.area());

    let chunks = layout::create_main_layout(frame.area());
    
    // Renderiza o painel esquerdo (Terminal Ativo)
    terminal::render_terminal_panel(frame, app, chunks[0]);
    
    // Renderiza o painel direito (Visualizador Reativo)
    reactive::render_reactive_panel(frame, app, chunks[1]);
    
    // Renderiza o HUD (Barra de Status)
    hud::render_hud(frame, app, chunks[2]);
    
    // Renderiza popup se estiver ativo
    if app.active_popup.is_some() {
        popup::render_popup(frame, app);
    }
}
