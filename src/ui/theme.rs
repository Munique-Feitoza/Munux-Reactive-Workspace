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

/// Estágio visual do terminal, derivado do nível.
///
/// Refina [`crate::game::tier::Tier`]: a **patente** marca a progressão
/// narrativa (5 degraus) e o **estágio** marca a progressão visual (6), porque o
/// nível 5 — onde o modo seguro se desliga — merece um visual próprio ainda
/// dentro da patente Novato.
///
/// Fonte única dos cortes visuais. Antes as mesmas seis faixas estavam copiadas
/// em `from_level`, `get_prompt_symbol`, `get_border_type` e
/// `get_character_art`, e uma sétima cópia divergente vivia em
/// `i18n::level_commands` (que cortava em 10 em vez de 9). O teste
/// `stage_refines_tier` garante que estágio e patente nunca voltem a divergir.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Stage {
    /// 1–4 — primeiros passos, modo seguro ligado.
    Beginner,
    /// 5–9 — modo seguro desligado.
    Terminal,
    /// 10–19
    Hacker,
    /// 20–29
    Cyberpunk,
    /// 30–39
    Elite,
    /// 40+
    Legend,
}

impl Stage {
    /// Estágio correspondente a um nível.
    pub fn from_level(level: u32) -> Self {
        match level {
            0..=4 => Stage::Beginner,
            5..=9 => Stage::Terminal,
            10..=19 => Stage::Hacker,
            20..=29 => Stage::Cyberpunk,
            30..=39 => Stage::Elite,
            _ => Stage::Legend,
        }
    }

    /// Sufixo da chave Fluent da etiqueta na arte (`game-art-{}-tag`).
    /// `None` no estágio inicial, que não exibe etiqueta.
    fn art_tag(self) -> Option<&'static str> {
        match self {
            Stage::Beginner => None,
            Stage::Terminal => Some("terminal"),
            Stage::Hacker => Some("hacker"),
            Stage::Cyberpunk => Some("cyberpunk"),
            Stage::Elite => Some("elite"),
            Stage::Legend => Some("legend"),
        }
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
        match Stage::from_level(level) {
            // Tema Iniciante (Azul claro e branco)
            Stage::Beginner => Self {
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
            
            // Tema Terminal (Verde Matrix)
            Stage::Terminal => Self {
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
            
            // Tema Hacker (Verde escuro e ciano)
            Stage::Hacker => Self {
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
            
            // Tema Cyberpunk (Magenta e ciano)
            Stage::Cyberpunk => Self {
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
            
            // Tema Elite (Roxo e vermelho) — alinhado à patente Elite
            Stage::Elite => Self {
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
            
            // Tema Legend (Arco-íris escuro) — a partir do nível 40
            Stage::Legend => Self {
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
        match Stage::from_level(level) {
            Stage::Beginner => "➜",
            Stage::Terminal => "►",
            Stage::Hacker => "▶",
            Stage::Cyberpunk => "◆",
            Stage::Elite => "⬢",
            Stage::Legend => "⬣",
        }
    }
    
    /// Retorna estilo da borda baseado no nível.
    ///
    /// O `ratatui` só oferece quatro tipos de borda, então os três estágios mais
    /// altos compartilham `Thick` — agora de forma explícita, e não como três
    /// ramos separados que por acaso devolviam o mesmo valor.
    pub fn get_border_type(level: u32) -> ratatui::widgets::BorderType {
        use ratatui::widgets::BorderType;
        match Stage::from_level(level) {
            Stage::Beginner => BorderType::Plain,
            Stage::Terminal => BorderType::Rounded,
            Stage::Cyberpunk => BorderType::Double,
            Stage::Hacker | Stage::Elite | Stage::Legend => BorderType::Thick,
        }
    }
    
    /// Retorna ASCII art do personagem baseado no nível
    pub fn get_character_art(level: u32, i18n: &crate::i18n::I18n) -> Vec<String> {
        let stage = Stage::from_level(level);
        // A etiqueta ao lado da arte vem do próprio estágio (fonte única).
        let tag = stage.art_tag().map(|m| i18n.art_tag(m)).unwrap_or_default();

        match stage {
            // Tux Iniciante
            Stage::Beginner => vec![
                "       .--.".to_string(),
                "      |o_o |".to_string(),
                "      |:_/ |".to_string(),
                "     //   \\ \\".to_string(),
                "    (|     | )".to_string(),
                "   /'\\_   _/`\\".to_string(),
                "   \\___)=(___/".to_string(),
            ],
            
            // Tux com Terminal
            Stage::Terminal => vec![
                "       .--.".to_string(),
                format!("      |>_< |  {}", tag),
                "      |:_/ |".to_string(),
                "     //   \\ \\".to_string(),
                "    (|  █  | )".to_string(),
                "   /'\\_   _/`\\".to_string(),
                "   \\___)=(___/".to_string(),
            ],
            
            // Tux Hacker
            Stage::Hacker => vec![
                "       .--.".to_string(),
                format!("      |◉_◉ |  {}", tag),
                "      |:_/ |".to_string(),
                "     //▓▓▓\\ \\".to_string(),
                "    (|  █  | )".to_string(),
                "   /'\\_▓▓▓_/`\\".to_string(),
                "   \\___)=(___/".to_string(),
            ],
            
            // Tux Cyberpunk
            Stage::Cyberpunk => vec![
                "    ▀▄▀▄.--.".to_string(),
                format!("    ▄▀▄|◉‿◉|  {}", tag),
                "      |:≈/ |".to_string(),
                "     //▓▓▓\\ \\".to_string(),
                "    (| ▓█▓ | )".to_string(),
                "   /'\\_▓▓▓_/`\\".to_string(),
                "   \\___)≡(___/".to_string(),
            ],
            
            // Tux Elite (alinhado à patente Elite)
            Stage::Elite => vec![
                "  ▓▓▀▄▀▄.--.".to_string(),
                format!("  ▓▓▄▀▄|◉‿◉|  {}", tag),
                "  ▓▓  |:≈/ |".to_string(),
                "     //▓█▓\\ \\".to_string(),
                "    (| ▓█▓ | )▓".to_string(),
                "   /'\\_▓█▓_/`\\▓".to_string(),
                "   \\___)≡(___/".to_string(),
            ],
            
            // Tux Legend — a partir do nível 40
            Stage::Legend => vec![
                "  ▓▓▓▀▄▀▄.--.".to_string(),
                format!("  ▓▓▓▄▀▄|★‿★|  {}", tag),
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
