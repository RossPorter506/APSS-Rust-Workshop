#![no_main]
#![no_std]
#![allow(clippy::empty_loop)]

use msp430fr2355 as _;
use panic_msp430 as _;

use msp430_rt::entry;

#[entry]
fn main() -> ! {
	// Welcome to the lowest possible level of embedded Rust: Manual bit twiddling

	// If we want to blink an LED, we need to:

	// 1. Set P1.0 as an output.
	// 2. Toggle P1.0.
	// 3. Clear the LOCKLPM5 bit from the PMM5CTL0 register.
	// The MSP430 has a watchdog timer active by default, so this will restart the program for us.

	// The datasheet tells us that port P1 registers start at 0x200.
	// In Rust we make a raw pointer with *mut or *const
	// They are safe to make, but dereferencing them requires 'unsafe'.
	let p1_base: *mut u8 = 0x200 as *mut u8;
	// P1DIR is at offset 4 ( = 0x204), and P1OUT is at offset 2 ( = 0x202)
	let p1out = p1_base.wrapping_add(0x2);
	let p1dir = p1_base.wrapping_add(0x4);
	
	unsafe{ 
		// Set P1.0 as output by setting bit 0 in P1DIR
		p1dir.write_volatile(1); 

		// Toggle P1.0 by toggling bit 0 in P1OUT
		let p1out_val = p1out.read_volatile();
		p1out.write_volatile(p1out_val ^ 1); 
	}
	
	// Clear the LOCKLPM5 bit in the PM5CTL0 register
	let pm5ctl0 = (0x120 + 0x10) as *mut u8;
	unsafe { pm5ctl0.write_volatile( pm5ctl0.read_volatile() & 0b11111110 ) }
	

	// Not only is this pretty verbose, ugly, and error-prone, it also means that at any point 
	// any piece of code can reach into memory and change a register's value. As a program gets more
	// and more complex, this can be tricky to manage. This is exactly why global mutable state is 
	// considered a bad thing in all sorts of programming languages. What can we do about it? 

    loop {}
}
