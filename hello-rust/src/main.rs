fn main() {
    let mut number = 300;

    let mut even_numbers = Vec::new();
    
    while number > 0{
        // let mut evenDigits = vec![]
        if number % 2 == 0{
            println!("{}", number);
            even_numbers.push(number);
        }
        number -=1;
    }
    println!(" Even Numvers: {:?}", even_numbers)
}