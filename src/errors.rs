use std::num::ParseIntError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseDurationError {
    #[error("the duration `{input}` is too long. Only `{max}` milliseconds are allowed.")]
    DurationTooLong{
        input: u64,
        max: u64
    },
    #[error("query param `wait` is missing, e.g. ?wait=2000")]
    DurationMissing,
    #[error("`{input}` is not a valid duration, error: {parse}")]
    InvalidDuration {
        input: String,
        parse: ParseIntError
    }
}