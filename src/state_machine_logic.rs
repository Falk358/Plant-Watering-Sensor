use rtt_target::rprintln;
use sfsm::{add_state_machine, State, TransitGuard, Transition};
use crate::peripheral_access::with_peripherals;
use crate::utils::{read_p1_4, set_led_color, set_rgb_output_dir};

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
