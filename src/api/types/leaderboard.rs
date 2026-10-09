#[derive(serde::Deserialize)]
pub struct StudentRating {
    pub id: Option<i64>,
    pub full_name: Option<String>,
    pub photo_path: Option<String>,
    pub position: Option<i64>,
    pub amount: Option<i64>,
}

#[derive(serde::Deserialize)]
#[serde(transparent)]
pub struct StudentRatingListGroup(pub Vec<StudentRating>);

#[derive(serde::Deserialize)]
#[serde(transparent)]
pub struct StudentRatingListStream(pub Vec<StudentRating>);

impl super::Endpoint for StudentRatingListGroup {
    const PATH: &'static str = "/dashboard/progress/leader-group";
}

impl super::Endpoint for StudentRatingListStream {
    const PATH: &'static str = "/dashboard/progress/leader-stream";
}
