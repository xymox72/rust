pub mod utils_base {
    use chrono::{DateTime, NaiveDate, ParseError, TimeZone, Utc};
    use thiserror::Error;

    pub fn date_format(input_data: &str) -> Result<DateTime<Utc>, ParseError> {
        let naive_date = NaiveDate::parse_from_str(input_data, "%d.%m.%Y")?;

        let naive_datetime = naive_date.and_hms_opt(23, 59, 0).unwrap();

        let date = Utc.from_utc_datetime(&naive_datetime);

        Ok(date)
    }

    #[derive(Error, Debug)]
    pub enum MessageServiceError {
        #[error("Database error: {0}")]
        DatabaseError(#[from] sqlx::Error),

        #[error("File system error: {0}")]
        FileSystemError(#[from] std::io::Error),

        #[error("Environment variable error: {0}")]
        EnvVarError(#[from] std::env::VarError),
    }
}
