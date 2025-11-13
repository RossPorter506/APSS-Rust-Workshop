#![no_main]
#![no_std]
#![allow(dead_code, clippy::empty_loop)]

use panic_msp430 as _;

use msp430_rt::entry;

use msp430fr2x5x_hal::{
    clock::{ClockConfig, DcoclkFreqSel, MclkDiv}, 
    delay::SysDelay, 
    ehal::{
        digital::{OutputPin, StatefulOutputPin}, 
        delay::DelayNs,
    },
    fram::Fram, 
    gpio::*, 
    pac, 
    pmm::Pmm, 
    watchdog::Wdt,
};

#[entry]
fn main() -> ! {
	// The highest level of abstraction is the Board Support Package (BSP). This is designed for a specific 
    // board as opposed to a microcontroller. Since we're using an MSP dev board there isn't much on this 
    // particular PCB, so you might want to look at other examples, like the APSS-2 self-test repository, or the 
    // OpenDataloggingThermometer repo here: 
    // https://github.com/RossPorter506/OpenDataLoggerThermometer/blob/main/software/open_dl_thermometer_rs/src/board.rs
    // Generally speaking, a BSP will do all the configuration for you, leaving you with some high-level 
    // object that controls everything. We'll make a simple example below:

    // (You will likely end up making something like a BSP for your projects.)

    let mut board = Board::configure();

    loop {
        board.led1.toggle();
        board.led2.toggle();
        board.delay.delay_ms(500);
    }
}

struct Board {
    pub led1: Led<P1, Pin0>,
    pub led2: Led<P6, Pin6>,
    pub delay: SysDelay,
}
impl Board {
    fn configure() -> Self {
        let regs = pac::Peripherals::take().unwrap();
        Wdt::constrain(regs.WDT_A); // Disable watchdog timer

        // Configure the clock system so we can get accurate delay timing
        let mut fram = Fram::new(regs.FRCTL); // Note: Remember we had to set FRAM wait states when setting clock speed? 
                                              // The HAL won't let you forget...
        let (_aclk, delay) = ClockConfig::new(regs.CS)
            .mclk_dcoclk(DcoclkFreqSel::_1MHz, MclkDiv::_1)
            .aclk_refoclk()
            .smclk_off()
            .freeze(&mut fram);// <-- ...because the clock system will mutably borrow it to configure that for you.

        let pmm = Pmm::new(regs.PMM);

        let port1 = Batch::new(regs.P1).split(&pmm);
        let port6 = Batch::new(regs.P6).split(&pmm);
        let led1 = Led::new(port1.pin0.to_output());
        let led2 = Led::new(port6.pin6.to_output());

        Self {led1, led2, delay}
    }
}


struct Led<PORT: PortNum, PIN: PinNum> {
    pin: Pin<PORT, PIN, Output>,
}
impl<PORT: PortNum, PIN: PinNum> Led<PORT, PIN> {
    fn new(pin: Pin<PORT, PIN, Output>) -> Self {
        Self { pin }
    }
    fn turn_on(&mut self) {
        self.pin.set_high().ok();
    }
    fn turn_off(&mut self) {
        self.pin.set_low().ok();
    }
    fn toggle(&mut self) {
        self.pin.toggle().ok();
    }
}
