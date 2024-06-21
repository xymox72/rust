mod my_date_format {

    use serde::{self, Deserialize, Deserializer, Serializer};
    use chrono::NaiveDateTime;
    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(dt)
    }
}

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct Message {
    #[serde(with = "my_date_format")]
    messagecreatetime: NaiveDateTime,
    messagefilename: String,
    messageflags:i64,
    messagefolderid: i64,
    messagefrom: String,
    messageid: i64,
    messageuid: i64,
}
