use anyhow::{Result, ensure};
use teloxide::{
    Bot,
    payloads::{EditMessageTextSetters, SendMessageSetters},
    requests::Requester,
    types::{ChatId, InlineKeyboardMarkup, MessageId, ParseMode},
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

pub async fn send_split(
    bot: &Bot,
    chat_id: ChatId,
    message_id: MessageId,
    text: &str,
    keyboard: InlineKeyboardMarkup,
) -> Result<()> {
    ensure!(!text.is_empty(), "нет текста");

    let parts = split_text(text);
    let mut keyboard = Some(keyboard);

    for (i, text) in parts.iter().enumerate() {
        let markup = if i == parts.len() - 1 {
            keyboard.take().unwrap_or_default()
        } else {
            InlineKeyboardMarkup::default()
        };

        if i == 0 {
            bot.edit_message_text(chat_id, message_id, text)
                .parse_mode(ParseMode::Html)
                .reply_markup(markup)
                .await?;
        } else {
            bot.send_message(chat_id, text)
                .parse_mode(ParseMode::Html)
                .reply_markup(markup)
                .await?;
        }
    }

    Ok(())
}
