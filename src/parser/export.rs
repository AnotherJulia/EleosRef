use serde::Serialize;

use csv::Writer;
use std::error::Error;
use std::fs::File;
use crate::scheduler::scheduler::ScheduleResults;

#[derive(Serialize)]
struct RefereeScheduleRecord {
    match_date: String,
    match_time: String,
    home_team: String,
    away_team: String,
    referee_team: String,
    // Add other fields as necessary
}


pub fn prepare_and_write_schedule(schedule_results: ScheduleResults, file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_writer(File::create(file_path)?);

    for match_details in schedule_results.schedule {
        let referee_team_name = match_details.first_ref; // Assuming first_ref holds the referee team's name
        let record = RefereeScheduleRecord {
            match_date: match_details.date.format("%Y-%m-%d").to_string(),
            match_time: match_details.time.format("%H:%M").to_string(),
            home_team: match_details.home_team,
            away_team: match_details.away_team,
            referee_team: referee_team_name,
        };

        wtr.serialize(record).expect("unable to write export");
    }

    wtr.flush()?;
    Ok(())
}

fn write_schedule_to_csv(schedule: Vec<RefereeScheduleRecord>, file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(file_path)?;

    for record in schedule {
        wtr.serialize(record)?;
    }

    wtr.flush()?;
    Ok(())
}
