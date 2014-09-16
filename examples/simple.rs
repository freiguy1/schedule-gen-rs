extern crate schedule_gen;

use schedule_gen::{ Date, GameWeekday, IdAndName, GameTime, Time };

fn main() {
    println!("Hello world!");

    let thing = schedule_gen::LeagueSpec {
        teams: vec![],
        locations: vec![
            IdAndName { id: "1", name: "name1" },
            IdAndName { id: "2", name: "name2" },
            IdAndName { id: "3", name: "name3" }
        ],
        start_date: Date {
            day: 16,
            month: 9,
            year: 2014
        },
        end_date: Date {
            day: 24,
            month: 9,
            year: 2014
        },
        game_weekdays: vec![
            GameWeekday {
                day: schedule_gen::Tuesday,
                game_times: vec![
                    GameTime {
                        time: Time {
                            hour: 16,
                            min: 0
                        },
                        location_ids: vec![ "1", "2" ]
                    },
                    GameTime {
                        time: Time {
                            hour: 17,
                            min: 0
                        },
                        location_ids: vec![ "1", "3" ]
                    },
                    GameTime {
                        time: Time {
                            hour: 18,
                            min: 0
                        },
                        location_ids: vec![ "2", "3" ]
                    }
                ]
            },
            GameWeekday {
                day: schedule_gen::Thursday,
                game_times: vec![
                    GameTime {
                        time: Time {
                            hour: 16,
                            min: 0
                        },
                        location_ids: vec![ "1", "2" ]
                    },
                    GameTime {
                        time: Time {
                            hour: 17,
                            min: 0
                        },
                        location_ids: vec![ "1", "3" ]
                    },
                    GameTime {
                        time: Time {
                            hour: 18,
                            min: 0
                        },
                        location_ids: vec![ "2", "3" ]
                    }
                ]
            }
        ]
    };

    println!("errors:\n{}", schedule_gen::validate(thing));
}
