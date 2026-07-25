// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

//! Sorteio pseudo-aleatório derivado do relógio, sem dependências externas.
//!
//! Fonte única: antes existiam duas implementações do mesmo conceito —
//! `easter_eggs::clock_index` (segundos) e `benchmark::pseudo_index`
//! (nanossegundos). A de segundos devolvia o mesmo índice para tudo que
//! acontecesse dentro do mesmo segundo; a granularidade de nanossegundos venceu.
//!
//! Não serve para nada relacionado a segurança — é só para escolher uma frase ou
//! uma citação.

use std::time::{SystemTime, UNIX_EPOCH};

/// Índice pseudo-aleatório em `[0, len)`. Devolve 0 quando `len == 0`.
pub fn index(len: usize) -> usize {
    if len == 0 {
        return 0;
    }

    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default();
    // Mistura segundos e nanossegundos: os nanos dão granularidade dentro do
    // mesmo segundo, os segundos evitam repetição quando a resolução do relógio
    // é grosseira.
    let seed = now.as_secs() as usize ^ (now.subsec_nanos() as usize).rotate_left(11);
    seed % len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_stays_in_range() {
        for len in [1usize, 2, 8, 9, 100] {
            for _ in 0..50 {
                assert!(index(len) < len, "índice fora de [0, {})", len);
            }
        }
    }

    #[test]
    fn empty_length_is_safe() {
        assert_eq!(index(0), 0);
    }
}
