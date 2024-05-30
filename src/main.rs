#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use sfsm::*;
use lib::peripheral_access::{init_peripherals, with_peripherals};
use lib::state_machine_logic::{Calibration, MainStateMachine};
use lib::utils::{set_p1_4_pull_up, stop_watchdog_timer};


#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Start...");

    init_peripherals();

    with_peripherals(|peripherals| {
        stop_watchdog_timer(peripherals);
        set_p1_4_pull_up(peripherals);
    });

    let mut main_state_machine = MainStateMachine::new();
    main_state_machine.start(Calibration {}).expect("Main-StateMachine could not start calibration");

    loop {
        main_state_machine.step().expect("Main-StateMachine could not transition to next state");

        let mut delay = 10000;
        while delay > 0 {
            delay -= 1;
        }
    }
}