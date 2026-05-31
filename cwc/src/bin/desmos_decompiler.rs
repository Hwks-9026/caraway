use std::io::{self, Read};

fn main() {
    println!("Paste compiled Desmos state JSON, then press Ctrl+D (macOS/Linux):");
    println!("1) Open your Desmos graph in your browser.");
    println!("2) Open the inspect element page (Developer Tools).");
    println!("3) Go to the Console tab.");
    println!("4) Enter: state = Calc.getState()");
    println!("5) Enter: copy(JSON.stringify(state, null, 2))");
    println!("6) The compiled Desmos JSON is now copied to your clipboard. Paste it here and end input.");

    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    if input.trim().is_empty() {
        eprintln!("No Desmos code entered.");
        std::process::exit(1);
    }

    println!("desmos code entered successfully");
}
