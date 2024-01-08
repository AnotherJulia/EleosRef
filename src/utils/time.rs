use chrono::{NaiveTime};

pub struct ExcelTimeConverter;

impl ExcelTimeConverter {
    pub fn convert(excel_time_num: &f64) -> NaiveTime {
        let total_seconds_in_day = (excel_time_num.fract() * 24.0 * 60.0 * 60.0).round() as u32;
        let hours = total_seconds_in_day / 3600;
        let minutes = (total_seconds_in_day % 3600) / 60;
        let seconds = total_seconds_in_day % 60;
        NaiveTime::from_hms_opt(hours, minutes, seconds).unwrap()
    }
}