use core::cell::RefCell;
use cortex_m::interrupt;
use cortex_m::interrupt::Mutex;
use msp432p401r::Peripherals;

// Define your peripherals globally with Mutex for safe concurrent access
pub static PERIPHERALS: Mutex<RefCell<Option<Peripherals>>> = Mutex::new(RefCell::new(None));

pub fn with_peripherals<F>(func: F)
    where
        F: FnOnce(&mut Peripherals)
{
    interrupt::free(|cs| {
        if let Some(ref mut peripherals) = PERIPHERALS.borrow(cs).borrow_mut().as_mut() {
            func(peripherals);
        }
    });
}

pub fn init_peripherals() {
    let peripherals = Peripherals::take().unwrap();

    // Store the peripherals in the global mutex
    interrupt::free(|cs| {
        // Get a mutable reference to the optional peripherals and replace its contents
        PERIPHERALS.borrow(cs).replace(Some(peripherals));
    });
}