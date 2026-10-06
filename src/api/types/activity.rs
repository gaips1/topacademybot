#[derive(serde::Deserialize)]
pub struct Activity {
    pub date_visit: String,
    pub lesson_number: i32,
    pub status_was: i32,
    pub spec_id: i64,
    pub teacher_name: String,
    pub spec_name: String,
    pub lesson_theme: String,
    pub control_work_mark: Option<i64>,
    pub home_work_mark: Option<i64>,
    pub lab_work_mark: Option<i64>,
    pub class_work_mark: Option<i64>,
    pub practical_work_mark: Option<i64>,
}

pub type ActivityList = Vec<Activity>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[repr(i32)]
pub enum StatusWas {
    Absent = 0,
    Present = 1,
    Late = 2,
}

impl StatusWas {
    pub fn translation(&self) -> &'static str {
        match self {
            StatusWas::Absent => "Отсутствовал",
            StatusWas::Present => "Присутствовал",
            StatusWas::Late => "Опоздал",
        }
    }

    pub fn from_code(code: i32) -> Option<Self> {
        match code {
            0 => Some(StatusWas::Absent),
            1 => Some(StatusWas::Present),
            2 => Some(StatusWas::Late),
            _ => None,
        }
    }

    pub fn from_field(field: &Activity) -> Option<Self> {
        Self::from_code(field.status_was)
    }
}

impl From<StatusWas> for i32 {
    fn from(s: StatusWas) -> i32 {
        s as i32
    }
}