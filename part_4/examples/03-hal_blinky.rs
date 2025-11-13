#![no_main]
#![no_std]
#![allow(clippy::empty_loop)]

use panic_msp430 as _;

use msp430_rt::entry;

//                     vvv The HAL re-exports the msp430fr2355 PAC 
use msp430fr2x5x_hal::{pac, gpio::*, ehal::digital::StatefulOutputPin, pmm::Pmm};
//                                   ^^^^ This is the 'embedded_hal' crate, which specifies a bunch of traits that 
//                                   microcontrollers can implement to allow peripheral drivers be completely device agnostic! 
//                                   More on this in a bit. Sometimes you may need to include these traits to use certain methods.
//                                   The HAL re-exports the embedded_hal traits so everything can be found in the HAL.

#[entry]
fn main() -> ! {
	// The next level of abstraction up is the Hardware Abstraction Layer (HAL).
	// Unlike the PAC, which automatically generated to exposes registers for your own use more conveniently, 
	// a HAL is hand-written to make common operations streamlined and to reduce the chances of you doing things wrong. 
	// As a result they can be more opinionated and may not cover all use cases.

	// (Pay attention! A HAL is the usual level of abstraction used by most embedded programs)

	let regs = pac::Peripherals::take().unwrap();

	// In order to prevent you from forgetting to clear the LOCKLPM5 bit, the HAL requires you to configure the PMM register first...
	let pmm = Pmm::new(regs.PMM);

	// Configure port P1 and split it into individual pins
	let port1 = Batch::new(regs.P1).split(&pmm);
	//                                    ^^^^ This method takes a reference to Pmm to make sure you didn't forget to clear LOCKLPM5!
	let mut led: Pin<P1, Pin0, Output> = port1.pin0.to_output();
    // Notice how the pin type specifies the pin number, and has a typestate based on the direction!
    // This prevents you from trying to set the value of an input pin, or read the value of an output pin.
    // Further, we can actually use this to enforce the correct GPIO configuration for other peripherals like SPI, I2C, etc..

	// Toggle the pin.
	led.toggle().ok();
	// Note: The HAL implements an embedded_hal trait which returns a Result, but 
	// for the MSP430 implementation this operation never fails. Unfortunately the compiler complains if
	// we don't 'use' the Result here. The easiest way to 'use' it is to convert the Result to an Option with .ok();
	
    // You might want to look at the documentation for the HAL: cargo doc --open --package msp430fr2x5x-hal

	// Now this is pretty cool! But can we go further?

    loop {}
}
