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

fn main() {
    if is_polindrome(142241) {
        println!("Polindrome!");
        return;
    }

    println!("Not a polindrome!!!");
}