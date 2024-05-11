use std::io; // module to accept input from the user

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Error reading input");
    println!("{}", input)
}
