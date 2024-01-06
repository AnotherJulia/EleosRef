use std::collections::HashMap;
use serde_json::Value;
use crate::models::team::Team;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Data {
    members: u32,
    points: i32,
}

pub fn extract_team_data(path: &str, n_matches: i32) -> Vec<Team> {

    // Get the JSON team data
    let file_content = std::fs::read_to_string(path).expect("Unable to read file");
    let teams_data: HashMap<String, Data> = serde_json::from_str(&file_content).unwrap();

    let mut teams: Vec<Team> = Vec::new();

    for (name, team) in teams_data {
        let t = Team {
            name,
            members: team.members,
            points: team.points,
            turns_needed: 0,
        };

        teams.push(t);
    }


    // Calculat total points deficit (consider only negative points)
    let total_deficit: i32 = teams.iter()
        .filter(|team| team.points < 0)
        .map(|team| team.points)
        .sum::<i32>().abs();

    println!("Total Point Deficit Across Teams: {}", total_deficit);

    // Calculate the point sper turn
    let points_per_turn = if total_deficit > 0 && n_matches > 0 {
        (total_deficit as f32 / n_matches as f32).ceil() as i32
    } else {
        0
    };

    println!("Points per turn: {}", points_per_turn);

    for team in &mut teams {
        if team.points < 0 {
            team.turns_needed = (-team.points / points_per_turn).max(4);
        } else {
            team.turns_needed = 4
        }
    }

    let mut total_turns: i32 = teams.iter().map(|team| team.turns_needed).sum();
    let mut adjustment = n_matches - total_turns;
    while adjustment != 0 {
        println!("Running - adjustment: {}", adjustment);
        for team in &mut teams {
            if adjustment > 0 && team.turns_needed < (-team.points / points_per_turn) {
                team.turns_needed += 1;
                adjustment -= 1;
            } else if adjustment < 0 && team.turns_needed > 4 {
                team.turns_needed -= 1;
                adjustment += 1;
            }
        }
        total_turns = teams.iter().map(|team| team.turns_needed).sum();
        adjustment = n_matches - total_turns;
    }

    // Lets test if the n_matches is equal to total turned
    assert_eq!(total_turns, n_matches);


    teams
}


