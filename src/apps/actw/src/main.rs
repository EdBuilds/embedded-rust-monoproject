#![no_std]
#![no_main]

use core::borrow::Borrow;
use cortex_m_rt::entry;
use actw_bsp::clock;
use actw_bsp::prelude::*;
use chrono::Duration;
#[entry]
fn main() -> ! {
    let mut clicky_clocky = clock::Clock::new();
    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    //debug::exit(debug::EXIT_SUCCESS);
    let mut wait_threshold = clicky_clocky.get_datetime().unwrap();
    loop {
        // your code goes here
        wait_threshold += Duration::seconds(15);
        clicky_clocky.wait_until(wait_threshold.borrow()).unwrap();
        let _res = hprintln!("{:?} ", clicky_clocky.get_datetime());
    }
}