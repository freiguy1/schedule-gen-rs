#![crate_name = "schedule_gen"]

extern crate chrono;
extern crate uuid;

use chrono::{ NaiveDate, NaiveTime, Datelike };
use chrono::Weekday as ChronoWeekday;

use uuid::Uuid;

use std::fmt::{ Show, Formatter, FormatError };
use std::rand::{task_rng, Rng};


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


pub fn generate_games(spec: &LeagueSpec) -> Result<Vec<TeamEvent>, Vec<&'static str>> {
    let errors = validate::validate(spec);
    if errors.len() > 0 {
        return Err(errors);
    }

    let game_shells = generate_shells(spec);

    // Create a new list of teams which includes our fake bye team if needed
    let mut teams = spec.teams.clone();
    let mut bye_id_opt: Option<String> = None;
    let generated_uuid = Uuid::new_v4().to_hyphenated_string();
    if teams.len() % 2 == 1 {
        bye_id_opt = Some(generated_uuid.clone());
        teams.push(IdAndName {
            name: String::from_str("Bye team"),
            id: generated_uuid
        });
    }
    let teams = teams;
    let bye_id_opt: Option<String> = bye_id_opt;

    let mut round_robin = generate_round_robin(&teams);
    let mut rng = task_rng();

    for rotation in game_shells.as_slice().chunks((teams.len() - 1) * (spec.teams.len() / 2)) {
        println!("New Rotation");
        rng.shuffle(round_robin.as_mut_slice());
        for shells_teams in rotation.as_slice().chunks(spec.teams.len() / 2).zip(round_robin.iter_mut()) {
            let (shells, teams) = shells_teams;
            rng.shuffle(teams.as_mut_slice());
            println!("\tNew Week");
            let non_byes = match bye_id_opt {
                Some(ref bye_id) => {
                    let bye_pair = teams.iter().find(|pair| pair.val0().id == *bye_id || pair.val1().id == *bye_id).unwrap();
                    if bye_pair.val0().id == *bye_id {
                        println!("\t\t{0} has bye", bye_pair.val1());
                    } else {
                        println!("\t\t{0} has bye", bye_pair.val0());
                    }
                    teams.iter().filter(|pair| pair.val0().id != *bye_id && pair.val1().id != *bye_id).collect::<Vec<_>>()
                }
                None => teams.iter().collect::<Vec<_>>()
            };
            for shell_team in shells.iter().zip(non_byes.iter()) {
                let (shell, team) = shell_team;
                println!("\t\t{0}|{1}", shell, team)
            }
        }
    }

    Ok(vec!())
}

fn generate_round_robin(teams: &Vec<IdAndName>) -> Vec<Vec<(&IdAndName, &IdAndName)>> {
    let static_team: &IdAndName = &teams[0];
    let mut other_teams: Vec<&IdAndName> = teams.iter().skip(1).collect();

    let mut result: Vec<Vec<(&IdAndName, &IdAndName)>> = Vec::new();

    for _ in range(0u, teams.len() - 1) {
        let mut session: Vec<(&IdAndName, &IdAndName)> = Vec::new();
        session.push((other_teams[0], static_team));
        for i in range(0u, (other_teams.len() - 1) / 2) {
            let team_1: &IdAndName = other_teams[i+1];
            let team_2: &IdAndName = other_teams[other_teams.len() - 1 - i];
            session.push((team_1, team_2));
        }

        // Rotate other_teams
        let temp = other_teams[0];
        for i in range(0u, other_teams.len() - 1) {
            other_teams[i] = other_teams[i + 1];
        }
        let other_teams_len = other_teams.len() - 1;
        other_teams[other_teams_len] = temp;

        result.push(session)
    }

    result
}

fn generate_shells(spec: &LeagueSpec) -> Vec<GameShell> {
    let games_per_night = spec.teams.len() as u8 / 2;

    let mut result: Vec<GameShell> = vec![];

    let start_date = spec.start_date.to_naive_date_opt().unwrap();
    let end_date = spec.end_date.to_naive_date_opt().unwrap();

    let mut i_date = start_date.clone();
    while i_date != end_date.succ() {
        if spec.game_weekday.day.to_chrono_weekday() == i_date.weekday() {
            let mut num_games = 0u8;
            for time in spec.game_weekday.game_times.iter() {
                for location in time.location_ids.iter() {
                    if num_games != games_per_night {
                        result.push( GameShell {
                            date: Date::from_naive_date(&i_date),
                            time: time.time,
                            location: spec.locations.iter().find(|loc| loc.id == *location).unwrap().clone()
                        });
                        num_games += 1;
                    }
                }
            }
        }
        i_date = i_date.succ();
    }

    result
}
