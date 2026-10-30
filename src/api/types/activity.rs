#[derive(serde::Deserialize)]
pub struct Activity {
    pub date_visit: String,
    pub lesson_number: Option<u64>,
    pub status_was: Option<u8>,
    pub teacher_name: String,
    pub spec_name: String,
    pub lesson_theme: String,
    pub control_work_mark: Option<u8>,
    pub home_work_mark: Option<u8>,
    pub lab_work_mark: Option<u8>,
    pub class_work_mark: Option<u8>,
    pub practical_work_mark: Option<u8>,
}

impl Activity {
    pub fn status_was(&self) -> &'static str {
        match self.status_was {
            Some(0) => "Отсутствовал",
            Some(1) => "Присутствовал",
            Some(2) => "Опоздал",
            _ => "Неизвестно",
        }
    }
}

pub type ActivityList = Vec<Activity>;

impl super::Endpoint for ActivityList {
    const PATH: &'static str = "/progress/operations/student-visits";
}
