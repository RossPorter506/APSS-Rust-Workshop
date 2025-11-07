#![no_main]
#![no_std]
#![allow(clippy::empty_loop)]
#![feature(abi_msp430_interrupt)] // When using interrupts you must include this line

use msp430::interrupt::enable as enable_interrupts;
use msp430_atomic::AtomicU16;
use panic_msp430 as _;

use msp430_rt::entry;
use msp430fr2355::interrupt;

use msp430fr2x5x_hal::{ehal::digital::OutputPin, gpio::Batch, pac, pmm::Pmm, watchdog::{Wdt, WdtClkPeriods}};

#[entry]
fn main() -> ! {
    // In this example we show the simplest possible interrupt configuration.
    // Using a global variable we toggle an LED every time an interrupt fires.

	let regs = pac::Peripherals::take().unwrap();

    let pmm = Pmm::new(regs.PMM);
    let mut led = Batch::new(regs.P1).split(&pmm).pin0.to_output();

    // Set up watchdog for approx 1 sec period
    let mut watchdog = Wdt::constrain(regs.WDT_A).to_interval();
    watchdog.set_vloclk()
        .enable_interrupts()
        .set_interval_and_start(WdtClkPeriods::_8192);

    // Globally enable interrupts. Will discuss why it's marked unsafe later.
    unsafe { enable_interrupts(); }

    loop {
        // Set LED high if COUNT is even, otherwise set low
        led.set_state( COUNT.load().is_multiple_of(2).into() ).ok();
    }
}

// If you remember back to one of the very first examples, we couldn't have a mutable static variable, partly because
// the compiler couldn't guarantee that multiple threads can nicely behave when sharing a value.
// Atomic types provide such a guarantee by ensuring that operations are 'atomic', that is, they can't be interrupted.
// The MSP430 supports atomic adds, subtractions, and XORs, so that's all that these atomic types allow.
// Note the type itself isn't declared as 'mut', so this is another type that has 'interior mutability'.
static COUNT: AtomicU16 = AtomicU16::new(0);

// This code runs whenever the watchdog time triggers. Unlike most other interrupts, the watchdog interrupt 
// doesn't require an interrupt flag to be cleared.
#[interrupt]
fn WDT() {
    COUNT.add(1);
}