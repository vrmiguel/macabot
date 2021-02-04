mod util;
mod phrases;

use std::{env, ptr::replace, str};
use futures::StreamExt;
use telegram_bot::*;
use tokio::*;

use util as ut;
use phrases::*;


#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);

    println!("Starting bot.");

    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {

            // Occasionally respond to a text message
            if let MessageKind::Text { ref data, .. } = message.kind {
                println!("<{}{}>: {}", &message.from.first_name, ut::get_last_name(&message.from), data);

                api.send(message.text_reply(format!(
                    "Hi, {}! You just wrote '{}'",
                    &message.from.first_name, data
                )))
                .await?;
            }

            // Respond to pinned messages
            if let MessageKind::PinnedMessage { ref data } = message.kind {
                // let t = *data;
                println!("Someone pinned a message: {:?}", data);
                api.send(message.text_reply(format!(
                    "Não tem mais o que fazer?",
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
                for username in usernames {
                    let chat_ref = ChatRef::from_chat_id(chat_id);
                    let message = format!("Olá, @{}! Qual o seu Pokémon favorito?", username);
                    api.send(SendMessage::new(chat_ref, &message)).await?;
                }
            }

            if let MessageKind::LeftChatMember { ref data } = message.kind {
                let name      = data.first_name.clone();
                let chat_ref = ut::get_chat_ref(&message);
                let msg = USER_LEFT[1];
                let msg = msg.replace("USER", &name);
                api.send(SendMessage::new(chat_ref, &msg)).await?;
            }

            if let MessageKind::Voice { ref data } = message.kind {
                let duration = data.duration;
                println!("Someone has sent a voice message. Duration: {}", duration);
                if duration > 6 {
                    let chat_ref = ut::get_chat_ref(&message);
                    api.send(SendMessage::new(chat_ref, "Ninguém quer ouvir seu áudio, irmão...")).await?;
                }
            }
        }
    }
    Ok(())
}