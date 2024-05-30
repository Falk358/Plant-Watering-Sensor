use rtt_target::rprintln;
use sfsm::{add_state_machine, State, TransitGuard, Transition, IsState, StateMachine, SfsmError};
use crate::peripheral_access::with_peripherals;
use crate::utils::{is_p1_4_high, set_rgb_led_state};

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
    }
    fn execute(&mut self) {
        rprintln!("Calibration: Execute");
        self.set_board_led_to_green();
    }
    fn exit(&mut self) {
        rprintln!("Calibration: Exit");
    }
}

impl Calibration {
    fn set_board_led_to_green(&self) {
        with_peripherals(|peripherals| {
            set_rgb_led_state(&peripherals, false, true, false);
        });
    }

    fn is_board_button_pressed(&self) -> bool {
        let mut is_pressed = false;
        with_peripherals(|peripherals| {
            if is_p1_4_high(&peripherals) {
                is_pressed = true;
            }
        });
        is_pressed
    }
}

// Then implement the transitions.
// Each transition can define an action that gets executed during the transition to the next state.
// Additionally, an Into implementation has to be provided so that each state can be transformed Into
// the next one.
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
        if self.is_board_button_pressed() {
            TransitGuard::Transit
        } else {
            TransitGuard::Remain
        }
    }
}

impl State for Measure {
    fn entry(&mut self) {
        rprintln!("Measure: Entry");
    }
    fn execute(&mut self) {
        rprintln!("Measure: Execute");
        self.set_board_led_to_blue();
    }
    fn exit(&mut self) {
        rprintln!("Measure: Exit");
    }
}

impl Measure {
    fn set_board_led_to_blue(&self) {
        with_peripherals(|peripherals| {
            set_rgb_led_state(&peripherals, false, false, true);
        });
    }
}
