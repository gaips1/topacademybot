#[derive(serde::Deserialize, Default)]
pub struct Chart {
    pub date: String,
    pub points: Option<i8>,
    pub previous_points: Option<i8>,
    pub has_rasp: Option<bool>,
}

#[derive(serde::Deserialize, Default)]
#[serde(transparent)]
pub struct AttedanceList(pub Vec<Chart>);

#[derive(serde::Deserialize, Default)]
#[serde(transparent)]
pub struct AverageProgressList(pub Vec<Chart>);

impl super::Endpoint for AttedanceList {
    const PATH: &'static str = "/dashboard/chart/attendance";
}

impl super::Endpoint for AverageProgressList {
    const PATH: &'static str = "/dashboard/chart/average-progress";
}
