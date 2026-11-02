#[derive(Debug, Clone, serde::Deserialize)]
pub struct Evaluate {
    key: String,
    date_visit: String,
    fio_teach: String,
    spec_name: String,
}

pub type EvaluateList = Vec<Evaluate>;

impl super::Endpoint for EvaluateList {
    const PATH: &'static str = "/feedback/students/evaluate-lesson-list";
}
