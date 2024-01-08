mod models;
mod parser;
mod utils;
mod scheduler;

use crate::models::matches::Match;
use crate::parser::excel::extract_match_details_from_sheet;
use crate::parser::filter::filter_matches;
use crate::parser::json::extract_team_data;
use crate::scheduler::scheduler::{create_schedule};

fn main() {
    let match_location: &str = "data/wedstrijden.xlsx";
    let team_location: &str = "data/teams.json";
    let sheet_name: &str = "Wedstrijden";
    let home_location: &str = "De Ackers, Bergschenhoek";

    // extract the match data from the excel sheets and filter it so it's just "home matches" with no ref
    let matches = extract_match_details_from_sheet(match_location, sheet_name).unwrap();
    let filtered_matches: Vec<Match> = filter_matches(&matches, home_location);

    let number_of_matches = filtered_matches.len() as i32;

    // let extract the team data -> finding the number of turns per team
    let teams = extract_team_data(team_location, number_of_matches);

    let schedule = create_schedule(matches, filtered_matches, teams);

    for m in schedule {
        println!("{:?}", m);
    }

}
