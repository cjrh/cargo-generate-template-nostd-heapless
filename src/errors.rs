use core::fmt::Display;
use core::fmt::Debug;
use heapless::String;

#[derive(Debug)]
pub enum Error {
    UnknownError(String<20>),
    StringAllocationFailure,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}
