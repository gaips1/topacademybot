use teloxide::dispatching::dialogue::{Dialogue, InMemStorage};

pub type HandlerError = Box<dyn std::error::Error + Send + Sync>;
pub type HandlerResult = Result<(), HandlerError>;
pub type MyDialogue = Dialogue<State, InMemStorage<State>>;

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceiveCredentials,
}
