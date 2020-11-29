// This app works in embedded environments,
// so we do not have 'std' crate.
#![no_std]

// And we have to make a special entrypoint ourselves,
// so we won't use 'main' entrypoint Rust provides.
#![no_main]

// Use crate 'panic_halt' to use it as panic handler.
extern crate panic_halt;

// Import some stuffs from arduino_uno.
use arduino_uno::{Peripherals, Pins};
use arduino_uno::prelude::*;

// Here is our entrypoint.
#[arduino_uno::entry]
fn main() -> ! {
    
    // Get the peripherals to access pin or some stuff
    let phr = Peripherals::take().unwrap();
    
    // Get pins and associate it to our peripherals...
    let mut pins = Pins::new(phr.PORTB, phr.PORTC, phr.PORTD);
    // And get D13 pin which is connected to the LED 'L' in arduino,
    // and make it output.
    let mut internal_led = pins.d13.into_output(&mut pins.ddr);
    
    // Now the initialization is done!
    // Here is the main routine.
    loop {
        // Toggle the LED! What a useful method.
        internal_led.toggle().void_unwrap();
        
        // Wait a moment.
        arduino_uno::delay_ms(100);
    }
}
