pub mod utils_base {
    use chrono::{DateTime, NaiveDate, ParseError, TimeZone, Utc};

    pub fn date_format(input_data: &str) -> Result<DateTime<Utc>, ParseError> {
        let naive_date = NaiveDate::parse_from_str(input_data, "%d.%m.%Y")?;

        let naive_datetime = naive_date.and_hms_opt(23, 59, 0).unwrap();

        let date = Utc.from_utc_datetime(&naive_datetime);

        Ok(date)
    }
}
