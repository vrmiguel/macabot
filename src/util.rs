use telegram_bot::*;

use rand::{distributions::WeightedIndex, prelude::*};

pub (crate) fn get_last_name(user: &User) -> String {
    if let Some(last_name) = user.last_name.clone() {
        format!(" {}", last_name)
    } else {
        "".into()
    }
}

pub (crate) fn choose_elem<T>(elems: &[T], weights: &[i32]) -> T 
    where T: Clone
{
    let dist = WeightedIndex::new(weights).unwrap();
    let mut rng = thread_rng();

    elems[dist.sample(&mut rng)].clone()
}

pub (crate) fn get_chat_ref(message: & Message) -> ChatRef {
    let chat_id   = message.chat.id();
    ChatRef::from_chat_id(chat_id)
}