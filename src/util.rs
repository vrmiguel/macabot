use telegram_bot::*;
use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime, Utc};

use rand::{distributions::WeightedIndex, prelude::*};

use crate::phrases::*;

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

pub (crate) async fn bait (message: &Message, api: &Api) -> Result<(), Error> {
    let msg = choose_elem(BAIT, BAIT_WEIGHTS);
    api.send(message.text_reply(msg)).await?;
    Ok(())
}

#[inline]
fn get_elapsed_time_since(previous: NaiveDateTime) -> Duration {
    let current_time = Utc::now().naive_utc();
    current_time.signed_duration_since(previous)
}

pub (crate) fn is_message_too_old(message: & Message) -> bool {
    let message_data = NaiveDateTime::from_timestamp(message.date, 0);

    let elapsed = get_elapsed_time_since(message_data);

    elapsed.num_seconds() > 5
}

pub (crate) fn is_in_cooldown(last_message_sent: & NaiveDateTime) -> bool {
    let elapsed = get_elapsed_time_since(*last_message_sent);
    elapsed.num_minutes() < 10
}