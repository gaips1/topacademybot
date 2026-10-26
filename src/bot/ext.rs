use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

pub trait DateTimeFormatExt {
    fn from_rus(fmt: &str) -> NaiveDateTime;
    fn to_rus(&self) -> String;
    fn to_rus_date(&self) -> String;
}

impl DateTimeFormatExt for NaiveDateTime {
    fn from_rus(fmt: &str) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(fmt, "%Y-%m-%d %H:%M:%S")
            .or_else(|_| {
                NaiveDate::parse_from_str(fmt, "%Y-%m-%d").map(|d| d.and_time(NaiveTime::MIN))
            })
            .unwrap_or(NaiveDateTime::MIN)
    }
    fn to_rus(&self) -> String {
        self.format("%d.%m.%Y %H:%M").to_string()
    }
    fn to_rus_date(&self) -> String {
        self.format("%d.%m.%Y").to_string()
    }
}
