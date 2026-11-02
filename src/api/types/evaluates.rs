#[derive(Debug, Clone, serde::Deserialize)]
pub struct Evaluate {
    pub key: String,
    pub date_visit: String,
    pub fio_teach: String,
    pub spec_name: String,
}

pub type EvaluateList = Vec<Evaluate>;

impl super::Endpoint for EvaluateList {
    const PATH: &'static str = "/feedback/students/evaluate-lesson-list";
}
