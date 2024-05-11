use std::io;

fn main() {
    let x: u8 = 12; // 0 - 255
    let y: u8 = 10; // -128 - 127
    
    let z = x + y;
    // println!("{}", z); // generates compiles error as operation are performed on differnt data types

    // type conversion
    let a = 127_000 as i64;
    let b = 10_i32;

    let c = a / (b as i64); // here the type of b was converted.
    // println!("{}", c);

    // converting user string input to integer type
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Expected input");

    let int_input:i64 = input.trim().parse().unwrap();
    println!("{}", int_input + 3);
}

