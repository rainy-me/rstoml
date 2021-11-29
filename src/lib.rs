#[macro_use]
extern crate napi_derive;

use napi::{bindgen_prelude::*, JsUnknown};

#[napi]
/// Parse input string as TOML
fn parse(env: Env, text: String) -> Result<JsUnknown> {
    env.to_js_value::<toml::Value>(&toml::from_str(&text).map_err(to_napi_err)?)
}

#[napi]
/// Parse input buffer as TOML
fn parse_buffer(env: Env, buffer: Buffer) -> Result<JsUnknown> {
    env.to_js_value::<toml::Value>(&toml::from_slice(buffer.as_ref()).map_err(to_napi_err)?)
}

#[napi(object)]
/// Stringify option
struct StringifyOption {
    /// Serialize the given data structure as a "pretty" String of TOML.
    pub pretty: bool,
}

#[napi]
/// Converts a JavaScript Object to TOML string.
fn stringify(env: Env, obj: Object, option: Option<StringifyOption>) -> Result<String> {
    let val: toml::Value = env.from_js_value(obj)?;
    match option {
        Some(StringifyOption { pretty: true }) => toml::to_string_pretty(&val),
        _ => toml::to_string(&val),
    }
    .map_err(to_napi_err)
}

fn to_napi_err(err: impl ToString) -> Error {
    Error::new(Status::InvalidArg, err.to_string())
}
