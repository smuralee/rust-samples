use std::cmp::Ordering;
use std::io;

fn main() {
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

    // Invoke data types examples
    data_types();

    // Initiate guess the number
    guess_the_number();
}


fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    use super::add;
    use super::subtract;

    #[test]
    fn test_add() {
        assert_eq!(2 + 5, add(2, 5))
    }

    #[test]
    fn test_subtract() {
        assert_eq!(-3, subtract(2, 5))
    }
}

fn data_types() {

    add(2,3);
    subtract(5,4);

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
