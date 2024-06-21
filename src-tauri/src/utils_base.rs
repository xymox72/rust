pub mod utils_base{


    use chrono::{DateTime, NaiveDate, ParseError, TimeZone, Utc};

    pub fn date_format(input_data: &str) -> Result<DateTime<Utc>, ParseError>{
        let naive_date = NaiveDate::parse_from_str(input_data, "%d.%m.%Y")?;
        let date = Utc.from_utc_date(&naive_date).and_hms(0, 0, 0);
  
        Ok(date)
    }
}