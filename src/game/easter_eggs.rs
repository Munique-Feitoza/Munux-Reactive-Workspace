// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

//! Easter eggs do terminal.
//!
//! O reconhecimento do comando acontece uma única vez em [`EasterEggs::classify`];
//! a partir do [`Egg`] resultante, [`EasterEggs::render`] produz a arte/texto e
//! [`EasterEggs::achievement`] diz qual conquista (se houver) ele concede. Isso
//! evita duplicar a lista de comandos entre a saída visual e o sistema de
//! conquistas.

use std::time::{SystemTime, UNIX_EPOCH};

/// Quantos easter eggs distintos é preciso encontrar para virar "caçador".
pub const HUNTER_THRESHOLD: usize = 5;

/// Cada easter egg reconhecido.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Egg {
    Train,
    Cowsay,
    Fortune,
    Matrix,
    SudoSu,
    Nuke,
    Whoami,
    HackThePlanet,
    Konami,
    Sandwich,
    SandwichSudo,
    Answer42,
    Xyzzy,
    Cake,
    VimEscape,
    StarWars,
}

/// Easter eggs do terminal.
pub struct EasterEggs;

impl EasterEggs {
    /// Reconhece o easter egg de um comando, se houver.
    pub fn classify(command: &str) -> Option<Egg> {
        let cmd = command.trim().to_lowercase();

        let egg = match cmd.as_str() {
            "sl" => Egg::Train,
            "fortune" => Egg::Fortune,
            "matrix" | "hack" => Egg::Matrix,
            "sudo su" => Egg::SudoSu,
            "whoami" => Egg::Whoami,
            "konami" | "↑↑↓↓←→←→ba" => Egg::Konami,
            "xyzzy" => Egg::Xyzzy,
            "42" | "the answer to life the universe and everything" => Egg::Answer42,
            "the cake is a lie" => Egg::Cake,
            ":q" | ":q!" | ":wq" | ":wq!" | ":x" | ":x!" => Egg::VimEscape,
            "star wars" | "telnet towel.blinkenlights.nl" => Egg::StarWars,
            "sudo make me a sandwich" => Egg::SandwichSudo,
            "make me a sandwich" => Egg::Sandwich,
            _ => {
                if cmd.contains("cowsay") {
                    Egg::Cowsay
                } else if cmd.contains("rm -rf /") {
                    Egg::Nuke
                } else if cmd.contains("hack the planet") {
                    Egg::HackThePlanet
                } else {
                    return None;
                }
            }
        };
        Some(egg)
    }

    /// Saída visual de um easter egg. Mantida como API de compatibilidade.
    pub fn check(command: &str) -> Option<String> {
        Self::classify(command).map(|egg| Self::render(egg, command))
    }

    /// `(id da conquista, XP)` para os easter eggs que concedem conquista.
    /// Retorna `None` para os puramente decorativos.
    pub fn achievement(egg: Egg) -> Option<(&'static str, u32)> {
        let pair = match egg {
            Egg::Train => ("easter_egg_train", 25),
            Egg::Cowsay => ("easter_egg_cow", 30),
            Egg::Nuke => ("easter_egg_nuke", 666),
            Egg::Matrix => ("easter_egg_matrix", 40),
            Egg::Konami => ("easter_egg_konami", 100),
            Egg::Sandwich | Egg::SandwichSudo => ("easter_egg_sandwich", 42),
            Egg::Answer42 => ("easter_egg_42", 42),
            Egg::Xyzzy => ("easter_egg_xyzzy", 30),
            Egg::Cake => ("easter_egg_cake", 30),
            Egg::VimEscape => ("easter_egg_vim", 20),
            Egg::StarWars => ("easter_egg_starwars", 50),
            Egg::Fortune | Egg::SudoSu | Egg::Whoami | Egg::HackThePlanet => return None,
        };
        Some(pair)
    }

    /// Renderiza a arte/texto do easter egg.
    fn render(egg: Egg, command: &str) -> String {
        match egg {
            Egg::Train => Self::train_animation(),
            Egg::Cowsay => {
                let message = command.to_lowercase().replace("cowsay", "").trim().to_string();
                Self::cowsay(if message.is_empty() {
                    "Moo! Use: cowsay <mensagem>"
                } else {
                    &message
                })
            }
            Egg::Fortune => Self::fortune(),
            Egg::Matrix => Self::matrix_message(),
            Egg::SudoSu => Self::sudo_su(),
            Egg::Nuke => Self::nuke_warning(),
            Egg::Whoami => Self::whoami(),
            Egg::HackThePlanet => Self::hack_the_planet(),
            Egg::Konami => Self::konami_code(),
            Egg::Sandwich => Self::sandwich(false),
            Egg::SandwichSudo => Self::sandwich(true),
            Egg::Answer42 => Self::answer_42(),
            Egg::Xyzzy => Self::xyzzy(),
            Egg::Cake => Self::cake(),
            Egg::VimEscape => Self::vim_escape(),
            Egg::StarWars => Self::star_wars(),
        }
    }

    /// Índice pseudo-aleatório derivado do relógio (sem dependências externas).
    fn clock_index(len: usize) -> usize {
        if len == 0 {
            return 0;
        }
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs() as usize)
            .unwrap_or(0)
            % len
    }

    fn train_animation() -> String {
        r#"
      ====        ________                ___________
  _D _|  |_______/        \__I_I_____===__|_________|
   |(_)---  |   H\________/ |   |        =|___ ___|      _________________
   /     |  |   H  |  |     |   |         ||_| |_||     _|                \_____
  |      |  |   H  |__--------------------| [___] |   =|                        |
  | ________|___H__/__|_____/[][]~\_______|       |   -|                        |
  |/ |   |-----------I_____I [][] []  D   |=======|____|________________________|_
__/ =| o |=-~~\  /~~\  /~~\  /~~\ ____Y___________|__|__________________________|_
 |/-=|___|=    ||    ||    ||    |_____/~\___/          |_D__D__D_|  |_D__D__D_|
  \_/      \_O=====O=====O=====O/      \_/               \_/   \_/    \_/   \_/

Ops! Você quis dizer 'ls'? 🚂
"#
        .to_string()
    }

    fn cowsay(message: &str) -> String {
        let border = "-".repeat(message.len() + 2);
        format!(
            r#"
 {border}
< {message} >
 {border}
        \   ^__^
         \  (oo)\_______
            (__)\       )\/\
                ||----w |
                ||     ||
"#
        )
    }

    fn fortune() -> String {
        const FORTUNES: &[&str] = &[
            "Talk is cheap. Show me the code. - Linus Torvalds",
            "In a world without walls and fences, who needs windows and gates?",
            "I'm doing a (free) operating system (just a hobby, won't be big and professional like gnu) - Linus Torvalds, 1991",
            "Software is like sex: it's better when it's free. - Linus Torvalds",
            "Intelligence is the ability to avoid doing work, yet getting the work done. - Linus Torvalds",
            "Real programmers don't use tabs. Real programmers don't use spaces. Real programmers use cats walking on their keyboard.",
            "There are only two hard things in Computer Science: cache invalidation and naming things. - Phil Karlton",
            "Weeks of programming can save you hours of planning.",
            "A computer is like air conditioning - it becomes useless when you open Windows.",
        ];
        format!("\n💭 {}\n", FORTUNES[Self::clock_index(FORTUNES.len())])
    }

    fn matrix_message() -> String {
        r#"
Wake up, Neo...
The Matrix has you...
Follow the white rabbit.

🐰 Knock, knock, Neo.

[ACCESSING MAINFRAME...]
[BYPASSING SECURITY...]
[DOWNLOADING DATA...]
█████████████████████ 100%

Welcome to the real world.
"#
        .to_string()
    }

    fn sudo_su() -> String {
        r#"
╔═══════════════════════════════════════╗
║                                       ║
║   Com grandes poderes vêm grandes     ║
║   responsabilidades.                  ║
║                                       ║
║   Você agora tem poder ROOT.          ║
║   Use com sabedoria.                  ║
║                                       ║
║              - Uncle Ben              ║
║                                       ║
╚═══════════════════════════════════════╝
"#
        .to_string()
    }

    fn nuke_warning() -> String {
        r#"
⚠️  ☢️  ⚠️  ALERTA NUCLEAR  ⚠️  ☢️  ⚠️

Você tentou deletar o UNIVERSO INTEIRO!

Por favor, não faça isso. Existem pessoas
(e gatos) que dependem deste sistema.

Este comando foi bloqueado para sua segurança
e a segurança do mundo digital.

Se você REALMENTE quer destruir tudo:
  1. Isso é uma má ideia
  2. Sério, não faça isso
  3. Vai destruir TUDO
  4. Não diga que não avisei

COMANDO BLOQUEADO! ❌
"#
        .to_string()
    }

    fn whoami() -> String {
        r#"
Você é...

Um hacker? 👨‍💻
Um aprendiz? 🎓
Um curioso? 🤔
Um rebelde? 😎

Você é quem você escolhe ser.

No Munux, você está no caminho
para se tornar uma LENDA! 🚀
"#
        .to_string()
    }

    fn hack_the_planet() -> String {
        r#"
🌍 HACK THE PLANET! 🌍

"They're trashing our rights! Trashing!
Trashing! Trashing!"

Access granted to Gibson mainframe...
Downloading all files...
███████████████████████ 100%

ZERO COOL IS HERE!

Congratulations, you've been 1337 since 1995! 🎮
"#
        .to_string()
    }

    fn konami_code() -> String {
        r#"
🎮 KONAMI CODE ATIVADO! 🎮

┏━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃   30 VIDAS DESBLOQUEADAS  ┃
┃   XP BOOST x2 ATIVADO     ┃
┃   GOD MODE: ON            ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━┛

Achievement desbloqueado:
"Old School Gamer" 🕹️
"#
        .to_string()
    }

    fn sandwich(as_sudo: bool) -> String {
        if as_sudo {
            r#"
🥪 Okay.

(Você fez sudo. Agora é root. O sanduíche é seu.)

   _________________
  /                 \
 |  🥬 🍅 🧀 🥓 🍞  |
  \_________________/

— xkcd #149
"#
            .to_string()
        } else {
            r#"
🤨 O que? Faça você mesmo.

(Dica: tente com 'sudo'.)

— xkcd #149
"#
            .to_string()
        }
    }

    fn answer_42() -> String {
        r#"
        4 2

A Resposta para a Pergunta Fundamental
sobre a Vida, o Universo e Tudo Mais.

Pena que ninguém sabe qual era a pergunta.

🐬 "So long, and thanks for all the fish."
        — Douglas Adams
"#
        .to_string()
    }

    fn xyzzy() -> String {
        r#"
Nothing happens.

(Mas você claramente já jogou Colossal Cave Adventure. Respeito. 🗿)
"#
        .to_string()
    }

    fn cake() -> String {
        r#"
        ,,,,,
       ;;;;;;;
      |_______|
      |   🕯   |
   ___|_______|___
  |               |
  |   THE CAKE    |
  |   IS A LIE    |
  |_______________|

This was a triumph. I'm making a note here: HUGE SUCCESS.
        — GLaDOS, Portal
"#
        .to_string()
    }

    fn vim_escape() -> String {
        r#"
Você não está no Vim. 😏

Mas que reflexo, hein? Anos de :wq na memória muscular.

(Para sair do Munux de verdade: Ctrl+C)
"#
        .to_string()
    }

    fn star_wars() -> String {
        r#"
        ✦        .          ✦
   .          ___________          .
        ____  /          /\
   ✦   /   /\/          /  \    .
      /   /  \         /    \        ✦
     /___/    \_______/______\
     \   \    /       \      /
   .  \   \  /         \    /   ✦
       \___\/___________\__/
              STAR WARS

May the Force be with you.

(Dica nerd: telnet towel.blinkenlights.nl roda o Episódio IV inteiro em ASCII.)
"#
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_known_commands() {
        assert_eq!(EasterEggs::classify("sl"), Some(Egg::Train));
        assert_eq!(EasterEggs::classify("MATRIX"), Some(Egg::Matrix));
        assert_eq!(EasterEggs::classify("cowsay hi"), Some(Egg::Cowsay));
        assert_eq!(EasterEggs::classify(":wq"), Some(Egg::VimEscape));
        assert_eq!(EasterEggs::classify("sudo make me a sandwich"), Some(Egg::SandwichSudo));
        assert_eq!(EasterEggs::classify("ls -la"), None);
    }

    #[test]
    fn check_returns_output_for_eggs() {
        assert!(EasterEggs::check("sl").is_some());
        assert!(EasterEggs::check("xyzzy").unwrap().contains("Nothing happens"));
        assert!(EasterEggs::check("git status").is_none());
    }

    #[test]
    fn decorative_eggs_have_no_achievement() {
        assert!(EasterEggs::achievement(Egg::Fortune).is_none());
        assert!(EasterEggs::achievement(Egg::SudoSu).is_none());
        assert_eq!(EasterEggs::achievement(Egg::Train), Some(("easter_egg_train", 25)));
        // Ambas variantes do sanduíche apontam para a mesma conquista.
        assert_eq!(
            EasterEggs::achievement(Egg::Sandwich),
            EasterEggs::achievement(Egg::SandwichSudo)
        );
    }
}
