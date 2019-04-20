use chrono::NaiveDateTime;
use serde::{Deserializer, Deserialize};
use serde::de::{Error, Unexpected, Visitor};

pub fn deserialize_time_t_option<'a, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
    where D: Deserializer<'a>
{
    struct TimeTVisitor;

    impl<'a> Visitor<'a> for TimeTVisitor {
        type Value = NaiveDateTime;

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where E: Error
        {
            NaiveDateTime::from_timestamp_opt(value, 0)
                .ok_or_else(|| Error::invalid_value(Unexpected::Signed(value), &"could not interpret as UNIX timestamp"))
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where E: Error
        {
            self.visit_i64(value as i64)
        }

        fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            write!(formatter, "")
        }
    }

    struct WrapVisitor;

    impl<'a> Visitor<'a> for WrapVisitor {
        type Value = Wrap;

        fn visit_unit<E>(self) -> Result<Wrap, E>
            where E: Error
        {
            Ok(Wrap(None))
        }
        fn visit_none<E>(self) -> Result<Wrap, E>
            where E: Error
        {
            Ok(Wrap(None))
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Wrap, D::Error>
            where D: Deserializer<'a>
        {
            Ok(Wrap(Some(deserializer.deserialize_any(TimeTVisitor)?)))
        }

        fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            write!(formatter, "")
        }
    }

    struct Wrap(Option<NaiveDateTime>);

    impl<'a> Deserialize<'a> for Wrap {
        fn deserialize<D>(deserializer: D) -> Result<Wrap, D::Error>
            where D: Deserializer<'a>
        {
            deserializer.deserialize_option(WrapVisitor)
        }
    }

    Wrap::deserialize(deserializer).map(|wrap| wrap.0)
}
