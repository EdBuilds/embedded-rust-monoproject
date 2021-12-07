#![no_std]
#![no_main]

use cortex_m_rt::entry;
use actw_bsp::clock;
use actw_bsp::prelude::*;
#[entry]
fn main() -> ! {
    let mut clicky_clocky = clock::Clock::new();
    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    //debug::exit(debug::EXIT_SUCCESS);
    loop {
        // your code goes here
        let _res = hprintln!("{:?} ", clicky_clocky.get_datetime());
    }
}
