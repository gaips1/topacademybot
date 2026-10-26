use anyhow::Context;
use teloxide::{
    payloads::{EditMessageTextSetters, SendMessageSetters}, requests::Requester, types::{ChatId, MessageId},
};

pub fn split_text(text: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut len = 0;

    for line in text.split('\n') {
        let chars: Vec<char> = line.chars().collect();

        let chunks: Vec<String> = if chars.is_empty() {
            vec![String::new()]
        } else {
            chars.chunks(4096).map(|c| c.iter().collect()).collect()
        };

        for chunk in chunks {
            let n = chunk.chars().count();

            if !current.is_empty() && len + 1 + n > 4096 {
                parts.push(std::mem::take(&mut current));
                len = 0;
            }
            if !current.is_empty() {
                current.push('\n');
                len += 1;
            }
            current.push_str(&chunk);
            len += n;
        }
    }

    if !current.is_empty() {
        parts.push(current);
    }
    parts
}

pub async fn send_split_text(
    bot: teloxide::Bot,
    msg_chat_id: ChatId,
    msg_id: MessageId,
    text: &str,
) -> super::types::HandlerResult {
    let splitted = split_text(text);

    bot.edit_message_text(msg_chat_id, msg_id, splitted.first().context("нет текста")?)
        .parse_mode(teloxide::types::ParseMode::Html)
        .await?;

    for i in 1..splitted.len() {
        bot.send_message(msg_chat_id, splitted.get(i).unwrap())
            .parse_mode(teloxide::types::ParseMode::Html)
            .await?;
    }

    Ok(())
}
