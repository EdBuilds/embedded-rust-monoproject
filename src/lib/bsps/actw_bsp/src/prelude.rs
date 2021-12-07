pub use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
pub use cortex_m_semihosting::{hprintln};
pub use virtual_devices::clock::VirtualClock;
