#![crate_name = "schedule_gen"]

extern crate chrono;
extern crate uuid;

use chrono::{ NaiveDate, NaiveTime, Datelike };
use chrono::Weekday as ChronoWeekday;

use uuid::Uuid;

use std::fmt::{ Show, Formatter, FormatError };


mod validate;

pub struct LeagueSpec {
    pub teams: Vec<IdAndName>,
    pub locations: Vec<IdAndName>,
    pub start_date: Date,
    pub end_date: Date,
    pub game_weekday: GameWeekday
}


#[deriving(Show, Clone)]
pub struct IdAndName {
    pub id: String,
    pub name: String
}

#[deriving(Show, Clone)]
pub struct Time {
    pub hour: u8,
    pub min: u8
}

impl Time {
    fn to_naive_time_opt(self) -> Option<NaiveTime> {
        NaiveTime::from_hms_opt(self.hour as u32, self.min as u32, 0)
    }
}

#[deriving(Show, Eq, PartialEq, Clone)]
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
    pub location_ids: Vec<String>
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

#[deriving(Clone)]
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

pub enum TeamEvent {
    Game(IdAndName, IdAndName, Date, Time, IdAndName),
    Bye(IdAndName, Date)
}

impl TeamEvent {
    fn date(&self) -> Date {
        match *self {
            Game(_, _, date, _, _) => date,
            Bye(_, date) => date
        }
    }
}

pub fn generate_games(spec: &LeagueSpec) -> Result<Vec<TeamEvent>, Vec<&'static str>> {
    let game_shells_result = generate_shells(spec);
    if game_shells_result.is_err() {
        return Err(game_shells_result.err().unwrap());
    }
    let game_shells = game_shells_result.ok().unwrap();

    // Create a new list of teams which includes our fake bye team if needed
    let mut teams = spec.teams.clone();
    let mut bye_id: Option<String> = None;
    let generated_uuid = Uuid::new_v4().to_hyphenated_string();
    if teams.len() % 2 == 1 {
        bye_id = Some(generated_uuid.clone());
        teams.push(IdAndName {
            name: String::from_str("Bye team"),
            id: generated_uuid
        });
    }
    let teams = teams;
    let bye_id = bye_id;
    let rotation_size = teams.len() - 1;
    let games_per_night = teams.len() / 2;

    // A set of all two team combinations
    let mut two_team_sets: Vec<(IdAndName, IdAndName)> = vec![];
    for i in range(0, teams.len()) {
        for j in range(i + 1, teams.len()) {
            two_team_sets.push((teams[i].clone(), teams[j].clone()));
        }
    }

    let mut game_shells_by_date: Vec<Vec<GameShell>> = vec!();

    let mut last_date = game_shells[0].date;
    let mut game_shell_date_builder: Vec<GameShell> = vec!();
    for game_shell in game_shells.iter() {
        if last_date != game_shell.date  {
            game_shells_by_date.push(game_shell_date_builder.clone());
            game_shell_date_builder = vec!();
            last_date = game_shell.date;
        }
        game_shell_date_builder.push(game_shell.clone());
    }
    game_shells_by_date.push(game_shell_date_builder.clone());

    // Rotations(Days(Games))
    let mut game_shells_by_rotation: Vec<Vec<Vec<GameShell>>> = vec!();
    let mut rotation_counter = 0u;
    let mut game_shell_rotation_builder: Vec<Vec<GameShell>> = vec!();
    for game_shells in game_shells_by_date.iter() {
        if rotation_counter == rotation_size {
            rotation_counter = 0;
            game_shells_by_rotation.push(game_shell_rotation_builder);
            game_shell_rotation_builder = vec!();
        }
        game_shell_rotation_builder.push(game_shells.clone());
        rotation_counter += 1;
    }

    game_shells_by_rotation.push(game_shell_rotation_builder);

    println!("Game Shells By Rotation: {}", game_shells_by_rotation);
    println!("Len: {}. thing[0].len(): {}. thing[1].len(): {})", game_shells_by_rotation.len(), game_shells_by_rotation[0].len(), game_shells_by_rotation[1].len());

    Ok(vec!())

}

fn generate_shells(spec: &LeagueSpec) -> Result<Vec<GameShell>, Vec<&'static str>> {
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
                        location: spec.locations.iter().find(|loc| loc.id == *location).unwrap().clone()
                    });
                }
            }
        }
        i_date = i_date.succ();
    }

    Ok(result)
}
