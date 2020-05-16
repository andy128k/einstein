mod base;
mod rules_de;
mod rules_en;
mod rules_ru;

pub use self::base::TextItem;
use crate::locale::get_language;

pub fn get_rules() -> &'static [TextItem<'static>] {
    match get_language().as_ref().map(String::as_str) {
        Some("ru") => &rules_ru::RULES,
        Some("de") => &rules_de::RULES,
        _ => &rules_en::RULES,
    }
}
