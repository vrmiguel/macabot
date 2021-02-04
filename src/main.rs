mod util;

use std::env;
use futures::StreamExt;
use telegram_bot::*;
use tokio::*;

use util::*;



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
                println!("<{}{}>: {}", &message.from.first_name, get_last_name(&message.from), data);

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
                    "Ebaa, maaaaais uma mensagem pinada...",
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
                    let message = format!("Olá, {}! Qual o seu Pokémon favorito?", username);
                    api.send(SendMessage::new(chat_ref, &message)).await?;
                }
            }

        }
    }
    Ok(())
}