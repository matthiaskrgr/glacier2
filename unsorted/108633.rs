use std::io;
use std::io::Write;

fn input(prompt: &str, value: &mut String) {
    print!("{}: ", prompt);

    value.clear();

    io::stdout().flush().unwrap();
    io::stdin().read_line(value).unwrap();

    *value = value.to_lowercase();
}

fn main() {
    let options: Vec<&str> = vec!["rock", "paper", "scissors", "quit"];
    println!("Enter \"quit\" or \"q\" to exit.");

    loop {
        let mut player: String = String::new();

        while player.len() == 0 {
            input("Rock (r), paper (p), scissors (s): ", &mut player);
            print!("\x1B[{}{}{}", "A", " ".repeat(player.len() + 35), "\r");

            if options.contains(&player.as_str()) {
                println!("{}", player);
            }
        }
    }
}
