# APSS-Rust-Workshop
A repository with a number of introductory Rust examples. The examples do assume a basic level of competency with typical programming concepts, such as knowledge of a C-like language and syntax, object-oriented concepts like objects, classes and polymorphism. 

# Dependencies
Install Rust from [here](https://rust-lang.org/tools/install/). 

We'll use VSCode as our IDE. Install VSCode from [here](https://code.visualstudio.com/download) if you don't have it already.

### VSCode extensions
- [Rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- [Error Lens](https://marketplace.visualstudio.com/items?itemName=usernamehw.errorlens)
- [Even Better TOML](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml)

### Project setup
Open the folder containing this readme in VSCode. 

# Building and running the project
Run the project by opening a terminal and typing `cargo run`. This builds and runs `src/main.rs`, which should print 'hello world!' and point you towards the examples, provided Rust was correctly installed.

The project can be built (but not run) by typing `cargo build` instead.

The examples can be built and run by `cargo run --example example_name_here`.
