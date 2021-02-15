use std::mem;
use super::{
    CustomEventType,
    onCustomEvent,
};

/// Very simple error type, only used to generate a String description of a
/// parsing error, to then be reported on the JS-side.
#[derive(Debug, Clone)]
pub struct ParsingError(pub String);

impl ParsingError {
    /// Call JS-side callback with this ParsingError in argument to report an
    /// error.
    pub fn report_err(&self) {
        let len = self.0.len();
        unsafe {
            onCustomEvent(
                CustomEventType::Error,
                mem::transmute((*self.0).as_ptr()),
                len);
        }
    }
}

impl<T : std::error::Error> From<T> for ParsingError {
    fn from(err : T) -> ParsingError {
        ParsingError(err.to_string())
    }
}

// impl fmt::Display for ParsingError {
//     // This trait requires `fmt` with this exact signature.
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // Write strictly the first element into the supplied output
//         // stream: `f`. Returns `fmt::Result` which indicates whether the
//         // operation succeeded or failed. Note that `write!` uses syntax which
//         // is very similar to `println!`.
//         write!(f, "{}", self.0)
//     }
// }

// impl From<&quick_xml::Error> for ParsingError {
//     fn from(err : &quick_xml::Error) -> ParsingError {
//         ParsingError(err.to_string())
//     }
// }

// impl From<quick_xml::Error> for ParsingError {
//     fn from(err : quick_xml::Error) -> ParsingError {
//         ParsingError(err.to_string())
//     }
// }

// impl From<std::str::Utf8Error> for ParsingError {
//     fn from(err : std::str::Utf8Error) -> ParsingError {
//         ParsingError(format!("Issue when parsing string: {}", err))
//     }
// }

// impl From<std::num::ParseFloatError> for ParsingError {
//     fn from(err : std::num::ParseFloatError) -> ParsingError {
//         ParsingError(format!("Issue when parsing float: {}", err))
//     }
// }

// impl From<std::num::ParseIntError> for ParsingError {
//     fn from(err : std::num::ParseIntError) -> ParsingError {
//         ParsingError(format!("Issue when parsing integer: {}", err))
//     }
// }
