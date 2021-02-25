use crate::errors::{ParsingError, Result};

/// Try to parse the given array of bytes into an f64, by first converting
/// it to the corresponding ASCII (or even here, UTF-8) values.
pub fn parse_f64(value : &[u8]) -> Result<f64> {
    let res = std::str::from_utf8(value)?;
    let res_f64 = res.parse::<f64>()?;
    Ok(res_f64 as f64)
}

/// Try to parse the given array of bytes into an u64, by first converting
/// it to the corresponding ASCII (or even here, UTF-8) values.
pub fn parse_u64(value : &[u8]) -> Result<u64> {
    let res = std::str::from_utf8(value)?;
    let res_u64 = res.parse::<u64>()?;
    Ok(res_u64)
}

/// Try to parse the given array of bytes into an f64:
///   - INFINITY if it represents `"true"` in ASCII
///   - -INFINITY if it represents `"false"` in ASCII
///   - If it represents an integer in ASCII, returns that integer under an
///     f64 form.
///
/// Return an error in any other case.
pub fn parse_u64_or_bool(value : &[u8]) -> Result<f64> {
    match value {
        b"true" => Ok(f64::INFINITY),
        b"false" => Ok(f64::NEG_INFINITY),
        val => {
            let res_u64 = parse_u64(val)?;
            Ok(res_u64 as f64)
        }
    }
}

/// Try to parse the given array of bytes into a bool:
///   - `true` if it represents `"true"` in ASCII
///   - `false` if it represents `"false"` in ASCII
///
/// Return an error in any other case.
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

/// Parse ISO 8601 duration format (e.g. P5Y10M43HT22H8M3S) into the
/// corresponding seconds in a float format.
/// This code could be much simpler if it was RegExp-based but I preferred not
/// to, mainly because I didn't want to incur the size cost of importing regex
/// code in here
pub fn parse_iso_8601_duration(value : &[u8]) -> Result<f64> {
    if value[0] != b'P' {
        let err = ParsingError("Unexpected duration. Should start with \"P\"".to_owned());
        return Err(err);
    }
    let mut base = 1;
    let mut result = 0.;
    if value[1] != b'T' {
        loop {
            let (number, i) = read_next_float(value, base)?;
            if i == value.len() {
                let e = ParsingError("Invalid ISO 8601 duration: end encountered too soon".to_owned());
                return Err(e);
            }
            let factor = match value[i] {
                b'Y' => 365 * 24 * 60 * 60,
                b'M' => 30 * 24 * 60 * 60,
                b'D' => 24 * 60 * 60,
                _ => {
                    let e = ParsingError(format!("Invalid duration: unexpected unit \"{}\" {}", std::str::from_utf8(value)?, i));
                    return Err(e);
                }
            };
            result += number * (factor as f64);
            if i + 1 == value.len() {
                return Ok(result as f64);
            }
            if value[i + 1] == b'T' {
                break;
            }
            base = i;
        }
    }
    base += 1;
    loop {
        let (number, i) = read_next_float(value, base)?;
        if i == value.len() {
            let e = ParsingError("Invalid ISO 8601 duration: end encountered too soon".to_owned());
            return Err(e);
        }
        let factor = match value[i] {
            b'H' => 60 * 60,
            b'M' => 60,
            b'S' => 1,
            _ => {
                let e = ParsingError(format!("Invalid duration: unexpected unit \"{}\" {}", std::str::from_utf8(value)?, i));
                return Err(e);
            }
        };
        result += number * (factor as f64);
        if i + 1 == value.len() {
            return Ok(result as f64);
        }
        base = i;
    }
}

/// Parse a floating point number, represented by `value` in ASCII, starting at
/// the position `base_offset`.
/// The decimal separator can either a be a point ('.') or a colon (',').
///
/// If it succeeds, returns both the floating point number as an f64 and the
/// offset coming just after that float in `value` (or the length of `value` if
/// it ended with a float).
fn read_next_float(
    value : &[u8],
    base_offset : usize
) -> Result<(f64, usize)> {
    let mut i = base_offset;
    while i < value.len() && value[i] >= b'0' && value[i] <= b'9' {
        i += 1;
    }
    if i == value.len() || value[i] != b'.' || value[i] != b',' {
        // UNSAFE: We already checked that this string represents a valid integer
        let val_str = unsafe {
            std::str::from_utf8_unchecked(&value[base_offset..i])
        };
        let val_u64 = val_str.parse::<u64>()?;
        return Ok((val_u64 as f64, i));
    }

    while i < value.len() && value[i] >= b'0' && value[i] <= b'9' {
        i += 1;
    }
    // UNSAFE: We already checked that this string represents a valid integer
    let val_str = unsafe {
        std::str::from_utf8_unchecked(&value[base_offset..i])
    };
    let val_f64 = val_str.parse::<f64>()?;
    Ok((val_f64 as f64, i))
}
