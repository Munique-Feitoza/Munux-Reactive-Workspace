// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

//! Modo tutorial interativo para iniciantes.
//!
//! O tutorial é uma sequência de passos; cada passo pede um comando e só avança
//! quando o usuário executa algo que casa com o esperado. O estado vive em
//! `App.tutorial` (índice do passo atual) e a orientação aparece no output do
//! terminal, então não precisa de um painel reativo dedicado.
//!
//! Os textos vivem nos `.ftl`: o passo guarda só o **sufixo** da chave e resolve
//! `tutorial-<sufixo>-title|instruction|hint`. Antes o roteiro inteiro estava em
//! português fixo dentro deste arquivo, num app que se anuncia bilíngue.

use crate::i18n::I18n;

/// Regra de correspondência entre o comando do usuário e o passo do tutorial.
pub enum Expect {
    /// A primeira palavra do comando deve ser exatamente esta.
    Command(&'static str),
    /// A primeira palavra deve ser esta E precisa haver pelo menos um argumento.
    CommandWithArg(&'static str),
}

/// Um passo do tutorial.
pub struct Step {
    /// Sufixo das chaves Fluent (`tutorial-{key}-title`, `-instruction`, `-hint`).
    pub key: &'static str,
    pub expect: Expect,
}

impl Step {
    /// Retorna `true` se `command` satisfaz este passo.
    pub fn matches(&self, command: &str) -> bool {
        let mut words = command.split_whitespace();
        let first = words.next();
        match self.expect {
            Expect::Command(c) => first == Some(c),
            Expect::CommandWithArg(c) => first == Some(c) && words.next().is_some(),
        }
    }
}

/// Sequência de passos do tutorial.
pub const STEPS: &[Step] = &[
    Step { key: "help", expect: Expect::Command("help") },
    Step { key: "pwd", expect: Expect::Command("pwd") },
    Step { key: "ls", expect: Expect::Command("ls") },
    Step { key: "stats", expect: Expect::Command("stats") },
    Step { key: "cat", expect: Expect::CommandWithArg("cat") },
];

/// XP de bônus concedido ao concluir o tutorial inteiro.
pub const COMPLETION_XP: u32 = 100;

/// Texto exibido para o passo de índice `idx`. Vazio se o índice não existe.
///
/// A montagem em várias linhas acontece aqui, em Rust: no Fluent, valores
/// multilinha exigem indentação de continuação e quebram com linhas em branco
/// no meio. Cada chave do `.ftl` é, portanto, uma linha só.
pub fn step_text(idx: usize, i18n: &I18n) -> String {
    use fluent::{FluentArgs, FluentValue};

    let Some(step) = STEPS.get(idx) else {
        return String::new();
    };

    // A numeração ("2/5") vem do tamanho real de STEPS, então acrescentar um
    // passo não exige reescrever cinco títulos.
    let mut header_args = FluentArgs::new();
    header_args.set("step", FluentValue::from(idx as u32 + 1));
    header_args.set("total", FluentValue::from(STEPS.len() as u32));
    header_args.set("title", i18n.tc(&format!("tutorial-{}-title", step.key)));

    format!(
        "{}\n\n{}\n\n{}\n\n{}",
        i18n.t("tutorial-header", Some(&header_args)),
        i18n.tc(&format!("tutorial-{}-instruction", step.key)),
        i18n.t1("tutorial-hint", "hint", i18n.tc(&format!("tutorial-{}-hint", step.key))),
        i18n.tc("tutorial-exit-note"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::Language;

    #[test]
    fn command_step_matches_first_word() {
        let step = &STEPS[0]; // espera "help"
        assert!(step.matches("help"));
        assert!(step.matches("help arch"));
        assert!(!step.matches("ls"));
    }

    #[test]
    fn command_with_arg_step() {
        let step = STEPS.last().unwrap(); // espera "cat <arquivo>"
        assert!(step.matches("cat README.md"));
        assert!(!step.matches("cat")); // sem argumento não conta
        assert!(!step.matches("concat foo")); // não confunde com substring
    }

    /// Todo passo precisa das três chaves nos dois locales — senão o roteiro
    /// chegaria ao usuário como `[MISSING: ...]`.
    #[test]
    fn step_text_resolves_in_both_locales() {
        for lang in [Language::PtBr, Language::EnUs] {
            let i18n = I18n::new(lang);
            for i in 0..STEPS.len() {
                let text = step_text(i, &i18n);
                assert!(!text.is_empty(), "passo {} vazio em {:?}", i, lang);
                assert!(
                    !text.contains("[MISSING"),
                    "passo {} tem chave ausente em {:?}: {}",
                    i,
                    lang,
                    text
                );
            }
            assert!(step_text(STEPS.len(), &i18n).is_empty());
        }
    }
}
