extern crate schedule_gen;

use schedule_gen::{ Date, GameWeekday, IdAndName, GameTime, Time };

fn main() {

    let thing = schedule_gen::LeagueSpec {
        teams: vec![
            IdAndName { id: "1", name: "team1" },
            IdAndName { id: "2", name: "team2" },
            IdAndName { id: "3", name: "team3" },
            IdAndName { id: "4", name: "team4" },
            IdAndName { id: "5", name: "team5" },
            IdAndName { id: "6", name: "team6" }
        ],
        locations: vec![
            IdAndName { id: "1", name: "name1" },
            IdAndName { id: "2", name: "name2" }
        ],
        start_date: Date {
            day: 16,
            month: 9,
            year: 2014
        },
        end_date: Date {
            day: 23,
            month: 9,
            year: 2014
        },
        game_weekday: GameWeekday {
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
                    location_ids: vec![ "1" ]
                }
            ]
        }
    };

    //println!("errors:\n{}", schedule_gen::validate(&thing));
    match schedule_gen::generate_games(&thing) {
        Ok(games) => {
            println!("Success:");
            for game in games.iter() {
                println!("{}", game);
            }
        }
        Err(errors) => {
            println!("Errors:");
            println!("{}", errors);
        }
    }
}
