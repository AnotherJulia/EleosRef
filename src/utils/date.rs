use chrono::{Duration, NaiveDate};

pub struct ExcelDateConverter;

impl ExcelDateConverter {
    pub fn convert(excel_date_num: &f64) -> NaiveDate {
        let days_since_base_date = excel_date_num.trunc() as i64;
        let base_date = NaiveDate::from_ymd_opt(1899, 12, 30).unwrap(); // Adjust as per your Excel's date system
        base_date + Duration::days(days_since_base_date)
    }
}