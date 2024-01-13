use serde::{Serialize, Serializer};
use core::fmt;

use colored::Colorize;

const EN: &'static str = "🇬🇧"; //english
const FR: &'static str = "🇲🇫"; //french
const VI: &'static str = "🇻🇳"; //vietnamese
const IT: &'static str = "🇮🇹"; //italian
const ZH: &'static str = "🇨🇳"; //simplified chinese
const ES: &'static str = "🇲🇽"; //spanish
const BR: &'static str = "🇧🇷"; //brasilian
const PL: &'static str = "🇵🇱"; //polish
const UK: &'static str = "🇺🇦"; //ukranian
const KO: &'static str = "🇰🇷"; //korean
const AR: &'static str = "🇸🇦"; //arabic
const UNKNOWN: &'static str = "🚩"; //unknown flag
const JP: &'static str = "🇯🇵"; //japanese
const DE: &'static str = "🇩🇪"; //german
const HI: &'static str = "🇮🇳"; //india
const PT: &'static str = "🇵🇹"; //portugal
const ID: &'static str = "🇮🇩"; //indonesia
const RU: &'static str = "🇷🇺"; //russian
const TH: &'static str = "🇹🇭"; //thailand
const TR: &'static str = "🇹🇷"; //turkish
const RO: &'static str = "🇷🇴"; //romanian

#[derive(Debug)]
pub struct Language {
    pub lang: String,
}
impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.lang)
    }
}
impl Language {
    pub fn as_str(&self) -> &str {
        &self.lang
    }
}

impl Serialize for Language {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("CustomType", 2)?;
        state.serialize_field("lang", &self.field2)?;
        // state.end()
    }
}



pub fn get_flag_offline(language: &str) -> &'static str {
    let flag = match language {
        "en" => EN,
        "fr" => FR,
        "vi" => VI,
        "it" => IT,
        "zh" => ZH,
        "es-la" | "es" => ES,
        "br" => BR,
        "pl" => PL,
        "uk" => UK,
        "ko" => KO,
        "ar" => AR,
        "ja" | "jp" => JP,
        "de" => DE,
        "hi" => HI,
        "pt" | "pt-br" => PT,
        "id" => ID,
        "ru" => RU,
        "th" => TH,
        "ro" => RO,
        "tr" => TR,
        // _=>format!("{}: {}", unknown, language).as_str().clone(),
        _ => {
            // println!("unknown language: {}", language.on_red());
            UNKNOWN
        }
    };
    return flag;
}
