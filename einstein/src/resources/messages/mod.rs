mod base;
mod messages_de;
mod messages_en;
mod messages_ru;

pub use self::base::Messages;
use crate::locale::get_language;

pub fn get_messages() -> &'static Messages<'static> {
    match get_language().as_deref() {
        Some("ru") => &messages_ru::MESSAGES,
        Some("de") => &messages_de::MESSAGES,
        _ => &messages_en::MESSAGES,
    }
}
