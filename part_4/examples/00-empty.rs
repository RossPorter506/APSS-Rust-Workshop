// Embedded Rust can't use the std library (instead we get to use 'core', a subset), 
// and we have to manually mark the start of the program
#![no_main]
#![no_std]

// We have to include these to make sure they get linked into the program. 
//  vvvvvvvvvvvv This our 'Peripheral Access Crate'. More on this later.
use msp430fr2355 as _;

use panic_msp430 as _;
//  ^^^^^^^^^^^^ This sets up our panic behaviour. It just disables interrupts and loops forever by default.
// We can implement this ourselves if we want custom panic behaviour.

use msp430_rt::entry;

#[entry]
fn main() -> ! { 
	//    ^^^^ This means the function never returns
    loop {}
}
