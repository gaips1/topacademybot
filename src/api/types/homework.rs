#[derive(serde::Deserialize)]
pub struct HomeworkComment {
    pub text_comment: Option<String>,
    pub attachment: Option<String>,
    pub attachment_path: Option<String>,
    pub date_updated: String,
}

#[derive(serde::Deserialize)]
pub struct HomeworkStud {
    pub id: i64,
    pub filename: Option<String>,
    pub file_path: Option<String>,
    pub tmp_file: Option<String>,
    pub mark: Option<i64>,
    pub creation_time: String,
    pub stud_answer: Option<String>,
    pub auto_mark: bool,
}

#[derive(serde::Deserialize)]
pub struct Homework {
    pub id: i64,
    pub id_spec: i64,
    pub id_teach: i64,
    pub id_group: i64,
    pub fio_teach: String,
    pub theme: String,
    pub completion_time: String,
    pub creation_time: String,
    pub overdue_time: String,
    pub filename: Option<String>,
    pub file_path: String,
    pub comment: String,
    pub name_spec: String,
    pub status: i32,
    pub common_status: Option<i32>,
    pub homework_stud: Option<HomeworkStud>,
    pub homework_comment: Option<HomeworkComment>,
    pub cover_image: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct HomeworkCounter {
    pub counter_type: i32,
    pub counter: i64,
}

pub type HomeworkCounterList = Vec<HomeworkCounter>;
pub type HomeworkList = Vec<Homework>;
