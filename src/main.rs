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
    
    // Inicializa o terminal via RAII guard: a restauração acontece no `Drop`,
    // cobrindo retorno normal, early-return via `?` e panic.
    let mut guard = tui::TerminalGuard::new()?;

    // Cria a aplicação (se falhar aqui, o `guard` restaura o terminal no Drop).
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
    let result = run(guard.terminal(), &mut app, event_handler);

    // Salva o progresso ao sair (o terminal é restaurado quando `guard` sai de escopo).
    app.save_progress();

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
                        // Limpa a tela (Ctrl+L) — mesmo conceito do comando `clear`.
                        app.clear_screen();
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
                    // Com a árvore visível e sem nada digitado, Enter abre o item
                    // selecionado (navegação); caso contrário executa o comando.
                    if app.is_browsing_files() {
                        app.open_selected_entry();
                    } else {
                        app.execute_command()?;
                        // Persiste o progresso após cada comando (resiliente a fechamentos abruptos).
                        app.save_progress();
                    }
                }
                KeyCode::Up => {
                    if app.is_browsing_files() {
                        app.move_file_selection(-1);
                    } else {
                        app.history_previous();
                    }
                }
                KeyCode::Down => {
                    if app.is_browsing_files() {
                        app.move_file_selection(1);
                    } else {
                        app.history_next();
                    }
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
                            app.last_output = app.i18n.tc("ui-back-to-normal");
                        }
                        _ => {
                            app.clear_input();
                        }
                    }
                }
                KeyCode::PageUp => app.scroll_by(-5),
                KeyCode::PageDown => app.scroll_by(5),
                _ => {}
            }
        }
        Event::Tick => {
            // Atualiza o monitor persistente quando um painel que mostra métricas
            // está ativo (recursos ou estatísticas).
            if matches!(
                app.right_panel_mode,
                app::RightPanelMode::ResourceMonitor { .. } | app::RightPanelMode::Stats
            ) {
                app.refresh_monitor();
            }

            // Com a árvore visível, relê o diretório a 1 Hz para refletir
            // mudanças feitas por fora (outro terminal, um script). Antes isso
            // acontecia a cada frame — ou seja, a cada tecla digitada.
            if matches!(app.right_panel_mode, app::RightPanelMode::FileTree { .. }) {
                app.refresh_dir_cache();
            }
        }
        Event::Resize => {
            // O Ratatui lida com resize automaticamente
            // Apenas re-renderiza no próximo loop
        }
        Event::Mouse(mouse_event) => {
            use crossterm::event::MouseEventKind;
            match mouse_event.kind {
                MouseEventKind::ScrollDown => app.scroll_by(1),
                MouseEventKind::ScrollUp => app.scroll_by(-1),
                _ => {}
            }
        }
    }
    
    Ok(())
}
