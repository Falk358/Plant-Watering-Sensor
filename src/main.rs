#![no_main]
#![no_std]

use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use sfsm::*;
use rtic::app;
use lib::peripheral_access::{init_peripherals, with_peripherals};
use lib::state_machine_logic::{Calibration, MainStateMachine};
use lib::utils::{enable_p1_4_pull_up_resistor, disable_watchdog_timer, configure_rgb_pins_as_output};


// #[app(device = msp432p401r, peripherals = true)]
// mod app {
//     use super::*;
//
//     #[shared]
//     struct Shared {
//         // Define shared peripherals or other shared data here
//     }
//
//     #[local]
//     struct Local {
//         // Local resources for tasks
//     }
//
//     #[init]
//     fn init(ctx: init::Context) -> (Shared, Local, init::Monotonics) {
//         let peripherals: Peripherals = ctx.device;
//
//         // Initialization code here, similar to your `init_peripherals()`
//         init_peripherals(&peripherals);
//
//         // Set up initial shared resources and local resources
//         (Shared {}, Local {}, init::Monotonics())
//     }
//
//     #[idle]
//     fn idle(_ctx: idle::Context) -> ! {
//         loop {
//             // Idle loop, low power mode or similar
//         }
//     }
//
//     // Example of a task that could be triggered by an external event or timer
//     #[task]
//     fn process_adc(ctx: process_adc::Context) {
//         // ADC processing logic here
//     }
//
//     // More tasks for button press etc.
// }


//#[entry]
// fn main() -> ! {
//     rtt_init_print!();
//     rprintln!("Start...");
//
//     init_peripherals();
//
//     with_peripherals(|peripherals| {
//         disable_watchdog_timer(peripherals);
//         enable_p1_4_pull_up_resistor(peripherals);
//         configure_rgb_pins_as_output(peripherals);
//     });
//
//     let mut main_state_machine = MainStateMachine::new();
//     main_state_machine.start(Calibration {}).expect("Main-StateMachine could not start calibration");
//
//     loop {
//         main_state_machine.step().expect("Main-StateMachine could not transition to next state");
//
//         let mut delay = 10000;
//         while delay > 0 {
//             delay -= 1;
//         }
//     }
// }