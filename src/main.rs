// This app works in embedded environments,
// so we do not have 'std' crate.
#![no_std]

// And we have to make a special entrypoint ourselves,
// so we won't use 'main' entrypoint Rust provides.
#![no_main]

// Use crate 'panic_halt' to use it as panic handler.
extern crate panic_halt;

// Here is our entrypoint.
#[arduino_uno::entry]
fn main() -> ! {
    loop {

    }
}
