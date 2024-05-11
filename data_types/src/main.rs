fn main() {
    // primitive data types
    // primitive data types are the basic or fundamental data types used to declare a variable

    // Categories
    // 1. Scalar types: Somthing that has a finite set of
    // possible values, following scale, i.e each value can be
    // compared toa anyother value as either equal or greater or less
    // e.g int, bool

    let _x: u32 = 34;
    let _y: i64 = 23;

    let _floating_point: f64 = 0.73;

    let _true_or_false: bool = true;

    let _letter: char = 'a';

    let _true_or_false:bool = false;


    // 2.  Compound types: types that can be constructed using the 
    // primitive data types and other composite types.
    // e.g array and tuple

    let tup: (i32, bool, char) = (1, true, 's'); //this is immutable
    // eprintln!("{:?}", tup); // printing all the elements
    // println!("{}", tup.2);

    let arr = [1,2,3,4,]; //this contains the same type of elements
    let un_initialized_arr: [i32; 4] // this creates an array which is not
    // initialized yet but has a type of i32 and then 4 elements.

    

}
