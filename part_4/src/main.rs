#![no_main]
#![no_std]
#![allow(dead_code, clippy::empty_loop)]
use msp430fr2355 as _;
use panic_msp430 as _; 

use msp430_rt::entry;

#[entry]
fn main() -> ! {
    const { panic!("Look at the examples first, then come back and remove this line.") };

    button_blink();
    // timer_blinky();
    // dac_adc_blinky();
}

// Part 1: Toggle an LED when one of the buttons on the board is pressed
fn button_blink() -> ! {

    loop {}
}

// Part 2: Configure a timer to trigger an interrupt exactly once per second.
// In the ISR increment a counter. In main toggle an LED based on the counter value.
fn timer_blinky() -> ! {

    loop {}
}

// Part 3: Configure a Smart Analog Combo (SAC) unit to use the included DAC to output an analog voltage.
// Then use the ADC to read this voltage (you can connect the SAC output pin to one of the ADC input pins).
fn dac_adc_blinky() -> ! {

    loop {}
}