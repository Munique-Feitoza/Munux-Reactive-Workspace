// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

//! Easter eggs do terminal.
//!
//! O reconhecimento do comando acontece uma única vez em [`EasterEggs::classify`];
//! a partir do [`Egg`] resultante, [`EasterEggs::render`] produz a arte/texto e
//! [`EasterEggs::achievement`] diz qual conquista (se houver) ele concede. Isso
//! evita duplicar a lista de comandos entre a saída visual e o sistema de
//! conquistas.
//!
//! A **arte e o texto** ficam em `locales/<lang>/eggs/<egg>.txt`, carregados por
//! [`crate::i18n::I18n::content`]. Antes estavam embutidos aqui em blocos
//! `r#"..."#` com o texto fixo em português. Alguns eggs citam obras em inglês
//! (Matrix, Portal, Hackers) e são idênticos nos dois idiomas de propósito —
//! citação não se traduz.

use crate::i18n::I18n;

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

impl Egg {
    /// Nome do arquivo em `locales/<lang>/eggs/`, sem extensão.
    ///
    /// `Cowsay` e `Fortune` devolvem `None`: são gerados em tempo de execução
    /// (a fala do usuário / uma citação sorteada), não lidos de um arquivo.
    fn content_file(self) -> Option<&'static str> {
        let name = match self {
            Egg::Train => "train",
            Egg::Matrix => "matrix",
            Egg::SudoSu => "sudo_su",
            Egg::Nuke => "nuke",
            Egg::Whoami => "whoami",
            Egg::HackThePlanet => "hack_the_planet",
            Egg::Konami => "konami",
            Egg::Sandwich => "sandwich",
            Egg::SandwichSudo => "sandwich_sudo",
            Egg::Answer42 => "answer42",
            Egg::Xyzzy => "xyzzy",
            Egg::Cake => "cake",
            Egg::VimEscape => "vim_escape",
            Egg::StarWars => "star_wars",
            Egg::Cowsay | Egg::Fortune => return None,
        };
        Some(name)
    }
}

/// Citações do `fortune`. Ficam no código, e não nos locales, porque são
/// citações originais em inglês — traduzi-las descaracterizaria a fala.
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

    /// Saída visual de um easter egg, no idioma ativo.
    pub fn check(command: &str, i18n: &I18n) -> Option<String> {
        Self::classify(command).map(|egg| Self::render(egg, command, i18n))
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

    /// Renderiza a arte/texto do easter egg no idioma ativo.
    ///
    /// Os que vêm de arquivo saem por [`Egg::content_file`]; só os dois
    /// dinâmicos — `cowsay` (repete a fala) e `fortune` (sorteia uma citação) —
    /// têm tratamento próprio.
    fn render(egg: Egg, command: &str, i18n: &I18n) -> String {
        match egg {
            Egg::Cowsay => {
                let message = command.to_lowercase().replace("cowsay", "").trim().to_string();
                let message =
                    if message.is_empty() { i18n.tc("egg-cowsay-default") } else { message };
                Self::cowsay(&message)
            }
            Egg::Fortune => {
                format!("\n💭 {}\n", FORTUNES[crate::game::rng::index(FORTUNES.len())])
            }
            _ => egg
                .content_file()
                .and_then(|name| i18n.content(&format!("eggs/{}.txt", name)))
                .unwrap_or_default()
                .to_string(),
        }
    }

    /// Monta o balão de fala da vaca em volta de `message`.
    fn cowsay(message: &str) -> String {
        // A largura da borda acompanha os **caracteres**, não os bytes: com
        // `len()` um "olá" acentuado desalinhava o balão.
        let border = "-".repeat(message.chars().count() + 2);
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::Language;

    fn i18n() -> I18n {
        I18n::new(Language::PtBr)
    }

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
        let i18n = i18n();
        assert!(EasterEggs::check("sl", &i18n).is_some());
        assert!(EasterEggs::check("xyzzy", &i18n).unwrap().contains("Nothing happens"));
        assert!(EasterEggs::check("git status", &i18n).is_none());
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

    /// Todo egg baseado em arquivo precisa ter conteúdo nos dois idiomas —
    /// senão o painel abriria vazio.
    #[test]
    fn every_file_backed_egg_has_content_in_both_locales() {
        const ALL: &[Egg] = &[
            Egg::Train,
            Egg::Matrix,
            Egg::SudoSu,
            Egg::Nuke,
            Egg::Whoami,
            Egg::HackThePlanet,
            Egg::Konami,
            Egg::Sandwich,
            Egg::SandwichSudo,
            Egg::Answer42,
            Egg::Xyzzy,
            Egg::Cake,
            Egg::VimEscape,
            Egg::StarWars,
        ];

        for lang in [Language::PtBr, Language::EnUs] {
            let i18n = I18n::new(lang);
            for egg in ALL {
                let name = egg.content_file().expect("egg de arquivo sem nome de arquivo");
                let text = i18n
                    .content(&format!("eggs/{}.txt", name))
                    .unwrap_or_else(|| panic!("eggs/{}.txt ausente em {:?}", name, lang));
                assert!(!text.trim().is_empty(), "eggs/{}.txt vazio em {:?}", name, lang);

                // E o render de verdade também não pode sair vazio.
                let rendered = EasterEggs::render(*egg, "", &i18n);
                assert!(!rendered.trim().is_empty(), "{:?} renderizou vazio em {:?}", egg, lang);
            }
        }
    }

    /// Os eggs com texto próprio precisam realmente mudar de idioma; os que
    /// citam obras em inglês continuam iguais de propósito.
    #[test]
    fn translated_eggs_differ_and_quotes_stay_the_same() {
        let pt = I18n::new(Language::PtBr);
        let en = I18n::new(Language::EnUs);

        for egg in [Egg::Nuke, Egg::Whoami, Egg::Konami, Egg::Sandwich, Egg::VimEscape] {
            assert_ne!(
                EasterEggs::render(egg, "", &pt),
                EasterEggs::render(egg, "", &en),
                "{:?} deveria ter tradução própria",
                egg
            );
        }

        // Citações de Matrix / Portal / Hackers não se traduzem.
        for egg in [Egg::Matrix, Egg::Cake, Egg::HackThePlanet] {
            assert_eq!(
                EasterEggs::render(egg, "", &pt),
                EasterEggs::render(egg, "", &en),
                "{:?} é citação em inglês e deve ser idêntica nos dois locales",
                egg
            );
        }
    }

    /// O balão do cowsay tem que fechar certo mesmo com acento — a borda é
    /// contada em caracteres, não em bytes.
    #[test]
    fn cowsay_border_matches_message_width_with_accents() {
        let out = EasterEggs::cowsay("olá coração");
        let border =
            out.lines().find(|l| l.trim_start().starts_with('-')).expect("sem borda");
        let speech = out.lines().find(|l| l.starts_with("< ")).expect("sem fala");

        assert_eq!(
            border.trim().chars().count(),
            speech.chars().count() - 2,
            "borda e balão desalinhados: {:?} vs {:?}",
            border,
            speech
        );
    }

    #[test]
    fn cowsay_without_message_uses_the_localized_default() {
        let out = EasterEggs::check("cowsay", &i18n()).unwrap();
        assert!(out.contains("cowsay <mensagem>"), "faltou a dica padrão: {}", out);
    }
}
