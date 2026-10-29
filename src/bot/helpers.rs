use anyhow::Context;
use teloxide::{
    payloads::{EditMessageTextSetters, SendMessageSetters},
    requests::Requester,
    types::{ChatId, MessageId},
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