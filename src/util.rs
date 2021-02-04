use telegram_bot::*;

pub (crate) fn get_last_name(user: &User) -> String {
    if let Some(last_name) = user.last_name.clone() {
        format!(" {}", last_name)
    } else {
        "".into()
    }
}