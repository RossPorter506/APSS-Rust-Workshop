#![no_main]
#![no_std]
#![allow(dead_code)]
#![feature(abi_msp430_interrupt)]

use bmp390::sync::Bmp390;
use panic_msp430 as _;

use msp430_rt::entry;

use msp430fr2x5x_hal::{
    clock::{ClockConfig, DcoclkFreqSel, MclkDiv, SmclkDiv}, 
    fram::Fram, 
    gpio::*, 
    i2c::{GlitchFilter, I2cConfig}, 
    pac, 
    pmm::Pmm, 
    watchdog::Wdt,
};

// This is an example of how we can use external drivers. In this case we're using the bmp390 crate for the BMP390 barometer.

// Because the embedded Rust ecosystem is built around embedded_hal, all drivers for devices are written with them in mind.
// This means you can just pick up and use someone else's driver without either you nor the driver author having to worry
// about what MCU you're using! 

// For example, there is an `I2c` trait in embedded_hal that defines something capable of I2C communication.
// External driver libraries (like the BMP390 crate) can use this trait without having to worry about how it's implemented, and 
// HAL users can seamlessly use these libraries without needing to implement I2C themselves!

#[entry]
fn main() -> ! {
    let regs = pac::Peripherals::take().unwrap();
    let _wdt = Wdt::constrain(regs.WDT_A);

    // GPIO
    let pmm = Pmm::new(regs.PMM);
    let p1 = Batch::new(regs.P1).split(&pmm);
    let sda = p1.pin2.to_output().to_alternate1();
    let scl = p1.pin3.to_output().to_alternate1();

    // Configure clock system for accurate I2C timing
    let mut fram = Fram::new(regs.FRCTL);
    let (smclk, _aclk, delay) = ClockConfig::new(regs.CS)
        .mclk_dcoclk(DcoclkFreqSel::_1MHz, MclkDiv::_1)
        .smclk_on(SmclkDiv::_1)
        .aclk_refoclk()
        .freeze(&mut fram);

    // Configure I2C
    let i2c_bus = I2cConfig::new(regs.E_USCI_B0, GlitchFilter::Max50ns)
        .as_single_master()
        .use_smclk(&smclk, 10) // 1 MHz / 10 = 100kHz
        .configure(scl, sda);
    
    // Set up BMP390
    let bmp390_config = bmp390::Configuration::default();
    let mut bmp390 = Bmp390::try_new(i2c_bus, bmp390::Address::Up, delay, &bmp390_config).unwrap();
    
    // Now we can just use it!
    let _ = bmp390.pressure();
    let _ = bmp390.altitude();
    let _ = bmp390.temperature();

    loop {}
}
