#[derive(serde::Deserialize)]
pub struct StudentRating {
    pub id: Option<i64>,
    pub full_name: Option<String>,
    pub photo_path: Option<String>,
    pub position: Option<i64>,
    pub amount: Option<i64>,
}

pub type StudentRatingList = Vec<StudentRating>;