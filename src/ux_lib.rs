use msp432p401r::Peripherals;


pub fn set_rgb_output_dir(peripherals: &Peripherals) {
    peripherals.DIO.padir.modify(|r, w| unsafe {
        w.p2dir().bits(r.p2dir().bits() | 0x07)
    });
}

pub fn set_p1_4_pull_up(peripherals: &Peripherals) {
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

pub fn read_p1_4(peripherals: &Peripherals) -> bool {
    // Access the PAIN register for P1 (Port 1 Input Register)
    // Bit mask for P1.4 is 0x10 (binary 0001 0000)
    let p1_4_value = peripherals.DIO.pain.read().p1in().bits() & 0x10;

    // If P1.4 is high, the result will be non-zero (0x10), otherwise, it will be zero.
    p1_4_value == 0
}


pub fn set_led_color(peripherals: &Peripherals, red: bool, green: bool, blue: bool) {
    peripherals.DIO.paout.modify(|r, w| unsafe {
        // Calculate the new bits for P2OUT based on boolean values for RGB
        let new_color = (if red { 0x01 } else { 0x00 }) |
            (if green { 0x02 } else { 0x00 }) |
            (if blue { 0x04 } else { 0x00 });

        // Set only the pins connected to the LEDs, preserve the state of other pins
        w.p2out().bits((r.p2out().bits() & 0xF8) | new_color)
    });
}

pub fn toggle_leds(peripherals: &Peripherals, red: bool, green: bool, blue: bool) {
    peripherals.DIO.paout.modify(|r, w| unsafe {
        // Determine which LED colors to toggle
        let toggle_mask = (if red { 0x01 } else { 0x00 }) |
            (if green { 0x02 } else { 0x00 }) |
            (if blue { 0x04 } else { 0x00 });

        // Toggle the specified LEDs
        w.p2out().bits(r.p2out().bits() ^ toggle_mask)
    });
}
