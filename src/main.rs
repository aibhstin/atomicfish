use std::io::stdout;
use std::io::stdin;
use std::io::Write;

use std::env;

use std::fs::read_to_string;

use std::process::exit;

// BASE COMMANDS
// i - increment
// d - decrement
// s - square
// o - output
//
// ADDITIONAL COMMANDS (REPL)
// q - quit
//
// ADDITIONAL COMMANDS (FILE)
// l - newline
// e - space
// 
// RULES:
// if acc reaches 256 or -1, it resets to 0.

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 1 {
        let filename = &args[1];
        if check_filename(&filename) {
            parse_file(filename.to_string());
            exit(0);
        } else {
            println!("Invalid filename!");
            println!("Files must have a '.deadfish' extension");
            exit(1); 
        }
    }

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("----------------ATOMICFISH----------------");
    println!("----------A DEADFISH INTERPRETER----------");

    let mut acc = 0;

    loop {
        let mut input = String::new();
        
        print!("> ");
        stdout()
            .flush()
            .expect("An error was encountered whilst flushing stdout.");
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

fn parse_file(filename: String) {
    let contents: String = read_to_string(filename)
        .unwrap();

    let mut acc = 0;

    for c in contents.chars() {
        match c {
            'i' => acc += 1,
            'd' => acc -= 1,
            's' => acc *= acc,
            'o' => print!("{}", acc),
            'l' => println!(""),
            'e' => print!(" "),
            _ => print!(""),
        }
        if acc == 256 || acc == -1 {acc = 0}
    }
    println!("");

}

fn check_filename(s: &String) -> bool {
    if s.len() <= 9 { 
        return false 
    }

    let suffix: String = s
        .chars()
        .rev()
        .take(9)
        .collect::<String>()
        .chars()
        .rev()
        .collect();

    if suffix == ".deadfish" {
        return true;
    } else {
        return false;
    }
}

fn leave() {
    println!("Shutting down...");
    exit(0);
}

fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}
