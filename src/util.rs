use telegram_bot::*;

use crate::phrases::*;

pub (crate) fn get_last_name(user: &User) -> String {
    if let Some(last_name) = user.last_name.clone() {
        format!(" {}", last_name)
    } else {
        "".into()
    }
}

pub (crate) fn get_chat_ref(message: & Message) -> ChatRef {
    let chat_id   = message.chat.id();
    ChatRef::from_chat_id(chat_id)
}