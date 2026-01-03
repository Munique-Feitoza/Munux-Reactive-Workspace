// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

pub mod layout;
pub mod terminal;
pub mod reactive;
pub mod hud;
pub mod popup;
pub mod theme;
pub mod stats;

use crate::app::App;
use ratatui::Frame;

/// Renderiza toda a interface do usuário
pub fn render(frame: &mut Frame, app: &App) {
    let chunks = layout::create_main_layout(frame.size());
    
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
