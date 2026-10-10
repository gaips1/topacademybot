use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub fn cancel_kb() -> InlineKeyboardMarkup {
    let rows = vec![vec![InlineKeyboardButton::callback("Отменить", "cancel")]];
    InlineKeyboardMarkup::new(rows)
}
