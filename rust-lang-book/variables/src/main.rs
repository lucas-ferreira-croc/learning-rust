use std::io;
fn main() {

    const MAX_POINTS: u32 = 100_000; //constant

    let x = 5; // immutable varible
    println!("The value of x is: {}", x);
    let x = x + 1;
    println!("The value of x is: {}", x);

    let x = x * 2; //shadowing
    println!("The value of x is: {}", x);

    let spaces = "    ";
    let spaces = spaces.len(); //shadowing

    let x = 2.0; // f64

    let y: f32 = 3.0; //f32

    //add
    let sum = 5 + 10;

    //sub
    let diff = 95.5 - 4.3;

    //mult
    let product = 4 * 30;

    // div
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    let t = true;

    let f: bool = false;

    let c = 'z';
    let z = 'Z';

    let emojo = '🐊';

    let tup: (i32, f64, u8) = (500, 6.4, 1); 

    let tup = (500, 6.4, 1); 
    
    let(x, y, z) = tup;

    println!("The value of y is {}", y);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    let a: [i32; 5] = [1, 2, 3, 4,5];

    let a = [3; 5];

    let a = [1, 2, 3, 4,5];

    println!("Please enter an array index");
    
    let mut index = String::new();

    io::stdin()
    .read_line(&mut index)
    .expect("Failed to readline");

    let index: usize = index
     .trim()
     .parse()
     .expect("Index entered was not a number");
     
    let element = a[index];

    print!(
        "The value of the element at index {} is: {}"
        , index, element);
    

}
