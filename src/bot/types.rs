pub type HandlerError = Box<dyn std::error::Error + Send + Sync>;
pub type HandlerResult = Result<(), HandlerError>;
