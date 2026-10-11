#[derive(Debug, Clone, serde::Deserialize)]
pub struct Reward {
    pub date: String,
    pub action: i32,
    pub current_point: i64,
    pub point_types_id: i64,
    pub point_types_name: PointType,
    pub achievements_id: i64,
    pub achievements_name: Achievement,
    pub achievements_type: i32,
    pub badge: i32,
    pub old_competition: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PointType {
    Diamond,
    Coin,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Achievement {
    EvaluationLessonMark,
    PairVisit,
    WorkInClass,
    Assesment,
    HometaskIntime,
    #[serde(rename = "20_VISITS_WITHOUT_DELAY")]
    Visits20WithoutDelay,
    #[serde(rename = "20_VISITS_WITHOUT_GAP")]
    Visits20WithoutGap,
    #[serde(rename = "10_VISITS_WITHOUT_DELAY")]
    Visits10WithoutDelay,
    #[serde(rename = "10_VISITS_WITHOUT_GAP")]
    Visits10WithoutGap,
    #[serde(rename = "5_VISITS_WITHOUT_DELAY")]
    Visits5WithoutDelay,
    #[serde(rename = "5_VISITS_WITHOUT_GAP")]
    Visits5WithoutGap,
    FillInProfile,
    EmailConfirmation,
    AutoMarkExpiredHomework,
    IntimeLaboratoryWork,
    Survey,
    #[serde(other)]
    Unknown,
}
impl Achievement {
    pub fn translation(&self) -> &'static str {
        match self {
            Achievement::EvaluationLessonMark => "Оценка занятия",
            Achievement::PairVisit => "Посещение пары",
            Achievement::WorkInClass => "Поощрение от преподавателя за работу на уроке",
            Achievement::Assesment => "Оценка",
            Achievement::HometaskIntime => "Своевременная сдача ДЗ",
            Achievement::Visits20WithoutDelay => "20 посещений без опозданий",
            Achievement::Visits20WithoutGap => "20 посещений без пропусков",
            Achievement::Visits10WithoutDelay => "10 посещений без опозданий",
            Achievement::Visits10WithoutGap => "10 посещений без пропусков",
            Achievement::Visits5WithoutDelay => "5 посещений без опозданий",
            Achievement::Visits5WithoutGap => "5 посещений без пропусков",
            Achievement::FillInProfile => "Заполнение профиля",
            Achievement::EmailConfirmation => "Подтверждение почты",
            Achievement::AutoMarkExpiredHomework => "Не выполнение сроков сдачи домашнего задания",
            Achievement::Survey => "Участие в опросе",
            Achievement::IntimeLaboratoryWork => "Своевременное выполнение лабораторной работы",
            Achievement::Unknown => "Неизвестно",
        }
    }
}

impl PointType {
    pub fn translation(&self) -> &'static str {
        match self {
            PointType::Diamond => "Топкоины",
            PointType::Coin => "Топгемы",
            PointType::Unknown => "Неизвестно",
        }
    }
}

pub type RewardsHistory = Vec<Reward>;

impl super::Endpoint for RewardsHistory {
    const PATH: &'static str = "/dashboard/progress/activity";
}
