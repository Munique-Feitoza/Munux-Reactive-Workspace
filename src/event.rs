// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, MouseEvent};
use anyhow::Result;
use std::time::Duration;

/// Tipos de eventos que a aplicação pode receber
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Event {
    /// Evento de tecla pressionada
    Key(KeyEvent),
    
    /// Evento de mouse
    Mouse(MouseEvent),
    
    /// Evento de redimensionamento de terminal
    Resize(u16, u16),
    
    /// Tick de atualização (para animações e polling)
    Tick,
}

/// Gerenciador de eventos
pub struct EventHandler {
    /// Intervalo entre ticks (em milissegundos)
    tick_rate: Duration,
}

impl EventHandler {
    /// Cria um novo gerenciador de eventos
    pub fn new(tick_rate_ms: u64) -> Self {
        Self {
            tick_rate: Duration::from_millis(tick_rate_ms),
        }
    }
    
    /// Aguarda e retorna o próximo evento
    pub fn next(&self) -> Result<Event> {
        // Polling com timeout
        if event::poll(self.tick_rate)? {
            match event::read()? {
                CrosstermEvent::Key(key) => Ok(Event::Key(key)),
                CrosstermEvent::Mouse(mouse) => Ok(Event::Mouse(mouse)),
                CrosstermEvent::Resize(width, height) => Ok(Event::Resize(width, height)),
                _ => Ok(Event::Tick),
            }
        } else {
            // Timeout expirou, retorna Tick
            Ok(Event::Tick)
        }
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new(1000) // 1000ms = 1 segundo entre ticks (mais lento, mais estável)
    }
}
