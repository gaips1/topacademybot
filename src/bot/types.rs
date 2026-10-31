use teloxide::dispatching::dialogue::{Dialogue, InMemStorage};

pub type HandlerError = Box<dyn std::error::Error + Send + Sync>;
pub type HandlerResult = Result<(), HandlerError>;
pub type MyDialogue = Dialogue<State, InMemStorage<State>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceiveCredentials,

    HomeworkReceiveTime {
        homework_id: u64,
        evaluate: u8,
    },
    HomeworkReceiveTextAnswer {
        homework_id: u64,
        evaluate: u8,
        time_hours: u8,
        time_minutes: u8,
    },
    HomeworkReceiveFileAnswer {
        homework_id: u64,
        evaluate: u8,
        time_hours: u8,
        time_minutes: u8,
        text_answer: Option<String>,
    },
}
