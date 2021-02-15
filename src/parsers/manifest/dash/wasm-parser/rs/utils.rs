use std::result;

use super::error::ParsingError;

pub type Result<T> = result::Result<T, ParsingError>;

pub fn parse_f64(value : &[u8]) -> Result<f64> {
    let res = std::str::from_utf8(value)?;
    let res_f64 = res.parse::<f64>()?;
    Ok(res_f64 as f64)
}

pub fn parse_u64_to_js_float(value : &[u8]) -> Result<f64> {
    let res = std::str::from_utf8(value)?;
    let res_u64 = res.parse::<u64>()?;
    Ok(res_u64 as f64)
}

pub fn parse_u64_or_bool_to_js_float(value : &[u8]) -> Result<f64> {
    match value {
        b"true" => Ok(f64::INFINITY),
        b"false" => Ok(f64::NEG_INFINITY),
        val => parse_u64_to_js_float(val),
    }
}

pub fn parse_bool(value : &[u8]) -> Result<bool> {
    match value {
        b"true" => Ok(true),
        b"false" => Ok(false),
        val => {
            let mut base_str = "Invalid boolean: ".to_owned();
            let val = std::str::from_utf8(val)?;
            base_str.push_str(val);
            Err(ParsingError(base_str))
        }
    }
}
