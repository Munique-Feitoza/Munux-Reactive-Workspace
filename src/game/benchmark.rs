// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

//! Modo benchmark: teste de velocidade de digitação.
//!
//! O usuário recebe uma frase, digita e pressiona Enter; o Munux mede o tempo,
//! calcula WPM e precisão e concede XP proporcional ao desempenho. O estado vive
//! em `App.benchmark` e não é persistido.

use std::time::Instant;

/// Frases sorteadas para o teste (temática terminal/Linux).
pub const PHRASES: &[&str] = &[
    "the quick brown fox jumps over the lazy dog",
    "sudo apt update and sudo apt upgrade",
    "git commit -m fix the login bug",
    "ls -la /home/user/projects",
    "cargo build --release",
    "echo hello world from munux",
    "grep -rn TODO src and fix everything",
    "tar -czf backup.tar.gz documents",
];

/// Estado de um teste de digitação em andamento.
pub struct BenchmarkState {
    pub prompt: String,
    pub started_at: Instant,
}

impl BenchmarkState {
    /// Inicia um novo teste com uma frase pseudo-aleatória.
    pub fn start() -> Self {
        let idx = crate::game::rng::index(PHRASES.len());
        BenchmarkState { prompt: PHRASES[idx].to_string(), started_at: Instant::now() }
    }
}

/// Resultado calculado de um teste.
pub struct BenchmarkResult {
    /// Palavras por minuto (1 palavra = 5 caracteres).
    pub wpm: u32,
    /// Precisão em porcentagem (0–100).
    pub accuracy: u32,
    /// Tempo decorrido em segundos.
    pub seconds: f64,
    /// XP concedido.
    pub xp: u32,
}

/// Calcula o resultado a partir da frase-alvo, do texto digitado e do tempo.
pub fn score(prompt: &str, typed: &str, seconds: f64) -> BenchmarkResult {
    let seconds = seconds.max(0.001);

    let prompt_chars: Vec<char> = prompt.chars().collect();
    let typed_chars: Vec<char> = typed.chars().collect();

    let matches = prompt_chars
        .iter()
        .zip(typed_chars.iter())
        .filter(|(a, b)| a == b)
        .count();
    let denom = prompt_chars.len().max(typed_chars.len()).max(1);
    let accuracy = ((matches as f64 / denom as f64) * 100.0).round() as u32;

    // Conta apenas caracteres corretos para o WPM (penaliza erros).
    let words = matches as f64 / 5.0;
    let wpm = (words / (seconds / 60.0)).round() as u32;

    // XP: proporcional a WPM ponderado pela precisão, com piso e teto.
    let xp = (((wpm as f64) * (accuracy as f64) / 100.0).round() as u32).clamp(5, 150);

    BenchmarkResult { wpm, accuracy, seconds, xp }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn perfect_typing_high_accuracy() {
        let r = score("cargo build --release", "cargo build --release", 3.0);
        assert_eq!(r.accuracy, 100);
        assert!(r.wpm > 0);
        assert!(r.xp >= 5);
    }

    #[test]
    fn wrong_typing_low_accuracy() {
        let r = score("hello world", "xxxxx xxxxx", 2.0);
        assert!(r.accuracy < 30);
    }

    #[test]
    fn empty_typed_is_handled() {
        let r = score("hello", "", 1.0);
        assert_eq!(r.accuracy, 0);
        assert_eq!(r.xp, 5); // piso
    }

    #[test]
    fn xp_is_capped() {
        let r = score("a", "a", 0.0001); // tempo absurdo -> WPM enorme
        assert!(r.xp <= 150);
    }
}
