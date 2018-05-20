extern crate rand;
// just like `use rand;`
// and then, we can use any methods in rand package.

use std::io;
// import io from std
use std::cmp::Ordering;
use rand::Rng;
// Rng scope (traits)

fn main() {
    println!("Guess the number!");
    // macro to print on screen;

    let  secret_number = rand::thread_rng().gen_range(1,101);
    println!("The secret number is: {}", secret_number);
    
    let mut guess = String::new();
    // mut make the variable can change.
    // String is a type, enhancable utf-8 string.
    // :: is a relation function, it relate to String, not others. Like a static method.

    println!("Please input your guess");
    
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    // io::stdin() means std::io::stdin()
    // .read_line(&mut guess)
    
    let guess: u32 = guess.trim().parse()
        .expect("please type a number");
    // guess can be [i32, u32, i64], rust don't know what it is.
    // so we defined u32.
    
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!")
    }

}
