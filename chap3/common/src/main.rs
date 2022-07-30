use std::io;

fn main(){

    // mutability
    let mut x = 5;
    println!("The value of x is {x}");

    x = 6;
    println!("The value of x is {x}");

    // constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;


    // shadowing
    println!("Start shadowing test");
    let x = 5;
    let x = x + 1;  // here x is being shadow 
    {
        let x = x * 2;
        println!("The value of x is the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    let spaces = "     ";
    println!("The value of spaces is: {spaces}");
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");

    
    // data types
    let guess: u32 = "3456".parse().expect("Not a Number"); // we have to give a type otherwise error

    // numeric overflow
    let val1: u8 = 250;
    println!("val1 is {val1}");
    let val1: u8 = val1+2;  // this may case panic due to numeric overflow
    println!("val1 is {val1}");


    // floating point types
    let x = 2.0; // f64
    let y: f32 = 3.0;  // f32
    println!("x is {x}");
    println!("y is {y}");
    
    
    // addition
    let sum = 5 + 10;
    println!("sum is {sum}");
    
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0
    // remainder
    let remainder = 43 % 5;

    // booleans
    let t = true;
    let f: bool = false; // with explicit type annotation

    // chars
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // tuples (compound types)
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x,y,z) = tup;
    println!("tup is ({x},{y},{z})");


    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("tup is ({five_hundred},{six_point_four},{one})");



    // array types
    let a = [1, 2, 3, 4, 5]; // data is store in the stack rather than the heap
    let a0 = a[0];
    println!("a0 {a0}");
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];


    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
