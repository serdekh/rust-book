use std::io;
use std::process;

fn is_polindrome(mut x: usize) -> bool {
    let mut power_of_ten = 1;
    let mut reversed = 0;
    let x_clone = x;

    while x / power_of_ten != 0 {
        power_of_ten *= 10;
    } 

    power_of_ten /= 10;

    while x != 0 {
        reversed += (x % 10) * power_of_ten; 
        x /= 10;
        power_of_ten /= 10;
    }
    
    reversed == x_clone
}

fn read_line_as_usize() -> usize {
    let mut input_number = String::new();
    
    match io::stdin().read_line(&mut input_number) {
        Err(error) => {
            println!("[Error] Failed to read line: {error}.");
            process::exit(1);
        },
        _ => {}
    }
    
    let input_number: usize = match input_number.trim().parse() {
        Err(error) => {
            println!("[Error] Failed to convert the input to `usize`: {error}.");
            process::exit(1);
        },
        Ok(number) => number
    };

    input_number
}

fn main() {
    println!("Please write a number.");

    let input_number = read_line_as_usize();

    if is_polindrome(input_number) {
        println!("Polindrome!");
        process::exit(0);
    }

    println!("Not a polindrome!!!");
}