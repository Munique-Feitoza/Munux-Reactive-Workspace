// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

//! Modo tutorial interativo para iniciantes.
//!
//! O tutorial é uma sequência de passos; cada passo pede um comando e só avança
//! quando o usuário executa algo que casa com o esperado. O estado vive em
//! `App.tutorial` (índice do passo atual) e a orientação aparece no output do
//! terminal, então não precisa de um painel reativo dedicado.

/// Regra de correspondência entre o comando do usuário e o passo do tutorial.
pub enum Expect {
    /// A primeira palavra do comando deve ser exatamente esta.
    Command(&'static str),
    /// A primeira palavra deve ser esta E precisa haver pelo menos um argumento.
    CommandWithArg(&'static str),
}

/// Um passo do tutorial.
pub struct Step {
    pub title: &'static str,
    pub instruction: &'static str,
    pub hint: &'static str,
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
    Step {
        title: "1/5 — Pedindo ajuda",
        instruction: "Todo bom terminal tem um comando de ajuda. Digite: help",
        hint: "É só escrever 'help' e apertar Enter.",
        expect: Expect::Command("help"),
    },
    Step {
        title: "2/5 — Onde estou?",
        instruction: "O comando 'pwd' mostra o diretório atual. Experimente: pwd",
        hint: "Digite 'pwd' (print working directory).",
        expect: Expect::Command("pwd"),
    },
    Step {
        title: "3/5 — Listando arquivos",
        instruction: "Use 'ls' para listar os arquivos do diretório atual.",
        hint: "Digite 'ls' e veja o painel da direita reagir.",
        expect: Expect::Command("ls"),
    },
    Step {
        title: "4/5 — Seu progresso",
        instruction: "O Munux acompanha seu XP. Veja com: stats",
        hint: "Digite 'stats' para abrir o painel de estatísticas.",
        expect: Expect::Command("stats"),
    },
    Step {
        title: "5/5 — Lendo um arquivo",
        instruction: "Use 'cat <arquivo>' para ver o conteúdo de um arquivo (ex.: cat README.md).",
        hint: "Comece o comando com 'cat ' seguido de um nome de arquivo.",
        expect: Expect::CommandWithArg("cat"),
    },
];

/// XP de bônus concedido ao concluir o tutorial inteiro.
pub const COMPLETION_XP: u32 = 100;

/// Texto exibido para o passo de índice `idx`.
pub fn step_text(idx: usize) -> String {
    match STEPS.get(idx) {
        Some(step) => format!(
            "🎓 TUTORIAL — {}\n\n{}\n\n💡 Dica: {}\n\n(digite 'tutorial sair' para encerrar)",
            step.title, step.instruction, step.hint
        ),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn step_text_for_each_step_is_nonempty() {
        for i in 0..STEPS.len() {
            assert!(!step_text(i).is_empty());
        }
        assert!(step_text(STEPS.len()).is_empty());
    }
}
