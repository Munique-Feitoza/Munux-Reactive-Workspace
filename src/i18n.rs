// Author: Munique Alves Pacheco Feitoza
// License: GPLv3

use std::collections::HashMap;
use fluent::{FluentBundle, FluentResource, FluentArgs};
use include_dir::{include_dir, Dir};
use serde::{Deserialize, Serialize};
use unic_langid::LanguageIdentifier;

static LOCALES_DIR: Dir<'_> = include_dir!("locales");

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    PtBr,
    EnUs,
}

impl Language {
    pub fn detect() -> Self {
        match sys_locale::get_locale() {
            Some(locale) if locale.starts_with("pt") => Language::PtBr,
            _ => Language::EnUs,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Language::PtBr => "pt-BR",
            Language::EnUs => "en-US",
        }
    }

    pub fn to_langid(self) -> LanguageIdentifier {
        self.as_str().parse().expect("Invalid LangID")
    }
}

pub struct I18n {
    pub lang: Language,
    bundles: HashMap<Language, FluentBundle<FluentResource>>,
}

impl I18n {
    pub fn new(lang: Language) -> Self {
        let mut bundles = HashMap::new();
        
        // Load all available locales
        for entry in LOCALES_DIR.dirs() {
            let lang_str = entry.path().file_name().unwrap().to_str().unwrap();
            let lang_enum = match lang_str {
                "pt-BR" => Language::PtBr,
                "en-US" => Language::EnUs,
                _ => continue,
            };

            let mut bundle = FluentBundle::new(vec![lang_enum.to_langid()]);
            
            if let Some(file) = entry.get_file(format!("{}/main.ftl", lang_str)) {
                let content = file.contents_utf8().expect("Invalid UTF-8 in FTL");
                let resource = FluentResource::try_new(content.to_string())
                    .expect("Failed to parse FTL");
                bundle.add_resource(resource).expect("Failed to add resource");
            }
            
            bundles.insert(lang_enum, bundle);
        }

        Self { lang, bundles }
    }

    /// Primary translation method with keys and optional arguments
    pub fn t(&self, key: &str, args: Option<&FluentArgs>) -> String {
        let bundle = self.bundles.get(&self.lang)
            .or_else(|| self.bundles.get(&Language::EnUs)) // Fallback to EN
            .expect("No bundles loaded");

        let msg = bundle.get_message(key)
            .or_else(|| {
                // If not found in current lang, try fallback (English)
                if self.lang != Language::EnUs {
                    self.bundles.get(&Language::EnUs)?.get_message(key)
                } else {
                    None
                }
            });

        match msg {
            Some(m) => {
                let pattern = m.value().expect("Message has no value");
                let mut errors = vec![];
                bundle.format_pattern(pattern, args, &mut errors).to_string()
            }
            None => format!("[MISSING: {}]", key),
        }
    }

    /// Shorthand for simple translations without arguments
    pub fn tc(&self, key: &str) -> String {
        self.t(key, None)
    }

    /// Implementation for specific helpers to maintain compatibility or ease of use
    pub fn xp_label(&self, current: u32, next: u32, percent: f64) -> String {
        format!("XP: {}/{} ({:.0}%)", current, next, percent)
    }

    pub fn rank_name(&self, level: u32) -> String {
        let key = match level {
            1..=9 => "game-rank-novice",
            10..=19 => "game-rank-apprentice",
            20..=29 => "game-rank-hacker",
            30..=39 => "game-rank-elite",
            _ => "game-rank-legend",
        };
        self.tc(key)
    }

    pub fn welcome_title(&self) -> String { self.tc("ui-welcome-title") }
    pub fn resource_title(&self) -> String { self.tc("ui-resource-title") }
    pub fn danger_title(&self) -> String { self.tc("ui-danger-zone-title") }
    pub fn level_up_title(&self) -> String { self.tc("ui-level-up-title") }
    pub fn cpu_usage_label(&self) -> String { self.tc("ui-cpu-usage") }
    
    pub fn navigation_title(&self) -> String { self.tc("ui-navigation") }
    pub fn level_message(&self, level: u32) -> String {
        let key = match level {
            1 => "game-msg-level-1",
            2..=4 => "game-msg-level-2",
            5 => "game-msg-level-5",
            6..=9 => "game-msg-level-6",
            10 => "game-msg-level-10",
            11..=19 => "game-msg-level-11",
            20 => "game-msg-level-20",
            21..=29 => "game-msg-level-21",
            30 => "game-msg-level-30",
            31..=49 => "game-msg-level-31",
            50 => "game-msg-level-50",
            _ => "game-msg-level-default",
        };
        self.tc(key)
    }

    pub fn art_tag(&self, mode: &str) -> String {
        let key = format!("game-art-{}-tag", mode.to_lowercase());
        let val = self.t(&key, None);
        if val.starts_with("[MISSING") {
            format!("[{}]", mode.to_uppercase())
        } else {
            val
        }
    }

    pub fn level_commands(&self, level: u32) -> Vec<(&'static str, String)> {
        match level {
            1..=4 => vec![
                ("ls", self.t("hint-ls", None)),
                ("pwd", self.t("hint-pwd", None)),
                ("mkdir [nome]", self.t("hint-mkdir", None)),
            ],
            5..=10 => vec![
                ("cat [arquivo]", self.t("hint-cat", None)),
                ("rm [arquivo]", self.t("hint-rm", None)),
                ("cp [orig] [dest]", self.t("hint-cp", None)),
                ("mv [orig] [dest]", self.t("hint-mv", None)),
            ],
            _ => vec![
                ("ssh [user]@[host]", self.t("hint-ssh", None)),
                ("ps aux | grep ...", self.t("hint-grep", None)),
                ("systemctl status ...", self.t("hint-systemctl", None)),
            ],
        }
    }

    pub fn esc_to_back(&self) -> String { self.tc("ui-esc-to-back") }
    pub fn scroll_hint(&self) -> String { self.tc("ui-scroll-hint") }
    
    pub fn preview_title(&self, filename: &str) -> String {
        let mut args = FluentArgs::new();
        args.set("filename", filename);
        self.t("ui-preview-title", Some(&args))
    }

    pub fn achievement_info(&self, id: &str) -> (String, String) {
        let name_key = format!("achievement-{}-name", id);
        let desc_key = format!("achievement-{}-desc", id);
        (self.tc(&name_key), self.tc(&desc_key))
    }

    pub fn command_hint(&self, cmd: &str) -> Option<String> {
        let key = format!("hint-{}", cmd);
        let val = self.t(&key, None);
        if val.starts_with("[MISSING") {
             None
        } else {
            Some(val)
        }
    }
}
