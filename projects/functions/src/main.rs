use std::io;
use std::process;

fn read_line_as_usize() -> usize {
    let mut input_number = String::new();

    match io::stdin().read_line(&mut input_number) {
        Err(error) => {
            println!("[Error]: failed to read line: {error}.");
            process::exit(1);
        },
        _ => {}
    }

    let input_number: usize = match input_number.trim().parse() {
        Err(error) => {
            println!("[Error] failed to convert the input to `usize`: {error}.");
            process::exit(1);
        },
        Ok(number) => number,
    };

    input_number
}

fn even(x: usize) -> bool {
    x % 2 == 0
}

fn main() {
    let x = read_line_as_usize();

    if even(x) {
        println!("Even!");
        process::exit(0);
    } 
        
    println!("Odd!");
}