
// #[macro_use] extern crate rocket;

mod models;
mod parser;
mod utils;

use rocket::serde::json::Json;
use crate::parser::excel::extract_match_details_from_sheet;
use crate::parser::filter::filter_matches;


// #[get("/")]
// fn hello() -> &'static str {
//     "Hello, world!"
// }
//
// #[get("/process")]
// fn run() -> Json<Vec<Match>> {
//     // Custom Parameters
//     let file_location: &str = "data/wedstrijden.xlsx";
//     let sheet_name: &str = "Wedstrijden";
//     let home_location: &str = "De Ackers, Bergschenhoek";
//
//     let m = parse_excel(file_location, sheet_name);
//     let matches = filter_matches(m, home_location);
//
//     Json(matches)
// }
//
// #[launch]
// fn rocket() -> _ {
//     rocket::build().mount("/", routes![hello, run])
// }
//

fn main() {
    let file_location: &str = "data/wedstrijden.xlsx";
    let sheet_name: &str = "Wedstrijden";
    let home_location: &str = "De Ackers, Bergschenhoek";

    let matches = extract_match_details_from_sheet(file_location, sheet_name).unwrap();
    let filtered_matches = filter_matches(matches, home_location);

    for m in filtered_matches {
        println!("{:?}", m);
    }

}
