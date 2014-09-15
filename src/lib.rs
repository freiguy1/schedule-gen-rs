#![crate_name = "schedule_gen"]

extern crate chrono;

use chrono::{ NaiveDate, Datelike };
use chrono::Weekday as ChronoWeekday;

use std::collections::HashSet;

pub struct LeagueSpec {
    pub teams: Vec<IdAndName>,
    pub locations: Vec<IdAndName>,
    pub start_date: Date,
    pub end_date: Date,
    pub game_weekdays: Vec<GameWeekday>
}


pub struct IdAndName {
    pub id: &'static str,
    pub name: &'static str
}

pub struct Time {
    pub hour: u8,
    pub min: u8
}

pub struct Date {
    pub day: u8,
    pub month: u8,
    pub year: u16
}

pub struct GameWeekday {
    pub day: Weekday,
    pub game_times: Vec<GameTime>
}

pub struct GameTime {
    pub time: Time,
    pub location_ids: Vec<&'static str>
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
        match chrono_weekday {
            chrono::Sun => Sunday,
            chrono::Mon => Monday,
            chrono::Tue => Tuesday,
            chrono::Wed => Wednesday,
            chrono::Thu => Thursday,
            chrono::Fri => Friday,
            chrono::Sat => Saturday
        }
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

    let good_start_date = spec.game_weekdays.iter()
        .any(|game_weekdays| game_weekdays.day.to_chrono_weekday() == start_date.weekday());
    let good_end_date = spec.game_weekdays.iter()
        .any(|game_weekdays| game_weekdays.day.to_chrono_weekday() == end_date.weekday());

    if !good_start_date {
        println!("The start date does not occur on a day of the week listed.");
    }
    if !good_end_date {
        println!("The end date does not occur on a day of the week listed.");
    }

    //Make sure all locations are used at least once
    let mut used_locations: HashSet<&str> = HashSet::new();
    for game_weekday in spec.game_weekdays.iter() {
        for game_time in game_weekday.game_times.iter() {
            for location_id in game_time.location_ids.iter() {
                used_locations.insert(location_id.clone());
            }
        }
    }

    if used_locations.ne(&spec.locations.iter().map(|x| x.id.clone()).collect()) {
        println!("Locations used in game_weekdays are not equal to the list of locations");
    }

}
