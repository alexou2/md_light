const en: &'static str = "🇬🇧"; //english
const fr: &'static str = "🇲🇫"; //french
const vi: &'static str = "🇻🇳"; //vietnamese
const it: &'static str = "🇮🇹"; //italian
const zh: &'static str = "🇨🇳"; //simplified chinese
const es: &'static str = "🇲🇽"; //spanish
const br: &'static str = "🇧🇷"; //brasilian
const po: &'static str = "🇵🇱"; //polish
const uk: &'static str = "🇺🇦"; //ukranian
const ko: &'static str = "🇰🇷"; //korean
const ar: &'static str = "🇸🇦"; //arabic
const unknown: &'static str = "🚩"; //unknown flag
const jp: &'static str = "🇯🇵"; //japanese
const de: &'static str = "🇩🇪"; //german
const hi: &'static str = "🇮🇳"; //india
const pt: &'static str = "🇵🇹"; //portugal
const id: &'static str = "🇮🇩"; //indonesia
const ru: &'static str = "🇷🇺"; //indonesia
const th: &'static str = "🇹🇭"; //indonesia

pub fn get_flag_offline(language: &str) -> &'static str {
    let flag = match language {
        "en" => en,
        "fr" => fr,
        "vi" => vi,
        "it" => it,
        "zh" => zh,
        "es" => es,
        "es-la" => es,
        "br" => br,
        "po" => po,
        "uk" => uk,
        "ko" => ko,
        "ar" => ar,
        "jp" => jp,
        "de" => de,
        "hi" => hi,
        "pt-br" => pt,
        "pt" => pt,
        "id" => id,
        "ru" => ru,
        "th" => th,
        // _=>format!("{}: {}", unknown, language).as_str().clone(),
        _ => {
            println!("unknown language: {language}");
            unknown
        }
    };
    return flag;
}
