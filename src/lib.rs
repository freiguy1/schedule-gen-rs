#![crate_name = "schedule_gen"]
#![allow(unstable)]

extern crate uuid;
extern crate chrono;

use uuid::Uuid;

use chrono::Datelike;

use std::rand::{thread_rng, Rng};

use contract::{ Time, Date, LeagueSpec, TeamEvent };
use contract::TeamEvent::{ Game, Bye };
use convert::{ DateConvert, WeekdayConvert };

mod convert;
mod validate;
pub mod contract;


#[derive(Clone)]
struct GameShell {
    pub date: Date,
    pub time: Time,
    pub location: (String, String)
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
        teams.push((generated_uuid, String::from_str("Bye team")));
    }
    let teams = teams;
    let bye_id_opt: Option<String> = bye_id_opt;

    let mut round_robin = generate_round_robin(&teams);

    let mut result: Vec<TeamEvent> = vec!();

    for rotation in game_shells.as_slice().chunks((teams.len() - 1) * (spec.teams.len() / 2)) {
        shuffle_round_robin(&mut round_robin);
        for shells_teams in rotation.as_slice().chunks(spec.teams.len() / 2).zip(round_robin.iter()) {
            let (shells, teams) = shells_teams;
            let non_byes = match bye_id_opt {
                Some(ref bye_id) => {
                    let bye_date = shells[0].date.clone();
                    let bye_pair = teams.iter().find(|pair| (pair.0).0 == *bye_id || (pair.1).0 == *bye_id).unwrap();
                    if (bye_pair.0).0 == *bye_id {
                        result.push(Bye(bye_pair.1.clone(), bye_date));
                    } else {
                        result.push(Bye(bye_pair.0.clone(), bye_date));
                    }
                    teams.iter().filter(|pair| (pair.0).0 != *bye_id && (pair.1).0 != *bye_id).collect::<Vec<_>>()
                }
                None => teams.iter().collect::<Vec<_>>()
            };
            for shell_team in shells.iter().zip(non_byes.iter()) {
                let (shell, team) = shell_team;
                result.push(Game(
                    team.0.clone(),
                    team.1.clone(),
                    shell.date.clone(),
                    shell.time.clone(),
                    shell.location.clone()));
            }
        }
    }

    Ok(result)
}

fn shuffle_round_robin(
    round_robin: &mut Vec<Vec<(&(String, String), &(String, String))>>) {
    let mut rng = thread_rng();
    let pair: (&(String, String), &(String, String)) = round_robin[round_robin.len() - 1][0];
    rng.shuffle(round_robin.as_mut_slice());
    while round_robin[0].iter().any(|pair2|
        ((pair2.0).0 == (pair.0).0 &&
        (pair2.1).0 == (pair.1).0) ||
        ((pair2.1).0 == (pair.0).0 &&
        (pair2.0).0 == (pair.1).0)) {
        rng.shuffle(round_robin.as_mut_slice());
    }

    for week in round_robin.iter_mut() {
        rng.shuffle(week.as_mut_slice());
    }
}

fn generate_round_robin(teams: &Vec<(String, String)>) -> Vec<Vec<(&(String, String), &(String, String))>> {
    let static_team: &(String, String) = &teams[0];
    let mut other_teams: Vec<&(String, String)> = teams.iter().skip(1).collect();

    let mut result: Vec<Vec<(&(String, String), &(String, String))>> = Vec::new();

    for _ in range(0us, teams.len() - 1) {
        let mut session: Vec<(&(String, String), &(String, String))> = Vec::new();
        session.push((other_teams[0], static_team));
        for i in range(0us, (other_teams.len() - 1) / 2) {
            let team_1: &(String, String) = other_teams[i+1];
            let team_2: &(String, String) = other_teams[other_teams.len() - 1 - i];
            session.push((team_1, team_2));
        }

        // Rotate other_teams
        let temp = other_teams[0];
        for i in range(0us, other_teams.len() - 1) {
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
                            date: DateConvert::from_naive_date(&i_date),
                            time: time.time,
                            location: spec.locations.iter().find(|loc| loc.0 == *location).unwrap().clone()
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
