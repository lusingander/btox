use arboard::Clipboard;

use crate::msg::Msg;

pub fn copy_to_clipboard(text: &str) -> Option<Msg> {
    let result = Clipboard::new().and_then(|mut c| c.set_text(text));
    match result {
        Ok(_) => Some(Msg::NotifyInfo("Copy to clipboard succeeded".into())),
        Err(_) => Some(Msg::NotifyError("Copy to clipboard failed".into())),
    }
}

pub fn paste_from_clipboard() -> Option<String> {
    Clipboard::new().and_then(|mut c| c.get_text()).ok()
}
