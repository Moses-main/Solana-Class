fn main() {
    let x = 4;
    println!("x is: {}", x);
    
    // This is the interior scope of the variable 
    // This can modify the value of x 
    {
        let x = x - 2;
        println!("x is: {}", x);
    }

    // This does not inherit the value from the inner
    // scope of the variable
    let x = x + 1;
    println!("x is: {}", x);

    // CONSTANT
    // All constants are written in all caps and separated with
    // underscores and then the constants equally have their return type

    const SECONDS_IN_MINUTE: u32 = 60;
    println!("{}", SECONDS_IN_MINUTE);
    
}
