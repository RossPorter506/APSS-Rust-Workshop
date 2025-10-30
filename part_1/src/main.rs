fn main() {
    println!("Hello, world!");
    println!("To run one of the example projects, use `cargo run --example example_name_here`");
    println!();
    println!("Once you've looked at all of the examples for this part, come back and have a go at this first project.");

    text_adventure_inventory_manager();
}

// Try to implement this. There is a working example in 'main.rs.done'.
fn text_adventure_inventory_manager() {
    println!("Time to begin your adventure! Your inventory consists of a number of potions, tools, and weapons");
    println!("Type commands to add or remove potions, tools or weapons; list your current inventory, or quit.");
    println!("Commands: 'add [potion, tool, weapon]', 'remove [potion, tool, weapon]', 'list', 'quit'");

    let mut inventory = todo!();

    loop {
        ready_for_input();
        let line = get_line();
        todo!();
    }

    println!("Bye!");
}

/// Place a '> ' at the start of the current line so the user knows they can type a command
fn ready_for_input() {
    print!("> ");
    // We also need to flush stdout to make sure it appears
    use std::io::Write;
    let _ = std::io::stdout().flush();
}

/// Get a line of text from the terminal, separated into words
fn get_line() -> Vec<String> {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.split_whitespace().map(|str| str.to_string()).collect()
}
