use chrono::{NaiveDate, NaiveDateTime, format::{DelayedFormat, StrftimeItems}};

pub trait DateTimeFormatExt {
    fn from_rus(fmt: &str) -> Self;
    fn to_rus(&self) -> DelayedFormat<StrftimeItems<'_>>;
}

impl DateTimeFormatExt for NaiveDateTime {
    fn from_rus(fmt: &str) -> NaiveDateTime {
        NaiveDateTime::parse_from_str(fmt, "%Y-%m-%d %H:%M:%S")
            .unwrap_or(NaiveDateTime::MIN)
    }
    fn to_rus(&self) -> DelayedFormat<StrftimeItems<'_>> {
        self.format("%d.%m.%Y %H:%M")
    }
}

impl DateTimeFormatExt for NaiveDate {
    fn from_rus(fmt: &str) -> NaiveDate {
        NaiveDate::parse_from_str(fmt, "%Y-%m-%d")
            .unwrap_or(NaiveDate::MIN)
    }
    fn to_rus(&self) -> DelayedFormat<StrftimeItems<'_>>{
        self.format("%d.%m.%Y")
    }
}