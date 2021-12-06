#[macro_use]
extern crate napi_derive;

use chrono::prelude::*;
use chrono::TimeZone;
use napi::{bindgen_prelude::*, JsUnknown};

#[napi]
/// Parse input string as TOML
pub fn parse(env: Env, text: String) -> Result<JsUnknown> {
    text.parse::<toml::Value>()
        .map_err(to_napi_err)?
        .to_js_value(env)
}

#[napi]
/// Parse input buffer as TOML
pub fn parse_buffer(env: Env, buffer: Buffer) -> Result<JsUnknown> {
    toml::from_slice::<toml::Value>(buffer.as_ref())
        .map_err(to_napi_err)?
        .to_js_value(env)
}

#[napi(object)]
/// Stringify option
pub struct StringifyOption {
    /// Serialize the given data structure as a "pretty" String of TOML.
    pub pretty: bool,
}

#[napi]
/// Converts a JavaScript Object to TOML string.
pub fn stringify(env: Env, obj: Object, option: Option<StringifyOption>) -> Result<String> {
    let val: toml::Value = env.from_js_value(obj)?;
    match option {
        Some(StringifyOption { pretty: true }) => toml::to_string_pretty(&val),
        _ => toml::to_string(&val),
    }
    .map_err(to_napi_err)
}

trait ToJsValue {
    fn to_js_value(self, env: Env) -> Result<JsUnknown>;
}

impl ToJsValue for toml::value::Datetime {
    // naive datetime support and not fully tested.
    fn to_js_value(self, env: Env) -> Result<JsUnknown> {
        let t = match (self.date, self.time, self.offset) {
            // [Offset Date-Time]
            (
                Some(toml::value::Date { year, month, day }),
                Some(toml::value::Time {
                    hour,
                    minute,
                    second,
                    nanosecond,
                }),
                Some(offset),
            ) => match offset {
                toml::value::Offset::Z => Ok(Utc
                    .ymd(year as i32, month as u32, day as u32)
                    .and_hms_nano(hour as u32, minute as u32, second as u32, nanosecond)
                    .timestamp_nanos()),
                toml::value::Offset::Custom { hours, minutes } => Ok(FixedOffset::east(
                    (hours as i32) * 3600 + (minutes as i32) * 60,
                )
                .ymd(year as i32, month as u32, day as u32)
                .and_hms_nano(hour as u32, minute as u32, second as u32, nanosecond)
                .timestamp_nanos()),
            },
            // [Local Date-Time]
            (
                Some(toml::value::Date { year, month, day }),
                Some(toml::value::Time {
                    hour,
                    minute,
                    second,
                    nanosecond,
                }),
                None,
            ) => Ok(Local
                .ymd(year as i32, month as u32, day as u32)
                .and_hms_nano(hour as u32, minute as u32, second as u32, nanosecond)
                .timestamp_nanos()),
            // [Local Date]
            (Some(toml::value::Date { year, month, day }), None, None) => Ok(Local
                .ymd(year as i32, month as u32, day as u32)
                .and_hms(0, 0, 0)
                .timestamp_nanos()),
            // [Local Time]
            (
                None,
                Some(toml::value::Time {
                    hour,
                    minute,
                    second,
                    nanosecond,
                }),
                None,
            ) => Ok(Local::today()
                .and_hms_nano(hour as u32, minute as u32, second as u32, nanosecond)
                .timestamp_nanos()),
            // [Invalid]
            _ => Err(Error::new(Status::InvalidArg, "invalid date".to_string())),
        }?;
        Ok(env.create_date(t as f64 / 1_000_000f64)?.into_unknown())
    }
}

impl ToJsValue for toml::Value {
    fn to_js_value(self, env: Env) -> Result<JsUnknown> {
        // assert_eq!(1, 2);
        match self {
            toml::Value::String(val) => env.to_js_value(&val),
            toml::Value::Integer(val) => env.to_js_value(&val),
            toml::Value::Float(val) => env.to_js_value(&val),
            toml::Value::Boolean(val) => env.to_js_value(&val),
            toml::Value::Array(val) => env.to_js_value(&val),
            toml::Value::Table(val) => {
                let mut obj = env.create_object().unwrap();
                for (k, v) in val {
                    obj.set(k, v.to_js_value(env))?;
                }
                Ok(obj.into_unknown())
            }
            toml::Value::Datetime(val) => val.to_js_value(env),
        }
    }
}
pub fn to_napi_err(err: impl ToString) -> Error {
    Error::new(Status::InvalidArg, err.to_string())
}
