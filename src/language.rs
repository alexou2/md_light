use core::fmt;
use serde::{ser::SerializeStruct, Serialize, Serializer};
use serde_json::Value;

use crate::tera_templates::ValueExtensions;

const EN: &str = "🇬🇧"; //english
const FR: &str = "🇲🇫"; //french
const VI: &str = "🇻🇳"; //vietnamese
const IT: &str = "🇮🇹"; //italian
const ZH: &str = "🇨🇳"; //simplified chinese
const ES: &str = "🇲🇽"; //spanish
const BR: &str = "🇧🇷"; //brasilian
const PL: &str = "🇵🇱"; //polish
const UK: &str = "🇺🇦"; //ukranian
const KO: &str = "🇰🇷"; //korean
const AR: &str = "🇸🇦"; //arabic
const JP: &str = "🇯🇵"; //japanese
const DE: &str = "🇩🇪"; //german
const HI: &str = "🇮🇳"; //india
const PT: &str = "🇵🇹"; //portugal
const ID: &str = "🇮🇩"; //indonesia
const RU: &str = "🇷🇺"; //russian
const TH: &str = "🇹🇭"; //thailand
const TR: &str = "🇹🇷"; //turkish
const RO: &str = "🇷🇴"; //romanian
const UNKNOWN: &str = "🌍"; //unknown flag
const ERROR: &str = "⚠️"; //unknown flag

#[derive(Clone, Debug)]
pub struct Language {
    pub lang: String,
    pub flag: String,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &self.flag)
    }
}

pub trait AsStr {
    fn as_str(&self) -> &str;
}

impl AsStr for Language {
    fn as_str(&self) -> &str {
        &self.flag
    }
}

impl Serialize for Language {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Language", 1)?;
        // s.serialize_field("lang", to_flag_str(&self.lang))?;
        s.serialize_field("flag", &self.flag)?;
        s.serialize_field("lang", &self.lang)?;

        s.end()
    }
}

impl std::convert::From<String> for Language {
    fn from(lang: String) -> Self {
        let flag = to_flag_str(&lang);
        Language {
            lang: lang.to_owned(),
            flag: flag.to_owned(),
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
            lang: lang.to_owned(),
            flag: flag.to_owned(),
        }
    }
}

impl std::convert::From<Option<String>> for Language {
    fn from(lang: Option<String>) -> Self {
        let lang = match lang {
            Some(e) => e,
            None => ERROR.to_string(),
        };

        let flag = to_flag_str(&lang);
        Language {
            lang: lang.to_owned(),
            flag: flag.to_owned(),
        }
    }
}

impl Language {
    pub fn to_language_vec(lang_vec: Option<&Vec<Value>>) -> Vec<Self> {
        let mut language_vector = vec![];
        if lang_vec.is_none() {
            let flag = to_flag_str(ERROR);
            let flag = Language {
                lang: "Error".to_owned(),
                flag: flag.to_owned(),
            };
            return vec![flag];
        }
        for lang in lang_vec.unwrap() {
            let lang = lang.remove_quotes();

            let lang = match lang {
                Some(e) => e,
                None => ERROR.to_string(),
            };
            let flag = to_flag_str(&lang);
            let flag = Language {
                lang: lang.to_owned(),
                flag: flag.to_owned(),
            };

            language_vector.push(flag);
        }

        // todo!()
        language_vector
    }
}

/// takes a language and returns the flag fot the language
pub fn to_flag_str(language: &str) -> &'static str {
    match language {
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
    }
}
