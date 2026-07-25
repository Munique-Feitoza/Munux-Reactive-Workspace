// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

//! Guia de comandos por distribuição Linux.
//!
//! O texto dos guias **não** vive aqui: cada idioma tem o seu em
//! `locales/<lang>/guides/<distro>.txt`, carregado por
//! [`crate::i18n::I18n::content`]. Antes os cinco guias estavam embutidos neste
//! arquivo em português fixo, o que deixava metade do `help` monolíngue num app
//! que se anuncia bilíngue.
//!
//! Acrescentar uma distro é: uma entrada em [`GUIDES`] + dois arquivos de texto
//! + uma chave de título nos `.ftl`.

use crate::i18n::I18n;

/// Guia de uma distro: os aliases aceitos no `help <distro>`, o nome do arquivo
/// de conteúdo e a chave Fluent do título.
struct Guide {
    /// Nomes que levam a este guia (comparados em minúsculas).
    aliases: &'static [&'static str],
    /// Nome do arquivo em `locales/<lang>/guides/`, sem a extensão.
    file: &'static str,
    /// Chave Fluent do título exibido no painel.
    title_key: &'static str,
}

/// Catálogo de guias. O último não tem aliases e serve de fallback universal.
const GUIDES: &[Guide] = &[
    Guide { aliases: &["manjaro", "arch"], file: "arch", title_key: "guide-arch-title" },
    Guide {
        aliases: &["ubuntu", "debian", "mint"],
        file: "debian",
        title_key: "guide-debian-title",
    },
    Guide {
        aliases: &["fedora", "rhel", "centos"],
        file: "fedora",
        title_key: "guide-fedora-title",
    },
    Guide { aliases: &["opensuse"], file: "opensuse", title_key: "guide-opensuse-title" },
    Guide { aliases: &[], file: "general", title_key: "guide-general-title" },
];

/// Guia de comandos por distribuição Linux
pub struct DistroGuide;

impl DistroGuide {
    /// Retorna `(conteúdo, título)` do guia da distro, no idioma ativo.
    ///
    /// Fonte única: antes o conteúdo (aqui) e o título (em `app.rs`) eram
    /// mapeados separadamente e divergiam — `help manjaro/mint/rhel/centos`
    /// mostrava o guia certo com o título errado.
    pub fn get(distro: &str, i18n: &I18n) -> (String, String) {
        let needle = distro.to_lowercase();

        let guide = GUIDES
            .iter()
            .find(|g| g.aliases.contains(&needle.as_str()))
            .unwrap_or_else(|| GUIDES.last().expect("GUIDES nunca é vazio"));

        let content = i18n
            .content(&format!("guides/{}.txt", guide.file))
            .unwrap_or_default()
            .to_string();

        (content, i18n.tc(guide.title_key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::i18n::Language;

    /// Todo guia precisa existir nos dois idiomas, com título traduzido. Sem
    /// esta guarda, uma distro nova poderia sair com o painel em branco.
    #[test]
    fn every_guide_has_content_and_title_in_both_locales() {
        for lang in [Language::PtBr, Language::EnUs] {
            let i18n = I18n::new(lang);
            for guide in GUIDES {
                let file = format!("guides/{}.txt", guide.file);
                let content = i18n
                    .content(&file)
                    .unwrap_or_else(|| panic!("{} ausente no locale {:?}", file, lang));
                assert!(
                    content.len() > 200,
                    "{} parece truncado em {:?} ({} bytes)",
                    file,
                    lang,
                    content.len()
                );

                let title = i18n.tc(guide.title_key);
                assert!(
                    !title.starts_with("[MISSING"),
                    "título '{}' ausente em {:?}",
                    guide.title_key,
                    lang
                );
            }
        }
    }

    /// Cada alias precisa levar ao guia certo, e um nome desconhecido tem que
    /// cair no guia universal.
    #[test]
    fn aliases_route_to_the_right_guide() {
        let i18n = I18n::new(Language::PtBr);

        for (alias, marker) in [
            ("manjaro", "pacman"),
            ("ARCH", "pacman"), // maiúsculas não importam
            ("mint", "apt"),
            ("centos", "dnf"),
            ("opensuse", "zypper"),
        ] {
            let (content, _) = DistroGuide::get(alias, &i18n);
            assert!(content.contains(marker), "'{}' não levou ao guia com '{}'", alias, marker);
        }

        // Distro desconhecida cai no universal.
        let (content, title) = DistroGuide::get("plan9", &i18n);
        assert!(content.contains("ls"), "o fallback deveria ser o guia universal");
        assert_eq!(title, i18n.tc("guide-general-title"));
    }

    /// O conteúdo tem que mudar de idioma junto com o locale — senão a tradução
    /// existe no disco mas ninguém a lê.
    #[test]
    fn guides_actually_differ_between_locales() {
        let pt = DistroGuide::get("arch", &I18n::new(Language::PtBr)).0;
        let en = DistroGuide::get("arch", &I18n::new(Language::EnUs)).0;
        assert_ne!(pt, en, "o guia em inglês não pode ser o texto em português");
        assert!(pt.contains("Instala pacote"), "guia pt-BR mudou de texto");
        assert!(en.contains("Install a package"), "guia en-US mudou de texto");
    }
}
