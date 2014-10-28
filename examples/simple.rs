extern crate schedule_gen;

use schedule_gen::{ Date, GameWeekday, IdAndName, GameTime, Time };

fn main() {

    let thing = schedule_gen::LeagueSpec {
        teams: vec![
            IdAndName { id: "1".to_string(), name: "team1".to_string() },
            IdAndName { id: "2".to_string(), name: "team2".to_string() },
            IdAndName { id: "3".to_string(), name: "team3".to_string() },
            IdAndName { id: "4".to_string(), name: "team4".to_string() },
            IdAndName { id: "5".to_string(), name: "team5".to_string() },
            IdAndName { id: "6".to_string(), name: "team6".to_string() },
            IdAndName { id: "7".to_string(), name: "team7".to_string() }
        ],
        locations: vec![
            IdAndName { id: "1".to_string(), name: "name1".to_string() },
            IdAndName { id: "2".to_string(), name: "name2".to_string() }
        ],
        start_date: Date {
            day: 16,
            month: 9,
            year: 2014
        },
        end_date: Date {
            day: 4,
            month: 11,
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
                    location_ids: vec![ "1".to_string(), "2".to_string() ]
                },
                GameTime {
                    time: Time {
                        hour: 17,
                        min: 0
                    },
                    location_ids: vec![ "1".to_string() ]
                }
            ]
        }
    };

    //println!("errors:\n{}", schedule_gen::validate(&thing));
    match schedule_gen::generate_games(&thing) {
        Ok(games) => {
            println!("Success:");
            //for game in games.iter() {
                //println!("{}", game);
            //}
        }
        Err(errors) => {
            println!("Errors:");
            println!("{}", errors);
        }
    }
}
