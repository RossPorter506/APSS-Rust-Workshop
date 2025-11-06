# Embedded Rust MSP430 Intro



More HAL examples can be found in the [`msp430fr2x5x-hal` repository](https://github.com/YuhanLiin/msp430fr2x5x-hal).

# Dependencies

To build the binary [`msp430-gcc`](https://www.ti.com/tool/MSP430-GCC-OPENSOURCE) should be available on your PATH.

To flash the binary on non-Windows systems [`mspdebug`](https://dlbeer.co.nz/mspdebug/) should be available on your PATH.
On non-Windows systems you should edit the 'runner' line in .cargo/config.toml to point to run.sh instead.

# Usage

Compile the empty project in `src/main.rs` using `cargo build`, or one of the examples with `cargo build --example <example_name_here>`. 

To flash the program to your device use `cargo run` or `cargo run --example <example_name_here>`.

### An aside - What are all these crates?

`msp430`
- Provides low-level access to MSP430 devices, such as access to CPU registers, enabling and disabling interrupts, and friendly wrappers around common assembly instructions like `nop`.

`msp430-rt`
- Manages the runtime and memory layout for an MSP430-based Rust project. Includes features like `#[entry]` to mark a function as the entry point of the program, and `#[interrupt]` to mark a function as an interrupt handler.

`msp430fr2355`
- The Peripheral Access Crate (PAC) for the MSP430FR2355. This provides a low-level register-level interface similar to what might be found in a typical embedded C program.

`msp430fr2x5x-hal`
- A high-level Hardware Abstraction Layer (HAL) designed to make working with the MSP430 hardware easier. Slightly higher-level and more opinionated than something like `MSP430 DriverLib`.

`panic-msp430`
- Provides a simple panic handler implementation that disables interrupts and loops infinitely.

`msp430-atomic`
- Provides wrappers for variable types that can be written to atomically. This makes them safe to use in a global context, like in interrupt handlers.

`critical-section`
- A generic 'critical section' trait that `msp430` implements. Contains wrappers like `Mutex` (not to be confused with `std::Mutex`) for using non-atomic types in global contexts.

# License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
