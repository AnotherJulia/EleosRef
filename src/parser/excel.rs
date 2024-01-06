use chrono::{NaiveDate, NaiveTime};
use office::Excel;
use crate::models::matches::{ExcelMatch, Match};
use crate::utils::time::{ExcelTimeConverter};
use crate::utils::date::{ExcelDateConverter};

pub fn extract_match_details_from_sheet(path: &str, sheet: &str) -> Result<Vec<Match>, String> {
    let mut matches: Vec<Match> = Vec::new();
    let mut excel = Excel::open(path).map_err(|e| e.to_string())?;
    let r = excel.worksheet_range(sheet).map_err(|e| e.to_string())?;

    for row in r.rows().skip(1) {
        // Extract data from columns with helper functions
        let date_str = parse_date(row.get(0));
        let time_str = parse_time(row.get(1));
        let home_str = parse_string(row.get(2));
        let out_str = parse_string(row.get(3));
        let location_str = parse_location(row.get(4));
        let field_int = parse_field(row.get(5));
        let first_ref = parse_referee(row.get(12));
        let second_ref = parse_referee(row.get(13));

        let match_details = Match {
            date: date_str,
            time: time_str,
            home_team: home_str,
            away_team: out_str,
            location: location_str,
            field: field_int,
            first_ref,
            second_ref,
        };
        matches.push(match_details);
    }
    Ok(matches) // Wrap the vector in `Ok`
}



fn parse_string(cell: Option<&office::DataType>) -> String {
    match cell {
        Some(office::DataType::String(s)) => s.clone(),
        _ => panic!("Unexpected type in colum")
    }
}

fn parse_date(cell: Option<&office::DataType>) -> NaiveDate {
    let default_date: NaiveDate = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
    match cell {
        Some(office::DataType::Float(f)) => {
            ExcelDateConverter::convert(*f)
        },
        _ => default_date,
    }
}

fn parse_time(cell: Option<&office::DataType>) -> NaiveTime {
    let default_time: NaiveTime = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    match cell {
        Some(office::DataType::Float(f)) => {
            ExcelTimeConverter::convert(*f)
        },
        _ => default_time
    }
}


fn parse_location(cell: Option<&office::DataType>) -> String {
    match cell {
        Some(office::DataType::String(s)) => s.clone(),
        Some(office::DataType::Empty) => panic!("Location column cannot be empty"),
        _ => panic!("Unexpected type in colum Locatie"),
    }
}

fn parse_field(cell: Option<&office::DataType>) -> i32 {
    match cell {
        Some(office::DataType::Float(f)) => *f as i32,
        Some(office::DataType::String(_)) => 0,
        _ => panic!("Unexpected type in colum Veld")
    }
}

fn parse_referee(cell: Option<&office::DataType>) -> String {
    match cell {
        Some(office::DataType::String(s)) => s.clone(),
        Some(office::DataType::Empty) => "".to_string(),
        _ => panic!("Unexpected type in colum Scheidsrechter")
    }
}

