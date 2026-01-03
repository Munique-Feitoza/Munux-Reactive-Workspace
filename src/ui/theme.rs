// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

use ratatui::style::Color;
use crate::game::state::GameState;

/// Tema visual progressivo baseado no nível
#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub success: Color,
    pub danger: Color,
    pub warning: Color,
    pub background: Color,
    pub border: Color,
    pub text: Color,
}

impl Theme {
    /// Retorna o tema baseado no nível do jogador
    pub fn from_level(level: u32) -> Self {
        match level {
            // Níveis 1-4: Tema Iniciante (Azul claro e branco)
            1..=4 => Self {
                primary: Color::Cyan,
                secondary: Color::Blue,
                accent: Color::LightBlue,
                success: Color::Green,
                danger: Color::Red,
                warning: Color::Yellow,
                background: Color::Black,
                border: Color::Cyan,
                text: Color::White,
            },
            
            // Níveis 5-9: Tema Terminal (Verde Matrix)
            5..=9 => Self {
                primary: Color::Green,
                secondary: Color::Rgb(0, 200, 0),
                accent: Color::Rgb(0, 255, 100),
                success: Color::Rgb(0, 255, 0),
                danger: Color::Red,
                warning: Color::Yellow,
                background: Color::Black,
                border: Color::Green,
                text: Color::Rgb(0, 255, 0),
            },
            
            // Níveis 10-19: Tema Hacker (Verde escuro e ciano)
            10..=19 => Self {
                primary: Color::Rgb(0, 255, 128),
                secondary: Color::Rgb(0, 200, 100),
                accent: Color::Cyan,
                success: Color::Rgb(0, 255, 128),
                danger: Color::Rgb(255, 0, 100),
                warning: Color::Rgb(255, 200, 0),
                background: Color::Black,
                border: Color::Rgb(0, 255, 128),
                text: Color::Rgb(200, 255, 200),
            },
            
            // Níveis 20-29: Tema Cyberpunk (Magenta e ciano)
            20..=29 => Self {
                primary: Color::Magenta,
                secondary: Color::Cyan,
                accent: Color::Rgb(255, 0, 255),
                success: Color::Rgb(0, 255, 255),
                danger: Color::Rgb(255, 0, 100),
                warning: Color::Rgb(255, 128, 0),
                background: Color::Black,
                border: Color::Magenta,
                text: Color::Rgb(255, 128, 255),
            },
            
            // Níveis 30-49: Tema Elite (Roxo e vermelho)
            30..=49 => Self {
                primary: Color::Rgb(128, 0, 255),
                secondary: Color::Rgb(255, 0, 128),
                accent: Color::Rgb(200, 0, 255),
                success: Color::Rgb(128, 255, 0),
                danger: Color::Rgb(255, 0, 0),
                warning: Color::Rgb(255, 128, 0),
                background: Color::Black,
                border: Color::Rgb(128, 0, 255),
                text: Color::Rgb(200, 100, 255),
            },
            
            // Níveis 50+: Tema Legend (Arco-íris escuro)
            _ => Self {
                primary: Color::Rgb(255, 0, 255),
                secondary: Color::Rgb(0, 255, 255),
                accent: Color::Rgb(255, 255, 0),
                success: Color::Rgb(0, 255, 128),
                danger: Color::Rgb(255, 50, 50),
                warning: Color::Rgb(255, 200, 0),
                background: Color::Black,
                border: Color::Rgb(255, 0, 255),
                text: Color::Rgb(255, 255, 255),
            },
        }
    }
    
    /// Retorna a cor do cursor baseada no tema
    pub fn get_cursor_color(&self) -> Color {
        self.accent
    }
    
    /// Retorna símbolo especial baseado no nível
    pub fn get_prompt_symbol(level: u32) -> &'static str {
        match level {
            1..=4 => "➜",
            5..=9 => "►",
            10..=19 => "▶",
            20..=29 => "◆",
            30..=49 => "⬢",
            _ => "⬣",
        }
    }
    
    /// Retorna efeito visual do border baseado no nível
    pub fn get_border_style(level: u32) -> &'static str {
        match level {
            1..=4 => "─",     // Simples
            5..=9 => "═",     // Duplo
            10..=19 => "━",   // Grosso
            20..=29 => "┉",   // Tracejado
            30..=49 => "╍",   // Pontilhado grosso
            _ => "▬",         // Elite
        }
    }
    
    /// Retorna ASCII art do personagem baseado no nível
    pub fn get_character_art(level: u32) -> Vec<&'static str> {
        match level {
            // 1-4: Tux Iniciante
            1..=4 => vec![
                "       .--.",
                "      |o_o |",
                "      |:_/ |",
                "     //   \\ \\",
                "    (|     | )",
                "   /'\\_   _/`\\",
                "   \\___)=(___/",
            ],
            
            // 5-9: Tux com Terminal
            5..=9 => vec![
                "       .--.",
                "      |>_< |  [TERMINAL MODE]",
                "      |:_/ |",
                "     //   \\ \\",
                "    (|  █  | )",
                "   /'\\_   _/`\\",
                "   \\___)=(___/",
            ],
            
            // 10-19: Tux Hacker
            10..=19 => vec![
                "       .--.",
                "      |◉_◉ |  [HACKER MODE]",
                "      |:_/ |",
                "     //▓▓▓\\ \\",
                "    (|  █  | )",
                "   /'\\_▓▓▓_/`\\",
                "   \\___)=(___/",
            ],
            
            // 20-29: Tux Cyberpunk
            20..=29 => vec![
                "    ▀▄▀▄.--.",
                "    ▄▀▄|◉‿◉|  [CYBERPUNK]",
                "      |:≈/ |",
                "     //▓▓▓\\ \\",
                "    (| ▓█▓ | )",
                "   /'\\_▓▓▓_/`\\",
                "   \\___)≡(___/",
            ],
            
            // 30-49: Tux Elite
            30..=49 => vec![
                "  ▓▓▀▄▀▄.--.",
                "  ▓▓▄▀▄|◉‿◉|  [ELITE HACKER]",
                "  ▓▓  |:≈/ |",
                "     //▓█▓\\ \\",
                "    (| ▓█▓ | )▓",
                "   /'\\_▓█▓_/`\\▓",
                "   \\___)≡(___/",
            ],
            
            // 50+: Tux Legend
            _ => vec![
                "  ▓▓▓▀▄▀▄.--.",
                "  ▓▓▓▄▀▄|★‿★|  [LEGEND]",
                "  ▓▓▓  |:≈/ |",
                "  ▓  //▓█▓\\ \\  ▓",
                "    (| ███ | )▓▓",
                "   /'\\_███_/`\\▓",
                "   \\___)≡(___/",
            ],
        }
    }
    
    /// Retorna mensagem motivacional baseada no nível
    pub fn get_level_message(level: u32) -> &'static str {
        match level {
            1 => "Bem-vindo ao terminal!",
            2..=4 => "Você está aprendendo!",
            5 => "Modo de segurança DESATIVADO!",
            6..=9 => "Comandos perigosos liberados",
            10 => "Você é um hacker agora!",
            11..=19 => "O sistema é seu playground",
            20 => "Entrando no Cyberpunk...",
            21..=29 => "Você domina o terminal",
            30 => "Elite mode ACTIVATED!",
            31..=49 => "Poucos chegam aqui...",
            50 => "VOCÊ É UMA LENDA!",
            _ => "All your base are belong to us",
        }
    }
}

impl GameState {
    /// Retorna o tema atual
    pub fn get_theme(&self) -> Theme {
        Theme::from_level(self.level)
    }
}
