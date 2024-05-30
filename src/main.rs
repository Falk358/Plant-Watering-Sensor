#![no_main]
#![no_std]

use core::cell::RefCell;
use cortex_m::interrupt;
use cortex_m::interrupt::Mutex;
use cortex_m_rt::entry;
use msp432p401r::Peripherals;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use sfsm::*;
use lib::ux_lib::{read_p1_4, set_led_color, set_p1_4_pull_up, set_rgb_output_dir};

// Define your peripherals globally with Mutex for safe concurrent access
static PERIPHERALS: Mutex<RefCell<Option<Peripherals>>> = Mutex::new(RefCell::new(None));

fn with_peripherals<F>(func: F)
    where
        F: FnOnce(&mut Peripherals)
{
    interrupt::free(|cs| {
        if let Some(ref mut peripherals) = PERIPHERALS.borrow(cs).borrow_mut().as_mut() {
            func(peripherals);
        }
    });
}


pub struct Calibration {}

pub struct Measure {}

// Then define the whole state machine
add_state_machine!(

    pub MainStateMachine,                      // Name of the state machine. Accepts a visibility modifier.
    Calibration,                    // The initial state the state machine will start in
    [Calibration, Measure],          // All possible states
    [
        Calibration => Measure,      // All transitions
    ]
);


// Now the functionality for the states has to be implemented
// Every state can define an entry, execute and exit function.
// The execute function will be called multiple times, while the entry function will only be called
// initially and the exit when the state is left.
impl State for Calibration {
    fn entry(&mut self) {
        rprintln!("Calibration: Entry");
        // Example of using the peripherals in the main loop
        with_peripherals(|peripherals| {
            // Call your library function here, passing the mutable peripherals
            set_rgb_output_dir(&peripherals);
        });
    }
    fn execute(&mut self) {
        rprintln!("Calibration: Execute");
        with_peripherals(|peripherals| {
            // Call your library function here, passing the mutable peripherals
            set_led_color(&peripherals, false, true, false);
        });
    }
    fn exit(&mut self) {
        rprintln!("Calibration: Exit");
    }
}

// Then implement the transitions.
// Each transition can define an action that gets executed during the transition to the next state.
// Additionally a Into implementation has to be provided so that each state can be transformed
// Into the next one.
impl Into<Measure> for Calibration {
    fn into(self) -> Measure {
        Measure {}
    }
}

impl Transition<Measure> for Calibration {
    fn action(&mut self) {
        rprintln!("Calibration => Measure: Exit");
    }
    fn guard(&self) -> TransitGuard {
        let mut transit_guard = TransitGuard::Remain;
        with_peripherals(|peripherals| {
            if read_p1_4(&peripherals) {
                transit_guard = TransitGuard::Transit
            }
        });
        return transit_guard;
    }
}

impl State for Measure {
    fn entry(&mut self) {
        rprintln!("Measure: Entry");
        with_peripherals(|peripherals| {
            set_rgb_output_dir(&peripherals);
        });
    }
    fn execute(&mut self) {
        rprintln!("Measure: Execute");
        with_peripherals(|peripherals| {
            set_led_color(&peripherals, false, false, true);
        });
    }
    fn exit(&mut self) {
        rprintln!("Measure: Exit");
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Start...");

    let peripherals = Peripherals::take().unwrap();

    // Store the peripherals in the global mutex
    interrupt::free(|cs| {
        // Get a mutable reference to the optional peripherals and replace its contents
        PERIPHERALS.borrow(cs).replace(Some(peripherals));
    });

    with_peripherals(|peripherals| {
        lib::stop_watchdog_timer(peripherals);
        set_p1_4_pull_up(peripherals);
    });


    let mut main_state_machine = MainStateMachine::new();

    let calibration = Calibration {};
    main_state_machine.start(calibration).expect("Main-StateMachine could not start calibration");

    loop {
        main_state_machine.step().expect("Main-StateMachine could not transition to next state");

        let mut delay = 10000;
        while delay > 0 {
            delay -= 1;
        }
    }
}