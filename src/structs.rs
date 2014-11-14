
use chrono::Weekday as ChronoWeekday;

use std::fmt::{ Show, Formatter, FormatError };

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

#[deriving(Show, Eq, PartialEq, Clone)]
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
            Sunday => ::chrono::Sun,
            Monday => ::chrono::Mon,
            Tuesday => ::chrono::Tue,
            Wednesday => ::chrono::Wed,
            Thursday => ::chrono::Thu,
            Friday => ::chrono::Fri,
            Saturday => ::chrono::Sat
        }
    }

    
    pub fn from_chrono_weekday(chrono_weekday: ChronoWeekday) -> Weekday {
        match chrono_weekday {
            ::chrono::Sun => Sunday,
            ::chrono::Mon => Monday,
            ::chrono::Tue => Tuesday,
            ::chrono::Wed => Wednesday,
            ::chrono::Thu => Thursday,
            ::chrono::Fri => Friday,
            ::chrono::Sat => Saturday
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
        write!(f, "{}:{:02} {:04}-{:02}-{:02} on {} (id: {})",
            self.time.hour, self.time.min,
            self.date.year, self.date.month, self.date.day,
            self.location.name, self.location.id)
    }
}

pub enum TeamEvent {
    Game(IdAndName, IdAndName, Date, Time, IdAndName),
    Bye(IdAndName, Date)
}
