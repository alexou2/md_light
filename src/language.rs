use core::fmt;
use serde::{ser::SerializeStruct, Serialize, Serializer};


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
const JP: &'static str = "🇯🇵"; //japanese
const DE: &'static str = "🇩🇪"; //german
const HI: &'static str = "🇮🇳"; //india
const PT: &'static str = "🇵🇹"; //portugal
const ID: &'static str = "🇮🇩"; //indonesia
const RU: &'static str = "🇷🇺"; //russian
const TH: &'static str = "🇹🇭"; //thailand
const TR: &'static str = "🇹🇷"; //turkish
const RO: &'static str = "🇷🇴"; //romanian
const UNKNOWN: &'static str = "🌍"; //unknown flag
const ERROR: &'static str = "⚠️"; //unknown flag

#[derive(Clone, Debug)]
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
        let mut s = serializer.serialize_struct("Language", 1)?;
        // s.serialize_field("lang", to_flag_str(&self.lang))?;
        s.serialize_field("lang", &self.lang)?;

        println!("language: {}", &self.lang);
        s.end()
    }
}

impl std::convert::From<String> for Language {
    fn from(lang: String) -> Self {
        let flag = to_flag_str(&lang);
        Language {
            lang: flag.to_owned(),
        }
    }
}

impl std::convert::From<&Option<String>> for Language {
    fn from(lang: &Option<String>) -> Self {
        let lang = match lang {
            Some(e) => e,
            None => ERROR,
        };

        let flag = to_flag_str(lang);
        Language {
            lang: flag.to_owned(),
        }
    }
}

/// takes a language and returns the flag fot the language
pub fn to_flag_str(language: &str) -> &'static str {
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
        "Error" => ERROR,
        _ => {
            println!("unknown language: {}", language);
            UNKNOWN
        }
    };
    return flag;
}
