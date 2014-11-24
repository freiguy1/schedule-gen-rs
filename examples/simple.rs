extern crate schedule_gen;

use std::collections::HashMap;

use schedule_gen::contract::{ Date, GameWeekday, GameTime, Time, TeamEvent, Game, Bye };

fn main() {

    let thing = schedule_gen::contract::LeagueSpec {
        teams: vec![
            ("1".to_string(), "team1".to_string()),
            ("2".to_string(), "team2".to_string()),
            ("3".to_string(), "team3".to_string()),
            ("4".to_string(), "team4".to_string()),
            ("5".to_string(), "team5".to_string()),
            ("6".to_string(), "team6".to_string()),
            ("7".to_string(), "team7".to_string())
        ],
        locations: vec![
            ("1".to_string(), "field1".to_string()),
            ("2".to_string(), "field2".to_string())
        ],
        start_date: Date {
            day: 16,
            month: 9,
            year: 2014
        },
        end_date: Date {
            day: 23,
            month: 12,
            year: 2014
        },
        game_weekday: GameWeekday {
            day: schedule_gen::contract::Tuesday,
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
                    location_ids: vec![ "1".to_string(), "2".to_string() ]
                }
            ]
        }
    };

    match schedule_gen::generate_games(&thing) {
        Ok(games) => {
            let games_by_date = games.iter().fold::<HashMap<&Date, Vec<&TeamEvent>>>(HashMap::new(), |map, &game| {
                if map.contains_key(&game.get_date()) {
                    println!("has date in map");
                    map.get_mut(game.get_date()).unwrap().push(game);
                } else {
                    println!("doesn't have date in map");
                    map.insert(&game.get_date(), vec![game]);
                }
                
            });
            println!("Success:");
            for game in games.iter() {
                match game {
                    &Game(ref home_team, ref away_team, ref date, ref time, ref location) => {
                        println!("Home: {0}, Away: {1}. Date: {2}. Time: {3}. Location: {4}",
                            home_team, away_team, date, time, location);
                    }
                    &Bye(ref team, ref date) => {
                        println!("Bye team: {0}, date: {1}", team, date);
                    }
                }
            }
        }
        Err(errors) => {
            println!("Errors:");
            println!("{}", errors);
        }
    }
}
