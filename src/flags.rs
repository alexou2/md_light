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

pub fn get_flag_offline(language: &str) -> &'static str {
    let flag = match language {
        "en" => EN,
        "fr" => FR,
        "vi" => VI,
        "it" => IT,
        "zh" => ZH,
        "es" => ES,
        "es-la" => ES,
        "br" => BR,
        "pl" => PL,
        "uk" => UK,
        "ko" => KO,
        "ar" => AR,
        "jp" => JP,
        "de" => DE,
        "hi" => HI,
        "pt-br" => PT,
        "pt" => PT,
        "id" => ID,
        "ru" => RU,
        "th" => TH,
        "ro" => RO,
        "tr" => TR,
        // _=>format!("{}: {}", unknown, language).as_str().clone(),
        _ => {
            println!("unknown language: {}", language.on_red());
            UNKNOWN
        }
    };
    return flag;
}
