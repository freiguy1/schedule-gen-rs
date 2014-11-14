
use chrono::{ NaiveTime, NaiveDate, Datelike };
use chrono::Weekday as ChronoWeekday;

use contract::{ Weekday, Time, Date };

pub trait DateConvert {
    fn to_naive_date_opt(self) -> Option<NaiveDate>;
    fn from_naive_date(naive_date: &NaiveDate) -> Date;
}

impl DateConvert for Date {

    fn to_naive_date_opt(self) -> Option<NaiveDate> {
        NaiveDate::from_ymd_opt(
            self.year as i32,
            self.month as u32,
            self.day as u32
        )
    }

    fn from_naive_date(naive_date: &NaiveDate) -> Date {
        Date {
            year: naive_date.year() as u16,
            month: naive_date.month() as u8,
            day: naive_date.day() as u8
        }
    }
}

pub trait TimeConvert {
    fn to_naive_time_opt(self) -> Option<NaiveTime>;
}

impl TimeConvert for Time {
    fn to_naive_time_opt(self) -> Option<NaiveTime> {
        NaiveTime::from_hms_opt(self.hour as u32, self.min as u32, 0)
    }
}

pub trait WeekdayConvert {
    fn to_chrono_weekday(&self) -> ChronoWeekday;
    fn from_chrono_weekday(chrono_weekday: ChronoWeekday) -> Weekday;
}

impl WeekdayConvert for Weekday {
    fn to_chrono_weekday(&self) -> ChronoWeekday {
        match *self {
            ::contract::Sunday => ::chrono::Sun,
            ::contract::Monday => ::chrono::Mon,
            ::contract::Tuesday => ::chrono::Tue,
            ::contract::Wednesday => ::chrono::Wed,
            ::contract::Thursday => ::chrono::Thu,
            ::contract::Friday => ::chrono::Fri,
            ::contract::Saturday => ::chrono::Sat
        }
    }

    fn from_chrono_weekday(chrono_weekday: ChronoWeekday) -> Weekday {
        match chrono_weekday {
            ::chrono::Sun => ::contract::Sunday,
            ::chrono::Mon => ::contract::Monday,
            ::chrono::Tue => ::contract::Tuesday,
            ::chrono::Wed => ::contract::Wednesday,
            ::chrono::Thu => ::contract::Thursday,
            ::chrono::Fri => ::contract::Friday,
            ::chrono::Sat => ::contract::Saturday
        }
    }
}
