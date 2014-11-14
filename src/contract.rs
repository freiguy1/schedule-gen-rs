

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

