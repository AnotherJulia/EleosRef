use std::collections::HashMap;
use crate::models::team::Team;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Data {
    pub points: i32,
    pub no_ref_teams: Option<Vec<String>>,
    pub no_ref_groups: Option<Vec<String>>,
    pub no_ref_filters: Option<Vec<String>>,
}

// TODO: Refactor -> Extract out Functions
pub fn extract_team_data(path: &str, n_matches: i32) -> Vec<Team> {

    // Get the JSON team data
    let file_content = std::fs::read_to_string(path).expect("Unable to read file");

    // Create a hashmap out of the data
    let teams_data: HashMap<String, Data> = serde_json::from_str(&file_content).unwrap(); // String: TeamName, Data: {Members, Points}

    // Assign each of the teams to a Team object
    let mut teams: Vec<Team> = Vec::new();
    for (name, team) in teams_data {
        let t = Team {
            name,
            points: team.points,
            turns_needed: 0,
            no_ref_teams: team.no_ref_teams.unwrap_or(Vec::new()),
            no_ref_groups: team.no_ref_groups.unwrap_or(Vec::new()),
            no_ref_filters: team.no_ref_filters.unwrap_or(Vec::new()),
        };

        teams.push(t);
    }

    // Calculate total points deficit (consider only negative points)
    let total_deficit: i32 = teams.iter()
        .filter(|team| team.points < 0)
        .map(|team| team.points)
        .sum::<i32>().abs();

    println!("Total Point Deficit Across Teams: {}", total_deficit);

    // Calculate the points per referee turn
    let points_per_turn = if total_deficit > 0 && n_matches > 0 {
        (total_deficit as f32 / n_matches as f32).ceil() as i32
    } else {
        0
    };

    println!("Points per turn: {}", points_per_turn);

    // Calculate how many ref. turns each team needs to get out of their point deficit
    // Each team needs to have at least "4" turns a season
    for team in &mut teams {
        if team.points < 0 {
            team.turns_needed = (-team.points / points_per_turn).max(4);
        } else {
            team.turns_needed = 4
        }
    }

    // Apply adjustments to make sure that all matches are covered (/or not too much turns are given)

    // Find total amount of turns currently divided between all teams
    let mut total_turns: i32 = teams.iter().map(|team| team.turns_needed).sum();

    // adjustment = difference between total amount of matches and turns given
    let mut adjustment = n_matches - total_turns;

    // While that adjustment is not yet 0; we continuously adjust teams turns
    while adjustment != 0 {
        println!("Running - adjustment: {}", adjustment);

        // Calculate how many turns can be added or taken from each team equally
        let per_team_adjustment = adjustment / teams.len() as i32;

        for team in &mut teams {
            if adjustment > 0 {
                team.turns_needed += 1;
                adjustment -= 1;
            } else if adjustment < 0 && team.turns_needed > 4 {
                // if adjustment is negative (more turns than matches) -> team has more than 4 turns -> we can take one away from them
                team.turns_needed -= 1;
                adjustment += 1;
            }
        }

        // After doing some adjustments, recalculate the adjustment value and loop again
        total_turns = teams.iter().map(|team| team.turns_needed).sum();
        adjustment = n_matches - total_turns;
    }

    // Make sure that we have the turns given to cover all the matches
    assert_eq!(total_turns, n_matches);

    // Return the teams data
    teams
}


