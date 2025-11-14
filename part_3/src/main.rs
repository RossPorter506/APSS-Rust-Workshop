// Matching on string slices in const contexts isn't stabilised yet
#![feature(const_cmp, const_trait_impl)]

fn main() {
    println!("Hello, world!");
    println!("Once you've looked at the examples in this section, try to fill out the following project.");

    // cpu_emulator();
}

// Part 1:
// Design an emulated CPU that can do the following instructions:
// STORE - store a value into a register
// INCRM - increment the value stored in a register
// PRNTR - print a register's value
// The list of instructions should be checked at compile time for correctness using const fn's
//
// Part 2:
// Add a PRNTS instruction to print an arbitrary string slice (&str)
// Use a macro to reduce the boilerplate around adding instructions
fn cpu_emulator() {
    const INSTRUCTIONS: &[Instruction] = &[
        // parse_instruction(&["STORE", "255", "r1"]), 
        // parse_instruction(&["PRINT", "r1"]),
        // parse_instruction(&["INCRM", "r1"]),
        // parse_instruction(&["PRINT", "r1"]),
    ];

    let mut cpu = Cpu::default();
    println!("Before: {cpu:?}");
    cpu.run(INSTRUCTIONS);
    println!("After: {cpu:?}");
}

#[derive(Debug, Default)]
struct Cpu {
    // TODO
}
impl Cpu {
    fn run(&mut self, instrs: &[Instruction]) {
        todo!()
    }
}

const fn parse_3digit(str_num: &str) -> u8 {
    todo!()
}

const fn parse_register(reg_str: &str) -> Register {
    todo!()
}

const fn parse_instruction(instruction: &[&str]) -> Instruction {
    todo!()
}

enum Register {
    // TODO
}

type Value = u8;

enum Instruction {
    // TODO
}