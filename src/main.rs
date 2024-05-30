#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use sfsm::*;
use lib::peripheral_access::{init_peripherals, with_peripherals};
use lib::state_machine_logic::{Calibration, MainStateMachine};
use lib::utils::{enable_p1_4_pull_up_resistor, disable_watchdog_timer, configure_rgb_pins_as_output};


#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Start...");

    init_peripherals();

    with_peripherals(|peripherals| {
        disable_watchdog_timer(peripherals);
        enable_p1_4_pull_up_resistor(peripherals);
        configure_rgb_pins_as_output(peripherals);
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