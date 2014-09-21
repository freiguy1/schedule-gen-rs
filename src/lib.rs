#![crate_name = "schedule_gen"]

extern crate chrono;

use chrono::{ NaiveDate, NaiveTime, Datelike };
use chrono::Weekday as ChronoWeekday;

use std::collections::HashSet;

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

pub fn generate_games(spec: &LeagueSpec) -> Result<Vec<GameShell>, Vec<&'static str>> {
    let errors = validate(spec);
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

#[deriving(Show)]
pub struct GameShell {
    pub date: Date,
    pub time: Time,
    pub location: IdAndName
}

fn validate(spec: &LeagueSpec) -> Vec<&'static str> {

    let mut result: Vec<&str> = Vec::new();

    let start_date_opt = spec.start_date.to_naive_date_opt();
    let end_date_opt = spec.end_date.to_naive_date_opt();

    match (start_date_opt, end_date_opt) {
        (Some(start_date), Some(end_date)) => {
            // Check for a start and end dates starting on appropriate week days
            let good_start_date = spec.game_weekday.day.to_chrono_weekday() == start_date.weekday();
            let good_end_date = spec.game_weekday.day.to_chrono_weekday() == end_date.weekday();

            if !good_start_date {
                result.push("The start date does not occur on the day of the week.");
            }
            if !good_end_date {
                result.push("The end date does not occur on the day of the week.");
            }
            if end_date.succ_opt().is_none() {
                result.push("The end date occurs too far in the future");
            }

            // Check that start date is before end date
            if start_date >= end_date {
                result.push("The start date must occur before end date.");
            }

        }
        (None, None) => {
            result.push("Start date is an invalid date");
            result.push("End date is an invalid date");
        }
        (None, _) => result.push("Start date is an invalid date"),
        (_, None) => result.push("End date is an invalid date")
    }

    // Make sure all locations are used at least once
    let mut used_locations: HashSet<&str> = HashSet::new();
    for game_time in spec.game_weekday.game_times.iter() {
        for location_id in game_time.location_ids.iter() {
            used_locations.insert(location_id.clone());
        }
    }

    if used_locations.ne(&spec.locations.iter().map(|x| x.id.clone()).collect()) {
        result.push("Locations used in game_weekday are not equal to the list of locations");
    }

    // Make sure there's at least 2 teams
    if spec.teams.len() < 2 {
        result.push("There must be at least two teams");
    }

    // Make sure there's at least one location
    if spec.locations.len() == 0 {
        result.push("There must be at least one location");
    }

    // Make sure there's at least one time for each the weekday
    // Make sure there's at least one location for each time for the weekday
    let mut has_times = spec.game_weekday.game_times.len() > 0;
    let mut has_locations = true;
    for game_time in spec.game_weekday.game_times.iter() {
        has_locations = has_locations && game_time.location_ids.len() > 0;
    }

    if !has_times {
        result.push("There must be at least one game time for the game weekday");
    }
    if !has_locations {
        result.push("There must be at least one location id for each game time for the game weekday");
    }

    // Check that times don't repeat on a given day
    let mut has_time_repeats = false;
    let set: HashSet<uint> = spec.game_weekday.game_times.iter()
        .map(|time| time.time.hour as uint * 60 + time.time.min as uint).collect();
    has_time_repeats = has_time_repeats || set.len() != spec.game_weekday.game_times.len();

    if has_time_repeats {
        result.push("There cannot be repeating game times on a particular day");
    }

    // Check for all valid times
    let mut has_invalid_times = spec.game_weekday.game_times.iter()
        .map(|time| NaiveTime::from_hms_opt(time.time.hour as u32, time.time.min as u32, 0))
        .any(|chrono_time_opt| chrono_time_opt.is_none());
    if has_invalid_times {
        result.push("All game times must be valid times");
    }

    // Check for a valid number of games per week given the team count
    let required_games_per_week = spec.teams.len() / 2;
    let actual_games_per_week = spec.game_weekday.game_times.iter()
        .fold(0, |sum, time| sum + time.location_ids.len());

    if required_games_per_week != actual_games_per_week {
        result.push(
            "There are a different number of possible 
            games per week than team matchups");
    }

    result

}

