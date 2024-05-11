use std::io; // importing the std library to get the io module from it.

fn main() {
    println!("Enter your name: ");
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Error reading input");
    println!("your name is {}", user_input)
}
