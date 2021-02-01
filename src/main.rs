use std::io::stdout;
use std::io::stdin;
use std::io::Write;

use std::process::exit;

// BASE COMMANDS
// i - increment
// d - decrement
// s - square
// o - output
//
// ADDITIONAL COMMANDS
// q - quit
// 
// RULES:
// if acc reaches 256 or -1, it resets to 0.

fn main() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("----------------ATOMICFISH----------------");
    println!("----------A DEADFISH INTERPRETER----------");

    let mut acc = 0;

    loop {
        let mut input = String::new();
        
        print!("> ");
        stdout()
            .flush();
        stdin()
            .read_line(&mut input)
            .unwrap();

        remove_whitespace(&mut input);

        for c in input.chars() {
            match c {
                'i' => acc += 1,
                'd' => acc -= 1,
                's' => acc *= acc,
                'o' => print!("{}", acc),
                'q' => leave(),
                _ => print!(""),
            }
            if acc == 256 || acc == -1 {acc = 0}
        }
        println!("");
    }
}

fn leave() {
    println!("Shutting down...");
    exit(0);
}

fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}
