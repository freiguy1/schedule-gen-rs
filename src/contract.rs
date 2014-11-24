
pub struct LeagueSpec {
    pub teams: Vec<(String, String)>,
    pub locations: Vec<(String, String)>,
    pub start_date: Date,
    pub end_date: Date,
    pub game_weekday: GameWeekday
}

#[deriving(Show, Clone)]
pub struct Time {
    pub hour: u8,
    pub min: u8
}

#[deriving(Show, Eq, PartialEq, Clone, PartialOrd, Ord)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8
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

pub enum TeamEvent {
    Game((String, String), (String, String), Date, Time, (String, String)),
    Bye((String, String), Date)
}

impl TeamEvent {
    pub fn get_date(&self) -> Date {
        match *self {
            TeamEvent::Game(_, _, date, _, _) => date,
            TeamEvent::Bye(_, date) => date
        }
    }
}
