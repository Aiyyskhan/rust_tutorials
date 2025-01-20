// импортирование библиотек
use std::{cmp::Ordering, io};
use rand::Rng;

pub fn game() {
    println!("Hello, Aiyyskhan!");
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guesse.");

        // объявление мутабельной переменной
        let mut guess = String::new();

        // ::stdin - вызов ассоциированной функции
        // .read_line() и .expect() - вызов методов
        // & - ссылка
        io::stdin()
        // std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to red line");

        // let guess: u8 = guess.trim().parse().expect("Please type a number!");
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("You need to enter a number!");
                continue
            },
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
