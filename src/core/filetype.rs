// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

//! Fonte única do mapeamento extensão → {linguagem, ícone, cor}.
//!
//! Antes, esse mapa vivia (divergente) em `parser::detect_language`,
//! `filesystem::FileEntry::get_icon` e na coloração de `ui::reactive`. A
//! divergência causava bugs reais — `.ts` realçado como JavaScript e ícones/cores
//! inconsistentes. Centralizar aqui mantém preview, ícone e cor sempre em sincronia.

use ratatui::style::Color;

/// Atributos visuais e de realce de um tipo de arquivo.
pub struct FileType {
    /// Identificador de linguagem para `ui::highlight` (ex.: "rust", "typescript").
    pub language: &'static str,
    /// Ícone exibido no painel de arquivos.
    pub icon: &'static str,
    /// Cor do nome no painel de arquivos.
    pub color: Color,
}

const TEXT: FileType = FileType { language: "text", icon: "📄", color: Color::White };
const HIDDEN: FileType = FileType { language: "text", icon: "👁️", color: Color::White };

/// Classifica um arquivo pelo nome. A extensão é comparada em minúsculas, então
/// `Main.RS` e `main.rs` recebem o mesmo tratamento.
pub fn classify(name: &str) -> FileType {
    let ext = std::path::Path::new(name)
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase())
        .unwrap_or_default();

    match ext.as_str() {
        "rs" => FileType { language: "rust", icon: "🦀", color: Color::LightRed },
        "py" => FileType { language: "python", icon: "🐍", color: Color::Yellow },
        "js" => FileType { language: "javascript", icon: "📜", color: Color::Yellow },
        "ts" => FileType { language: "typescript", icon: "📜", color: Color::Blue },
        "sh" => FileType { language: "bash", icon: "📜", color: Color::Green },
        "toml" => FileType { language: "toml", icon: "⚙️", color: Color::Blue },
        "json" => FileType { language: "json", icon: "⚙️", color: Color::Blue },
        "md" | "markdown" => FileType { language: "markdown", icon: "📝", color: Color::Cyan },
        // Sem extensão reconhecida: arquivos ocultos ganham ícone próprio.
        _ if name.starts_with('.') => HIDDEN,
        _ => TEXT,
    }
}
