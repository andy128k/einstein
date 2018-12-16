use std::ffi::{CStr, CString};
use libc::{setlocale, LC_ALL};
use regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    static ref LANGUAGE: Option<String> = detect_language();
}

fn safe_setlocale_lc_all() -> Option<String> {
    let empty = CString::new("").ok()?;
    unsafe {
        let locale = setlocale(LC_ALL, empty.as_ptr());
        if locale.is_null() {
            None
        } else {
            CStr::from_ptr(locale)
                .to_str()
                .map(|s| s.to_string())
                .ok()
        }
    }
}

#[cfg(unix)]
fn detect_language() -> Option<String> {
    let locale_str = safe_setlocale_lc_all()?;
    let re = Regex::new("^[^_.]*").ok()?;
    if let Some(m) = re.find(&locale_str) {
        Some(m.as_str().to_lowercase())
    } else {
        None
    }
}

#[cfg(windows)]
fn detect_language() -> Option<String> {
    use winapi::um::winnls::GetLocaleInfoW;
    use winapi::um::winnt::LOCALE_USER_DEFAULT;
    use winapi::um::winnt::WCHAR;

    safe_setlocale_lc_all()?;

    let mut buf: [WCHAR; 100] = [0; 100];
    let len = unsafe { GetLocaleInfoW(LOCALE_USER_DEFAULT, /*LOCALE_SABBREVLANGNAME*/3, buf.as_mut_ptr(), 99) };
    if len <= 0 {
        return None;
    }

    /* according to MSDN:
       LOCALE_SABBREVLANGNAME   Abbreviated name of the language,
       created by taking the 2-letter language abbreviation from the
       ISO Standard 639 and adding a third letter, as appropriate,
       to indicate the sublanguage.
    */
    String::from_utf16(&buf[0..2]).map(|s| s.to_lowercase()).ok()
}

pub fn get_language() -> Option<String> {
    LANGUAGE.clone()
}
