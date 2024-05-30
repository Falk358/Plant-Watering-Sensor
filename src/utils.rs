use msp432p401r::Peripherals;

pub fn disable_watchdog_timer(peripherals: &Peripherals) {
    peripherals.WDT_A.wdtctl.modify(|r, w| unsafe {
        let watchdog_password: u16 = 0x5A00;
        // Set bit 7 to one, everything else stays the same
        let hold: u16 = (r.bits() | 0x0080) & 0x00FF;
        w.bits(watchdog_password + hold)
    });
}

pub fn configure_rgb_pins_as_output(peripherals: &Peripherals) {
    peripherals.DIO.padir.modify(|r, w| unsafe {
        w.p2dir().bits(r.p2dir().bits() | 0x07)
    });
}

pub fn enable_p1_4_pull_up_resistor(peripherals: &Peripherals) {
    // Access the PADIR register for P1 (Port 1 Direction Register)
    // Clear the bit for P1.4 to set it as input
    peripherals.DIO.padir.modify(|r, w| unsafe {
        w.p1dir().bits(r.p1dir().bits() & !0x10)
    });

    // Set P1.4 to use a pull-up resistor
    peripherals.DIO.paout.modify(|r, w| unsafe {
        w.p1out().bits(r.p1out().bits() | 0x10)
    });

    // Enable the pull-up resistor for P1.4
    peripherals.DIO.paren.modify(|r, w| unsafe {
        w.p1ren().bits(r.p1ren().bits() | 0x10)
    });
}

pub fn is_p1_4_high(peripherals: &Peripherals) -> bool {
    // Access the PAIN register for P1 (Port 1 Input Register)
    // Bit mask for P1.4 is 0x10 (binary 0001 0000)
    let p1_4_value = peripherals.DIO.pain.read().p1in().bits() & 0x10;
    p1_4_value == 0
}


pub fn set_rgb_led_state(peripherals: &Peripherals, red: bool, green: bool, blue: bool) {
    let new_color = compute_rgb_led_mask(red, green, blue);
    peripherals.DIO.paout.modify(|r, w| unsafe {
        // Set only the pins connected to the LEDs, preserve the state of other pins
        w.p2out().bits((r.p2out().bits() & 0xF8) | new_color)
    });
}

pub fn toggle_rgb_leds(peripherals: &Peripherals, red: bool, green: bool, blue: bool) {
    let toggle_mask = compute_rgb_led_mask(red, green, blue);
    peripherals.DIO.paout.modify(|r, w| unsafe {
        // Toggle the specified LEDs
        w.p2out().bits(r.p2out().bits() ^ toggle_mask)
    });
}

/// Computes the bitmask for RGB LED states based on boolean inputs.
fn compute_rgb_led_mask(red: bool, green: bool, blue: bool) -> u8 {
    (if red { 0x01 } else { 0x00 }) |
        (if green { 0x02 } else { 0x00 }) |
        (if blue { 0x04 } else { 0x00 })
}

