#![no_std]
#![no_main]

// pick a panicking behavior
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger
use cortex_m::asm;
use cortex_m_semihosting::{hprintln};
use stm32f4xx_hal::rtc;
use stm32f4xx_hal::i2c;
use virtual_devices::clock::VirtualClock;

pub mod clock;
pub mod prelude;

pub fn fake_main() -> ! {
    asm::nop(); // To not have main optimize to abort in release mode, remove when you add code
    let mut clicky_clocky = clock::Clock::new();
    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    //debug::exit(debug::EXIT_SUCCESS);
    loop {
        // your code goes here
        let _res = hprintln!("{:?} ", clicky_clocky.get_datetime());

        asm::nop(); // To not have main optimize to abort in release mode, remove when you add code
    }
}
