
use std::io;
use std::ops::Try;
use std::process::Termination;

// I don't want to have to deal with translating between result types since this is just a fun hacky
// project. As such, I'm defining one single return type that can subsume a bunch of other kinds of
// null-like returns, like:
//
//   - Result::Error
//   - Option::None
//
// And probably more since I won't update documentation!

#[derive(Debug)]
pub enum NiceResultError {
    NoneError,
    IOError(String),
}

pub enum NiceResult<T> {
    Err(NiceResultError),
    Ok(T),
}

impl<T> Termination for NiceResult<T> {
    fn report(self) -> i32 {
        match self {
            NiceResult::Err(_) => 1,
            NiceResult::Ok(_) => 0,
        }
    }
}

impl<T> Try for NiceResult<T> {
    type Ok = T;
    type Error = NiceResultError;

    fn into_result(self) -> Result<Self::Ok, Self::Error> {
        match self {
            NiceResult::Err(e) => Err(e),
            NiceResult::Ok(t) => Ok(t),
        }
    }

    fn from_error(e: Self::Error) -> Self {
        NiceResult::Err(e)
    }

    fn from_ok(t: Self::OK) -> Self {
        NiceResult::Ok(t)
    }
}

impl<T> From<Option<T>> for NiceResult<T> {
    fn from(option: Option<T>) -> NiceResult<T> {
        match option {
            None => NiceResult::Err(NiceResultError::NoneError),
            Some(t) => NiceResult::Ok(t),
        }
    }
}

impl<T> From<io::Result<T>> for NiceResult<T> {
    fn from(result: io::Result<T>) -> NiceResult<T> {
        match result {
            Err(e) => NiceResult::Err(NiceResultError::IOError(e.to_string())),
            Ok(t) => NiceResult::Ok(t),
        }
    }
}
