#![no_std]

pub mod ux_lib;

use msp432p401r::Peripherals;

pub fn stop_watchdog_timer(peripherals: &Peripherals) {
    peripherals.WDT_A.wdtctl.modify(|r, w| unsafe {
        let watchdog_password: u16 = 0x5A00;
        let hold: u16 = (r.bits() | 0x0080) & 0x00FF; // Set bit 7 to one, everything else stays the same
        w.bits(watchdog_password + hold)
    });
}


