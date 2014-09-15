#![crate_name = "schedule_gen"]

extern crate chrono;

use chrono::{ NaiveDate, Datelike };
use chrono::Weekday as ChronoWeekday;


pub struct LeagueSpec {
    teams: Vec<IdAndName>,
    locations: Vec<IdAndName>,
    start_date: Date,
    end_date: Date,
    day_and_times: Vec<DayAndTimes>
}


pub struct IdAndName {
    id: String,
    name: String
}

pub struct Time {
    hour: u8,
    min: u8
}

pub struct Date {
    day: u8,
    month: u8,
    year: u16
}

pub struct DayAndTimes {
    day: Weekday,
    time_and_locations: Vec<TimeAndLocationIds>
}

pub struct TimeAndLocationIds {
    time: Time,
    location_ids: Vec<String>
}

pub enum Weekday {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday
}

impl Weekday {

    pub fn to_chrono_weekday(&self) -> ChronoWeekday {
        match *self {
            Sunday => chrono::Sun,
            Monday => chrono::Mon,
            Tuesday => chrono::Tue,
            Wednesday => chrono::Wed,
            Thursday => chrono::Thu,
            Friday => chrono::Fri,
            Saturday => chrono::Sat
        }
    }

    pub fn from_chrono_weekday(chrono_weekday: ChronoWeekday) -> Weekday {
        Sunday
    }

}

pub fn validate(spec: LeagueSpec) {
    let start_date = NaiveDate::from_ymd(
        spec.start_date.year as i32,
        spec.start_date.month as u32,
        spec.start_date.day as u32);
    let end_date = NaiveDate::from_ymd(
        spec.start_date.year as i32,
        spec.start_date.month as u32,
        spec.start_date.day as u32);

    let good_start_date = spec.day_and_times.iter()
        .any(|day_and_time| day_and_time.day.to_chrono_weekday() == start_date.weekday());
    let good_end_date = spec.day_and_times.iter()
        .any(|day_and_time| day_and_time.day.to_chrono_weekday() == end_date.weekday());

    if !good_start_date {
        println!("The start date does not occur on a day of the week listed.");
    }
    if !good_end_date {
        println!("The end date does not occur on a day of the week listed.");
    }
}
