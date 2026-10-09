#[derive(serde::Deserialize)]
pub struct Reward {
    pub date: String,
    pub action: i32,
    pub current_point: i64,
    pub point_types_id: i64,
    pub point_types_name: String,
    pub achievements_id: i64,
    pub achievements_name: String,
    pub achievements_type: i32,
    pub badge: i32,
    pub old_competition: bool,
}

pub type RewardsHistory = Vec<Reward>;

impl super::Endpoint for RewardsHistory {
    const PATH: &'static str = "/dashboard/progress/activity";
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum PointType {
    #[serde(rename = "DIAMOND")]
    Diamond,
    #[serde(rename = "COIN")]
    Coin,
}

impl PointType {
    pub fn translation(&self) -> &'static str {
        match self {
            PointType::Diamond => "Топкоины",
            PointType::Coin => "Топгемы",
        }
    }

    pub fn from_code(code: &str) -> Option<Self> {
        match code {
            "DIAMOND" => Some(PointType::Diamond),
            "COIN" => Some(PointType::Coin),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Achievement {
    #[serde(rename = "EVALUATION_LESSON_MARK")]
    EvaluationLessonMark,
    #[serde(rename = "PAIR_VISIT")]
    PairVisit,
    #[serde(rename = "WORK_IN_CLASS")]
    WorkInClass,
    #[serde(rename = "ASSESMENT")]
    Assesment,
    #[serde(rename = "HOMETASK_INTIME")]
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
    #[serde(rename = "FILL_IN_PROFILE")]
    FillInProfile,
    #[serde(rename = "EMAIL_CONFIRMATION")]
    EmailConfirmation,
    #[serde(rename = "AUTO_MARK_EXPIRED_HOMEWORK")]
    AutoMarkExpiredHomework,
    #[serde(rename = "SURVEY")]
    Survey,
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
        }
    }

    pub fn from_code(code: &str) -> Option<Self> {
        match code {
            "EVALUATION_LESSON_MARK" => Some(Achievement::EvaluationLessonMark),
            "PAIR_VISIT" => Some(Achievement::PairVisit),
            "WORK_IN_CLASS" => Some(Achievement::WorkInClass),
            "ASSESMENT" => Some(Achievement::Assesment),
            "HOMETASK_INTIME" => Some(Achievement::HometaskIntime),
            "20_VISITS_WITHOUT_DELAY" => Some(Achievement::Visits20WithoutDelay),
            "20_VISITS_WITHOUT_GAP" => Some(Achievement::Visits20WithoutGap),
            "10_VISITS_WITHOUT_DELAY" => Some(Achievement::Visits10WithoutDelay),
            "10_VISITS_WITHOUT_GAP" => Some(Achievement::Visits10WithoutGap),
            "5_VISITS_WITHOUT_DELAY" => Some(Achievement::Visits5WithoutDelay),
            "5_VISITS_WITHOUT_GAP" => Some(Achievement::Visits5WithoutGap),
            "FILL_IN_PROFILE" => Some(Achievement::FillInProfile),
            "EMAIL_CONFIRMATION" => Some(Achievement::EmailConfirmation),
            "AUTO_MARK_EXPIRED_HOMEWORK" => Some(Achievement::AutoMarkExpiredHomework),
            "SURVEY" => Some(Achievement::Survey),
            _ => None,
        }
    }
}
