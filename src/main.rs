// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

mod app;
mod event;
mod tui;
mod ui;
mod core;
mod game;
mod i18n;

use anyhow::Result;
use app::App;
use crossterm::event::{KeyCode, KeyModifiers};
use event::{Event, EventHandler};

fn main() -> Result<()> {
    // Configura panic hook para garantir que o terminal seja restaurado
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic| {
        let _ = tui::restore();
        original_hook(panic);
    }));
    
    // Inicializa o terminal
    let mut terminal = tui::init()?;
    
    // Cria a aplicação
    let mut app = App::new()?;
    
    // Cria o gerenciador de eventos
    let event_handler = EventHandler::default();
    
    // Mensagem de boas-vindas
    app.show_popup(
        app.i18n.welcome_title(),
        app.i18n.tc("sys-welcome-body"),
        app::PopupType::Info,
    );
    
    // Loop principal (The Elm Architecture)
    let result = run(&mut terminal, &mut app, event_handler);

    // Salva o progresso ao sair
    app.save_progress();

    // Restaura o terminal
    tui::restore()?;

    result
}

/// Loop principal da aplicação
fn run(
    terminal: &mut tui::Tui,
    app: &mut App,
    event_handler: EventHandler,
) -> Result<()> {
    // Primeiro draw
    terminal.draw(|frame| {
        ui::render(frame, app);
    })?;
    
    loop {
        // EVENT: Aguarda próximo evento (com timeout)
        let event = event_handler.next()?;
        
        // UPDATE: Processa o evento e atualiza o estado
        handle_event(app, event)?;
        
        // Verifica se deve sair ANTES de redesenhar
        if app.should_quit {
            break;
        }
        
        // DRAW: Renderiza a UI apenas DEPOIS de processar o evento
        terminal.draw(|frame| {
            ui::render(frame, app);
        })?;
    }
    
    Ok(())
}

/// Processa eventos e atualiza o estado da aplicação
fn handle_event(app: &mut App, event: Event) -> Result<()> {
    match event {
        Event::Key(key_event) => {
            // Se há popup ativo, ESC fecha o popup
            if app.active_popup.is_some() {
                if key_event.code == KeyCode::Esc {
                    app.close_popup();
                }
                return Ok(());
            }
            
            // Processa teclas com modificadores
            if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                match key_event.code {
                    KeyCode::Char('c') | KeyCode::Char('C') => {
                        app.should_quit = true;
                    }
                    KeyCode::Char('l') | KeyCode::Char('L') => {
                        // Limpa a tela (Ctrl+L)
                        app.command_history.clear();
                        app.clear_input();
                    }
                    _ => {}
                }
                return Ok(());
            }
            
            // Processa teclas normais
            match key_event.code {
                KeyCode::Char(c) => {
                    app.update_input(c);
                }
                KeyCode::Backspace => {
                    app.delete_char();
                }
                KeyCode::Enter => {
                    app.execute_command()?;
                    // Persiste o progresso após cada comando (resiliente a fechamentos abruptos).
                    app.save_progress();
                }
                KeyCode::Up => {
                    app.history_previous();
                }
                KeyCode::Down => {
                    app.history_next();
                }
                KeyCode::Tab => {
                    app.autocomplete();
                }
                KeyCode::Esc => {
                    // Se estiver em modo Help, Stats ou Quests, volta ao Welcome
                    match &app.right_panel_mode {
                        app::RightPanelMode::Help { .. } |
                        app::RightPanelMode::Stats |
                        app::RightPanelMode::Quests |
                        app::RightPanelMode::EasterEgg { .. } |
                        app::RightPanelMode::CommandOutput(_) => {
                            app.right_panel_mode = app::RightPanelMode::Welcome;
                            app.last_output = "Voltando ao modo normal".to_string();
                        }
                        _ => {
                            app.clear_input();
                        }
                    }
                }
                KeyCode::PageUp => {
                    app.scroll = app.scroll.saturating_sub(5);
                }
                KeyCode::PageDown => {
                    app.scroll = app.scroll.saturating_add(5);
                }
                _ => {}
            }
        }
        Event::Tick => {
            // Atualiza informações do sistema se estiver em modo monitor
            if matches!(app.right_panel_mode, app::RightPanelMode::ResourceMonitor { .. }) {
                update_system_monitor(app);
            }
        }
        Event::Resize(_, _) => {
            // O Ratatui lida com resize automaticamente
            // Apenas re-renderiza no próximo loop
        }
        Event::Mouse(mouse_event) => {
            match mouse_event.kind {
                crossterm::event::MouseEventKind::ScrollDown => {
                    app.scroll = app.scroll.saturating_add(1);
                }
                crossterm::event::MouseEventKind::ScrollUp => {
                    app.scroll = app.scroll.saturating_sub(1);
                }
                _ => {}
            }
        }
    }
    
    Ok(())
}

/// Atualiza as informações do monitor de sistema
fn update_system_monitor(app: &mut App) {
    use core::SystemMonitor;
    
    let mut monitor = SystemMonitor::new();
    let summary = monitor.get_system_summary();
    
    app.right_panel_mode = app::RightPanelMode::ResourceMonitor {
        cpu_usage: summary.cpu_usage,
        memory_used: summary.memory_used,
        memory_total: summary.memory_total,
        process_count: summary.process_count,
    };
}
