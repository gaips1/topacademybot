use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub fn cancel_kb() -> InlineKeyboardMarkup {
    let rows = vec![vec![InlineKeyboardButton::callback("Отменить", "cancel")]];
    InlineKeyboardMarkup::new(rows)
}

pub fn main_menu_kb() -> InlineKeyboardMarkup {
    let rows = vec![
        vec![
            InlineKeyboardButton::callback("Домашние задания", "homework/3/1"),
        ],
        vec![
            InlineKeyboardButton::callback("Посещаемость/оценки", "activity/0")
        ],
        vec![
            InlineKeyboardButton::callback("Таблица лидеров", "leaderboard/stream")
        ],
        vec![
            InlineKeyboardButton::callback("Награды", "rewards/0")
        ]
    ];
    InlineKeyboardMarkup::new(rows)
}