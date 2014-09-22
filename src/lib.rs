#![crate_name = "schedule_gen"]

extern crate chrono;

use chrono::{ NaiveDate, NaiveTime, Datelike };
use chrono::Weekday as ChronoWeekday;

use std::fmt::{ Show, Formatter, FormatError };

mod validate;

pub struct LeagueSpec {
    pub teams: Vec<IdAndName>,
    pub locations: Vec<IdAndName>,
    pub start_date: Date,
    pub end_date: Date,
    pub game_weekday: GameWeekday
}


#[deriving(Show)]
pub struct IdAndName {
    pub id: &'static str,
    pub name: &'static str
}

#[deriving(Show)]
pub struct Time {
    pub hour: u8,
    pub min: u8
}

impl Time {
    fn to_naive_time_opt(self) -> Option<NaiveTime> {
        NaiveTime::from_hms_opt(self.hour as u32, self.min as u32, 0)
    }
}

#[deriving(Show)]
pub struct Date {
    pub day: u8,
    pub month: u8,
    pub year: u16
}

impl Date {
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

pub struct GameShell {
    pub date: Date,
    pub time: Time,
    pub location: IdAndName
}

impl Show for GameShell {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        write!(f, "{}:{:02} {:04}-{:02}-{:02} at {} (id: {})",
            self.time.hour, self.time.min,
            self.date.year, self.date.month, self.date.day,
            self.location.name, self.location.id)
    }
}


pub fn generate_games(spec: &LeagueSpec) -> Result<Vec<GameShell>, Vec<&'static str>> {
    let errors = validate::validate(spec);
    if errors.len() > 0 {
        return Err(errors);
    }

    let mut result: Vec<GameShell> = vec![];

    let start_date = spec.start_date.to_naive_date_opt().unwrap();
    let end_date = spec.end_date.to_naive_date_opt().unwrap();

    let mut i_date = start_date.clone();
    while i_date != end_date.succ() {
        if spec.game_weekday.day.to_chrono_weekday() == i_date.weekday() {
            for time in spec.game_weekday.game_times.iter() {
                for location in time.location_ids.iter() {
                    result.push( GameShell {
                        date: Date::from_naive_date(&i_date),
                        time: time.time,
                        location: *spec.locations.iter().find(|loc| loc.id == *location).unwrap()
                    });
                }
            }
            println!("i_date: {}", i_date);
        }
        i_date = i_date.succ();
    }

    Ok(result)
}


