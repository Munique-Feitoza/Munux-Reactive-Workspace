// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

use ratatui::style::Color;
use crate::game::state::GameState;

/// Cor por nível de saúde (alto = bom): usada para integridade. Verde ≥80,
/// amarelo ≥50, laranja ≥20, vermelho abaixo. Fonte única — antes o HUD (4 faixas)
/// e o painel de stats (3 faixas) divergiam (ex.: integridade=30 dava cores diferentes).
pub fn health_color(pct: u8) -> Color {
    if pct >= 80 {
        Color::Green
    } else if pct >= 50 {
        Color::Yellow
    } else if pct >= 20 {
        Color::LightRed
    } else {
        Color::Red
    }
}

/// Cor por nível de carga (alto = ruim): usada para CPU e RAM. Vermelho acima de
/// 80%, ciano caso contrário.
pub fn load_color(pct: f32) -> Color {
    if pct > 80.0 {
        Color::Red
    } else {
        Color::Cyan
    }
}

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
            
            // Níveis 30-39: Tema Elite (Roxo e vermelho) — alinhado à patente Elite
            30..=39 => Self {
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
    
    /// Retorna cor do background
    pub fn get_background(&self) -> Color {
        self.background
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
            30..=39 => "⬢",
            _ => "⬣",
        }
    }
    
    /// Retorna estilo da borda baseado no nível
    pub fn get_border_type(level: u32) -> ratatui::widgets::BorderType {
        use ratatui::widgets::BorderType;
        match level {
            1..=4 => BorderType::Plain,
            5..=9 => BorderType::Rounded,
            10..=19 => BorderType::Thick,
            20..=29 => BorderType::Double,
            30..=39 => BorderType::Thick, // Repetido pois não temos muitos tipos padrão
            _ => BorderType::Thick,
        }
    }
    
    /// Retorna ASCII art do personagem baseado no nível
    pub fn get_character_art(level: u32, i18n: &crate::i18n::I18n) -> Vec<String> {
        match level {
            // 1-4: Tux Iniciante
            1..=4 => vec![
                "       .--.".to_string(),
                "      |o_o |".to_string(),
                "      |:_/ |".to_string(),
                "     //   \\ \\".to_string(),
                "    (|     | )".to_string(),
                "   /'\\_   _/`\\".to_string(),
                "   \\___)=(___/".to_string(),
            ],
            
            // 5-9: Tux com Terminal
            5..=9 => vec![
                "       .--.".to_string(),
                format!("      |>_< |  {}", i18n.art_tag("terminal")),
                "      |:_/ |".to_string(),
                "     //   \\ \\".to_string(),
                "    (|  █  | )".to_string(),
                "   /'\\_   _/`\\".to_string(),
                "   \\___)=(___/".to_string(),
            ],
            
            // 10-19: Tux Hacker
            10..=19 => vec![
                "       .--.".to_string(),
                format!("      |◉_◉ |  {}", i18n.art_tag("hacker")),
                "      |:_/ |".to_string(),
                "     //▓▓▓\\ \\".to_string(),
                "    (|  █  | )".to_string(),
                "   /'\\_▓▓▓_/`\\".to_string(),
                "   \\___)=(___/".to_string(),
            ],
            
            // 20-29: Tux Cyberpunk
            20..=29 => vec![
                "    ▀▄▀▄.--.".to_string(),
                format!("    ▄▀▄|◉‿◉|  {}", i18n.art_tag("cyberpunk")),
                "      |:≈/ |".to_string(),
                "     //▓▓▓\\ \\".to_string(),
                "    (| ▓█▓ | )".to_string(),
                "   /'\\_▓▓▓_/`\\".to_string(),
                "   \\___)≡(___/".to_string(),
            ],
            
            // 30-39: Tux Elite (alinhado à patente Elite)
            30..=39 => vec![
                "  ▓▓▀▄▀▄.--.".to_string(),
                format!("  ▓▓▄▀▄|◉‿◉|  {}", i18n.art_tag("elite")),
                "  ▓▓  |:≈/ |".to_string(),
                "     //▓█▓\\ \\".to_string(),
                "    (| ▓█▓ | )▓".to_string(),
                "   /'\\_▓█▓_/`\\▓".to_string(),
                "   \\___)≡(___/".to_string(),
            ],
            
            // 50+: Tux Legend
            _ => vec![
                "  ▓▓▓▀▄▀▄.--.".to_string(),
                format!("  ▓▓▓▄▀▄|★‿★|  {}", i18n.art_tag("legend")),
                "  ▓▓▓  |:≈/ |".to_string(),
                "  ▓  //▓█▓\\ \\  ▓".to_string(),
                "    (| ███ | )▓▓".to_string(),
                "   /'\\_███_/`\\▓".to_string(),
                "   \\___)≡(___/".to_string(),
            ],
        }
    }
}

impl GameState {
    /// Retorna o tema atual
    pub fn get_theme(&self) -> Theme {
        Theme::from_level(self.level)
    }
}
