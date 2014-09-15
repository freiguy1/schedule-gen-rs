extern crate schedule_gen;

use schedule_gen::{ Date, GameWeekday, IdAndName, GameTime, Time };

fn main() {
    println!("Hello world!");

    let thing = schedule_gen::LeagueSpec {
        teams: vec![],
        locations: vec![
            IdAndName { id: "1".to_string(), name: "name1".to_string() },
            IdAndName { id: "2".to_string(), name: "name2".to_string() },
            IdAndName { id: "3".to_string(), name: "name3".to_string() }
        ],
        start_date: Date {
            day: 15,
            month: 9,
            year: 2014
        },
        end_date: Date {
            day: 22,
            month: 9,
            year: 2014
        },
        game_weekdays: vec![
            GameWeekday {
                day: schedule_gen::Monday,
                game_times: vec![
                    GameTime {
                        time: Time {
                            hour: 16,
                            min: 0
                        },
                        location_ids: vec!["1".to_string(), "2".to_string()]
                    },
                    GameTime {
                        time: Time {
                            hour: 17,
                            min: 0
                        },
                        location_ids: vec!["1".to_string(), "3".to_string()]
                    },
                    GameTime {
                        time: Time {
                            hour: 18,
                            min: 0
                        },
                        location_ids: vec!["2".to_string(), "3".to_string()]
                    }
                ]
            }
        ]
    };

    schedule_gen::validate(thing);
}
