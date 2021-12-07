#![no_std]
extern crate chrono;
pub use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, Timelike};
pub trait VirtualClock {
    type Error;
    fn set_datetime(&mut self, datetime: &NaiveDateTime) -> Result<(), Self::Error>;
    fn get_datetime(&mut self) -> Result<NaiveDateTime, Self::Error>;
    fn wait_until(&self, datetime: &NaiveDateTime);
}


