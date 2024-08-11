#![no_std]
#![no_main]

use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics

use stm32g0::stm32g0b1;

use rtt_target::{rprintln, rtt_init_print};

use rtic::app;

#[app(device = stm32g0b1)]
mod app {
    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(_: init::Context) -> (Shared, Local, init::Monotonics) {
        rtt_init_print!();
        rprintln!("Hello, world!");

        let mut i :u64 = 0;

        loop {
            rprintln!("RTIC works {}!", i);
            i += 1;
        }
    }
}
