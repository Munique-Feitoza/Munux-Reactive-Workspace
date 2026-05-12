// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

//! Realce de sintaxe leve para o preview de arquivos.
//!
//! Não é um parser completo — faz um scanner por linha que reconhece
//! comentários, strings, números, palavras-chave e identificadores
//! capitalizados (tipos). É suficiente para o preview de ~30 linhas mostrado
//! no painel reativo, sem adicionar dependências pesadas.

use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

/// Cor de cada categoria de token.
const COMMENT: Color = Color::DarkGray;
const STRING: Color = Color::Green;
const NUMBER: Color = Color::Yellow;
const KEYWORD: Color = Color::Magenta;
const TYPE: Color = Color::Cyan;
const DEFAULT: Color = Color::White;

/// Retorna `true` se a linguagem tem regras de realce dedicadas.
pub fn is_supported(language: &str) -> bool {
    matches!(
        language,
        "rust" | "python" | "javascript" | "typescript" | "bash" | "json" | "toml"
    )
}

/// Converte o conteúdo do arquivo em linhas com realce.
pub fn highlight(content: &str, language: &str) -> Vec<Line<'static>> {
    content.lines().map(|line| highlight_line(line, language)).collect()
}

/// Prefixo de comentário de linha para a linguagem (se houver).
fn line_comment(language: &str) -> Option<&'static str> {
    match language {
        "rust" | "javascript" | "typescript" => Some("//"),
        "python" | "bash" | "toml" => Some("#"),
        _ => None,
    }
}

/// Conjunto de palavras-chave da linguagem.
fn keywords(language: &str) -> &'static [&'static str] {
    match language {
        "rust" => &[
            "fn", "let", "mut", "const", "static", "struct", "enum", "impl", "trait",
            "pub", "use", "mod", "match", "if", "else", "for", "while", "loop",
            "return", "self", "Self", "where", "as", "dyn", "ref", "move", "async",
            "await", "unsafe", "crate", "super", "in", "break", "continue", "type",
        ],
        "python" => &[
            "def", "class", "import", "from", "as", "if", "elif", "else", "for",
            "while", "return", "yield", "lambda", "with", "try", "except", "finally",
            "raise", "pass", "break", "continue", "in", "is", "not", "and", "or",
            "None", "True", "False", "global", "nonlocal", "del", "assert", "await",
            "async",
        ],
        "javascript" | "typescript" => &[
            "function", "var", "let", "const", "if", "else", "for", "while", "return",
            "class", "extends", "new", "this", "import", "export", "default", "async",
            "await", "try", "catch", "finally", "throw", "typeof", "instanceof", "in",
            "of", "break", "continue", "switch", "case", "null", "undefined", "true",
            "false", "interface", "type", "enum",
        ],
        "bash" => &[
            "if", "then", "else", "elif", "fi", "for", "while", "until", "do", "done",
            "case", "esac", "function", "in", "echo", "export", "local", "return",
            "source", "alias", "unset", "read",
        ],
        "json" | "toml" => &["true", "false", "null"],
        _ => &[],
    }
}

fn span(text: &str, color: Color) -> Span<'static> {
    Span::styled(text.to_string(), Style::default().fg(color))
}

/// Realça uma única linha de código.
fn highlight_line(line: &str, language: &str) -> Line<'static> {
    let kws = keywords(language);
    let comment_marker = line_comment(language);

    let chars: Vec<char> = line.chars().collect();
    let mut spans: Vec<Span<'static>> = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        // Comentário de linha: consome o resto.
        if let Some(marker) = comment_marker {
            if starts_with_at(&chars, i, marker) {
                let rest: String = chars[i..].iter().collect();
                spans.push(span(&rest, COMMENT));
                break;
            }
        }

        // String entre aspas (simples ou duplas), com escape básico.
        if c == '"' || c == '\'' {
            let quote = c;
            let start = i;
            i += 1;
            while i < chars.len() {
                if chars[i] == '\\' && i + 1 < chars.len() {
                    i += 2;
                    continue;
                }
                if chars[i] == quote {
                    i += 1;
                    break;
                }
                i += 1;
            }
            let text: String = chars[start..i].iter().collect();
            spans.push(span(&text, STRING));
            continue;
        }

        // Identificador / palavra-chave / número.
        if c.is_alphanumeric() || c == '_' {
            let start = i;
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let word: String = chars[start..i].iter().collect();
            let color = if kws.contains(&word.as_str()) {
                KEYWORD
            } else if word.chars().next().map(|ch| ch.is_ascii_digit()).unwrap_or(false) {
                NUMBER
            } else if word.chars().next().map(|ch| ch.is_uppercase()).unwrap_or(false) {
                TYPE
            } else {
                DEFAULT
            };
            spans.push(span(&word, color));
            continue;
        }

        // Qualquer outro caractere: agrupa run de pontuação/espaço como default.
        let start = i;
        while i < chars.len() {
            let ch = chars[i];
            if ch.is_alphanumeric() || ch == '_' || ch == '"' || ch == '\'' {
                break;
            }
            if let Some(marker) = comment_marker {
                if starts_with_at(&chars, i, marker) {
                    break;
                }
            }
            i += 1;
        }
        let text: String = chars[start..i].iter().collect();
        spans.push(span(&text, DEFAULT));
    }

    if spans.is_empty() {
        // Linha vazia: span vazio para preservar a altura.
        spans.push(Span::raw(String::new()));
    }

    Line::from(spans)
}

/// `true` se `chars[at..]` começa com a sequência `pat`.
fn starts_with_at(chars: &[char], at: usize, pat: &str) -> bool {
    let pat: Vec<char> = pat.chars().collect();
    if at + pat.len() > chars.len() {
        return false;
    }
    chars[at..at + pat.len()] == pat[..]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rust_keyword_and_string() {
        let line = highlight_line("    let name = \"munux\"; // ok", "rust");
        // Esperado: indent, "let", " name = ", string, "; ", comentário
        let texts: Vec<String> = line.spans.iter().map(|s| s.content.to_string()).collect();
        assert!(texts.iter().any(|t| t == "let"));
        assert!(texts.iter().any(|t| t.contains("munux")));
        assert!(texts.iter().any(|t| t.contains("// ok")));
    }

    #[test]
    fn python_comment_whole_line() {
        let line = highlight_line("# comentário", "python");
        assert_eq!(line.spans.len(), 1);
        assert_eq!(line.spans[0].content, "# comentário");
    }

    #[test]
    fn empty_line_has_span() {
        let line = highlight_line("", "rust");
        assert_eq!(line.spans.len(), 1);
    }

    #[test]
    fn unsupported_language() {
        assert!(!is_supported("markdown"));
        assert!(is_supported("rust"));
    }
}
