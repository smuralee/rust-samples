use std::cmp::Ordering;
use std::io;

fn main() {
    add(2, 3);
    subtract(5, 4);
    multiply(3, 4);
    divide(6, 2);

    guess_the_number();
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn divide(a: i32, b: i32) -> i32 {
    a / b
}


#[cfg(test)]
mod tests {
    use crate::{add, divide, multiply, subtract};

    #[test]
    fn test_add() {
        assert_eq!(2 + 5, add(2, 5));
    }

    #[test]
    fn test_subtract() {
        assert_eq!(-3, subtract(2, 5));
    }

    #[test]
    fn test_multiply() {
        assert_eq!(20, multiply(10, 2));
    }

    #[test]
    fn test_divide() {
        assert_eq!(5, divide(25, 5));
    }
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
