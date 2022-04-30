///////////////////////////////////////////////////////////////////////////////
// NAME:            ser_date.rs
//
// AUTHOR:          Ethan D. Twardy <ethan.twardy@gmail.com>
//
// DESCRIPTION:     Serialization/Deserialization handlers for
//                  Option<chrono::NaiveDate>
//
// CREATED:         04/29/2022
//
// LAST EDITED:     04/29/2022
////

pub mod naive_datetime_option {
    use std::fmt;
    use serde::{ser, de};
    use chrono::naive::{NaiveDateTime, serde::ts_milliseconds};

    struct NaiveDateTimeOptionVisitor;
    impl<'de> de::Visitor<'de> for NaiveDateTimeOptionVisitor {
        type Value = Option<NaiveDateTime>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a formatted date and time string, or none")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where E: de::Error
        {
            let datetime: Result<NaiveDateTime, _> = value.parse();
            match datetime {
                Ok(timestamp) => Ok(Some(timestamp)),
                Err(e) => Err(E::custom(e)),
            }
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where E: de::Error
        { Ok(None) }
    }

    pub fn deserialize<'de, D>(deserializer: D) ->
        Result<Option<NaiveDateTime>, D::Error>
    where D: de::Deserializer<'de>
    {
        deserializer.deserialize_option(NaiveDateTimeOptionVisitor)
    }

    pub fn serialize<S>(timestamp: &Option<NaiveDateTime>, serializer: S) ->
        Result<S::Ok, S::Error>
    where S: ser::Serializer
    {
        match *timestamp {
            Some(ref date_time) =>
                ts_milliseconds::serialize(date_time, serializer),
            None => serializer.serialize_none(),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
