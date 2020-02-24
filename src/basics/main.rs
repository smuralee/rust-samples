use std::cmp::Ordering;
use std::io;

fn main() {
    basics();
    data_types();

    let param1 = 10;
    let param2 = 12.5;
    println!("Return value of function: {}", fn_with_params(param1, param2));

    guess_the_number();
}

fn basics() {
    // Hello world!
    println!("Hello, world!");

    // Printing the variables
    let a = 5;
    let b = 10;
    println!("The values are : a = {} and b = {}", a, b);

    //Shadowing - use "let" for each update of c, since the variable is immutable by default
    let c = 10;
    let c = c * 45;
    let c = c / 24;
    println!("The value of c is {}", c);
}

fn data_types() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    println!("Sum : {}", sum);
    println!("Difference : {}", difference);
    println!("Product : {}", product);
    println!("Quotient : {}", quotient);
    println!("Remainder: {}", remainder);

    let t = true;
    let f: bool = false;
    println!("The boolean values are : {} and {}", t, f);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup; // deconstructing a tuple
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    let five_hundred = tup.0; //direct access of tuple using period (.)
    let six_point_four = tup.1;
    let one = tup.2;
    println!("The tuple values are: ({}, {}, {})", five_hundred, six_point_four, one);

    let arry = [1, 2, 3, 4, 5];
    println!("The 4th array element is : {}", arry[3]);
}

fn fn_with_params(param1: u32, param2: f64) -> f32 {
    println!("The params are: {} and {}", param1, param2);
    (param2 / param1 as f64) as f32
}

fn guess_the_number() {
    // Read the input and print the input
    const SECRET_NUMBER: u32 = 21; // Constant declaration

    loop {
        println!("Please input your guess!!");
        let mut guess = String::new(); // mutable variable
        io::stdin().read_line(&mut guess) // mutable reference
            .expect("Failed to read the line");
        let guess: u32 = guess.trim().parse() //shadowing the previous variable for the type change
            .expect("Please type a number"); // parse the string to number
        println!("You guessed: {}", guess);


        match guess.cmp(&SECRET_NUMBER) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
