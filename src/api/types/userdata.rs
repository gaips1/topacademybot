#[derive(serde::Deserialize)]
pub struct Group {
    pub id: i64,
    pub name: String,
    pub group_status: i32,
    pub is_primary: bool,
}

#[derive(serde::Deserialize)]
pub struct GamingPoint {
    pub new_gaming_point_types_id: i64,
    pub points: i64,
}

#[derive(serde::Deserialize)]
pub struct Visibility {
    pub is_design: bool,
    pub is_video_courses: bool,
    pub is_vacancy: bool,
    pub is_signal: bool,
    pub is_promo: bool,
    pub is_test: bool,
    pub is_email_verified: bool,
    pub is_quizzes_expired: bool,
    pub is_debtor: bool,
    pub is_phone_verified: bool,
    pub is_only_profile: bool,
    pub is_referral_program: bool,
    pub is_dz_group_issue: bool,
    pub is_birthday: bool,
    pub is_school: bool,
    pub is_news_popup: bool,
    pub is_school_branch: bool,
    pub is_college_branch: bool,
    pub is_higher_education_branch: bool,
    pub is_russian_branch: bool,
}

#[derive(serde::Deserialize)]
pub struct UserData {
    pub groups: Vec<Group>,
    pub manual_link: Option<serde_json::Value>,
    pub student_id: i64,
    pub current_group_id: i64,
    pub full_name: String,
    pub achieves_count: i64,
    pub stream_id: i64,
    pub stream_name: String,
    pub group_name: String,
    pub level: i64,
    pub photo: String,
    pub gaming_points: Vec<GamingPoint>,
    pub spent_gaming_points: Vec<serde_json::Value>,
    pub visibility: Visibility,
    pub current_group_status: i32,
    pub birthday: String,
    pub age: i64,
    pub last_date_visit: String,
    pub registration_date: String,
    pub gender: i32,
    pub study_form_short_name: String,
}

impl super::Endpoint for UserData {
    const PATH: &'static str = "";
}