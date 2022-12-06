use std::{io, cmp::Ordering};

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let mut contador: i32 = 0;

    let result = loop {

        let secret_number = rand::thread_rng().gen_range(1..=99);

        println!("Please input your guess.");
    
        let mut guess = String::new();
    
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
        let intvalue: i32 = guess.trim().parse().expect("inválid number");
    
        contador += 1;

        match intvalue.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You WIN!!!");
                break contador;
            },
        }
    
        println!("You guessed: {guess}");
        println!("The secret number is: {secret_number}.");
            
    };

    print!("The result is {result}");


}
