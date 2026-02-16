// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use std::io::{self, Stdout};

/// Tipo de terminal usado pela aplicação
pub type Tui = Terminal<CrosstermBackend<Stdout>>;

/// Inicializa o terminal em modo TUI
pub fn init() -> Result<Tui> {
    // Habilita raw mode (desabilita echo e line buffering)
    enable_raw_mode()?;
    
    // Entra no alternate screen (não polui o histórico do terminal)
    execute!(io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    
    // Cria o backend e o terminal
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    
    // Limpa a tela inicial
    terminal.clear()?;
    
    Ok(terminal)
}

/// Restaura o terminal ao estado normal
pub fn restore() -> Result<()> {
    // Desabilita raw mode
    disable_raw_mode()?;
    
    // Sai do alternate screen
    execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    
    Ok(())
}

/// RAII guard que garante que o terminal será restaurado
#[allow(dead_code)]
pub struct TerminalGuard;

impl TerminalGuard {
    #[allow(dead_code)]
    pub fn new() -> Result<Self> {
        init()?;
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        if let Err(e) = restore() {
            eprintln!("Erro ao restaurar terminal: {}", e);
        }
    }
}
