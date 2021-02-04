mod util;
mod phrases;

use std::env;
use futures::StreamExt;
use telegram_bot::*;

use util as ut;
use phrases::*;

use chrono::{NaiveDateTime, Utc};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);

    println!("Starting bot.");
    

    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        let update = update?;

        if let UpdateKind::Message(message) = update.kind {

            let message_data = NaiveDateTime::from_timestamp(message.date, 0);
            let current_time = Utc::now().naive_utc();

            let elapsed = current_time.signed_duration_since(message_data);

            if elapsed.num_seconds() > 5 {
                println!("Message is too old. Skipping.");
                continue;
            }

            // Occasionally respond to a text message
            if let MessageKind::Text { ref data, .. } = message.kind {
                println!("<{}{}>: {}", &message.from.first_name, ut::get_last_name(&message.from), data);

                match ut::choose_elem(SAY_SOMETHING, SAY_SOMETHING_WEIGHTS) {
                    true => {
                        println!("Saying something");
                        api.send(message.text_reply(ut::choose_elem(
                            RAND_PHRASES, RAND_PHRASES_WEIGHTS
                        )))
                        .await?;
                    },
                    _ => {}
                }
            }

            // Respond to pinned messages
            if let MessageKind::PinnedMessage { ref data } = message.kind {
                // let t = *data;
                println!("Someone pinned a message: {:?}", data);
                api.send(message.text_reply(ut::choose_elem(
                    PINNED_MESSAGE,
                          PINNED_MESSAGE_WEIGHTS
                )))
                .await?;
            }

            if let MessageKind::NewChatMembers { ref data } = message.kind {
                // usernames of the users that entered the chat recently
                let usernames: Vec<String> = data.into_iter()
                            .map(|user| user.username.clone())
                            .filter_map(|option| option)
                            .collect();
            
                let chat_id = message.chat.id();

                // If only one user entered recently, we can reply to his 'entering' message directly
                if usernames.len() == 1 {
                    let msg = format!("Olá, @{}! Qual o seu Pokémon favorito?", usernames[0]);
                    api.send(message.text_reply(&msg)).await?;
                } else {
                    for username in usernames {
                        let chat_ref = ChatRef::from_chat_id(chat_id);
                        let message = format!("Olá, @{}! Qual o seu Pokémon favorito?", username);
                        api.send(SendMessage::new(chat_ref, &message)).await?;
                    }
                }
            }

            if let MessageKind::LeftChatMember { ref data } = message.kind {
                let name      = data.first_name.clone();
                let msg = ut::choose_elem(USER_LEFT, USER_LEFT_WEIGHTS);
                let msg = msg.replace("USER", &name);
                api.send(message.text_reply(msg)).await?;
            }

            if let MessageKind::Voice { ref data } = message.kind {
                let duration = data.duration;
                println!("Someone has sent a voice message. Duration: {}", duration);
                if duration > 6 {
                    // let chat_ref = ut::get_chat_ref(&message);
                    api.send(message.text_reply("Ninguém quer ouvir áudio, irmão...")).await?;
                }
            }

            if let MessageKind::NewChatPhoto { data: _ } = message.kind {                
                api.send(message.text_reply("'Tava melhor antes")).await?;   
            }
        }
    }
    Ok(())
}