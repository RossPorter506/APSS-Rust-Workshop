#![no_main]
#![no_std]
#![allow(clippy::empty_loop)]
#![feature(abi_msp430_interrupt)]

use core::cell::RefCell;

use critical_section::{Mutex, with as critical_section};
use msp430::interrupt::enable as enable_interrupts;
use panic_msp430 as _;

use msp430_rt::entry;
use msp430fr2355::interrupt;

use msp430fr2x5x_hal::{ehal::digital::StatefulOutputPin, gpio::*, pac, pmm::Pmm, watchdog::Wdt};

// In the previous example we shared an atomic type between the main function and interrupt handler. It wasn't so bad. Unfortunately, we 
// often want to share non-atomic types too! For example, we might want to share a peripheral with the ISR to help handle the interrupt.

// In the following example we'll be wrapping non-atomic types to make them safe to share between interrupt handlers
// and the main function. The type names can be a bit of a mouthful. Generally in order to share a peripheral, you need to wrap in a
// Mutex<RefCell<Option< T >>>. Let's break that down:

//              vvvvv - Provides synchronisation for non-atomic types, makes them safe to share. Only allows getting a &T though, not &mut T.
static RED_LED: Mutex<RefCell<Option< Pin<P1, Pin0, Output> >>> = Mutex::new(RefCell::new(None));
//                    ^^^^^^^ - By dynamically checking the borrow rules at runtime it allows us to get a &mut T instead.

static P2IV: Mutex<RefCell<Option< PxIV<P2> >>> = Mutex::new(RefCell::new(None));
//                         ^^^^^^ - Note we can't configure peripherals at compile time, so we have to put 'None' here now and come 
//                                  back later to put a value in the global variable once it's been configured.

#[entry]
fn main() -> ! {
    let regs = pac::Peripherals::take().unwrap();
    let _wdt = Wdt::constrain(regs.WDT_A);

    let pmm = Pmm::new(regs.PMM);
    let p1 = Batch::new(regs.P1).split(&pmm);
    let p2 = Batch::new(regs.P2).split(&pmm);

    let red_led = p1.pin0.to_output();
    let mut button = p2.pin3.pullup();
    let p2iv = p2.pxiv; // Interrupt vector register for Port 2, tells us the cause of an interrupt

    // Next, we move our local variables into our global variables.
    // Open a 'critical section' - interrupts are temporarily disabled inside.
    //                vv Critical section token. Provided to Mutex to prove our operations will be atomic (with respect to interrupts).
    critical_section(|cs| {
        //      vvvvvvvvvvvvvv - Unwrap the Mutex<RefCell<T>> part to get a &mut T. In our case, we get a &mut Option<Pin<_>>
        RED_LED.borrow_ref_mut(cs).replace(red_led);
        //                         ^^^^^^^ - Replace the contents of the Option with the provided value. i.e., move red_led into the global variable.
        P2IV.borrow_ref_mut(cs).replace(p2iv);
    });

    button.select_falling_edge_trigger();

    // Note: This could technically be called inside a critical section, which would be bad! Hence why it's marked as unsafe.
    // Hopefully that's enough to make you stop and think about what you're doing!
    unsafe { enable_interrupts() };

    loop {}
}

#[interrupt]
fn PORT2() {
    // In this interrupt we toggle the red LED and clear the interrupt flag.

    // The MSP430 does support nested interrupts (if configured), so we still need a critical section in the interrupt too, just in case.
    critical_section(|cs| {
        // First, we need to unwrap our two shared resources, the LED and the interrupt vector:
        // Get a &mut T out of the Mutex<RefCell<Option<T>>> in two steps:

        // First, unwrap the Mutex<RefCell<_>> part to get a &mut Option<T>: 
        let mut maybe_red_led = RED_LED.borrow_ref_mut(cs);

        // Then get a &mut T from a &mut Option<T>:
        //                           vvvvvv - Convert a &mut Option<T> into an Option<&mut T>
        let _red_led = maybe_red_led.as_mut().unwrap();
        //                                    ^^^^^^ - Get a &mut T if the value is Some(), panic if None.
        //                                             This is fine, because we're *sure* we put values in before enabling interrupts

        // Once you get the idea, we can instead do this as a one-liner:
        // We can use a let-else statement (with some odd pattern matching syntax) and panic in the None case.
        let Some(ref mut red_led) = *RED_LED.borrow_ref_mut(cs) else { unreachable!() };
        //       ^^^^^^^ - This syntax is basically the same as .as_mut(), don't take the T out of the Option, just take a &mut T instead.

        // Do the same for the interrupt vector 
        let Some(ref mut p2iv) = *P2IV.borrow_ref_mut(cs) else { unreachable!() };
        

        // Reading the GPIO interrupt vector clears the interrupt flag and also tells us which pin cause the interrupt.
        if let GpioVector::Pin3Isr = p2iv.get_interrupt_vector() {
            red_led.toggle().ok();
        }
    });
}