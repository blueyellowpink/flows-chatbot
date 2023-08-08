use flowsnet_platform_sdk::logger;
use tg_flows::{listen_to_update, Telegram, Update, UpdateKind};

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    logger::init();

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
