use tg_flows::{listen_to_update, Telegram, Update, UpdateKind};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    let telegram_token = std::env::var("telegram_token").unwrap();
    let tele = Telegram::new(telegram_token.clone());

    listen_to_update(&telegram_token, |update| handler(tele, update)).await;
    Ok(())
}

async fn handler(tele: Telegram, update: Update) {
    if let UpdateKind::Message(msg) = update.kind {
        let text = msg.text().unwrap_or("");
        let chat_id = msg.chat.id;

        let _sended_msg = tele.send_message(chat_id, text);
    }
}

// async fn handler(bot: &ProvidedBot, msg: Message) {
//     logger::init();
//     let discord = bot.get_client();
//
//     if msg.author.bot {
//         log::debug!("ignored bot message");
//         return;
//     }
//     if msg.member.is_some() {
//         log::debug!("ignored channel message");
//         return;
//     }
//
//     let channel_id = msg.channel_id;
//     let resp = format!("Welcome to flows.network.\nYou just said: '{}'.\nLearn more at: https://github.com/flows-network/hello-world\n", msg.content);
//
//     _ = discord.send_message(
//         channel_id.into(),
//         &serde_json::json!({
//             "content": resp
//         }),
//     ).await;
// }
