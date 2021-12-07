use core::borrow::{Borrow, BorrowMut};
use virtual_devices::clock::{NaiveDateTime, VirtualClock};
use stm32f4xx_hal::{pac, rtc};
use stm32f4xx_hal::rtc::Rtc;
use stm32f4xx_hal::prelude::*;

use rtcc::Rtcc;
use crate::clock::Error::HalRtcError;

#[derive(Debug)]
pub enum Error {
    InvalidInputData,
    HalRtcError(rtc::Error)
}
impl From<stm32f4xx_hal::rtc::Error> for Error{
    fn from(err: rtc::Error) -> Self {
        HalRtcError(err)
    }
}
pub struct Clock {
   rtc: Rtc

}

impl Clock {
    pub fn new() -> Self {
        let mut dp = pac::Peripherals::take().unwrap();
        Self { rtc: Rtc::new(dp.RTC, 255, 127, false, dp.PWR.borrow_mut()) }
    }
}

impl VirtualClock for Clock {
    type Error = Error;

    fn set_datetime(&mut self, datetime: &NaiveDateTime) -> Result<(), Self::Error> {

        self.rtc.set_datetime(datetime)?;
        Ok(())
    }

    fn get_datetime(&mut self) -> Result<NaiveDateTime, Self::Error> {
        Ok(self.rtc.get_datetime()?)
    }

    fn wait_until(&self, datetime: &NaiveDateTime) {
        todo!()
    }
}