use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Error reading input, please enter again");
    }
    let buffer = buffer.trim().to_owned();
    if buffer == "" {
        println!("Please enter something");
        return None;
    }
    Some(buffer)
}

fn get_int() -> Option<u32> {
    loop {
        let num = get_input()?;
        match num.parse::<u32>() {
            Ok(n) => {
                return Some(n);
            }
            Err(_) => {
                println!("Error parsing to u32: {num}");
                continue;
            }
        };
    }
}

fn main() {
    let random_number = rand::thread_rng().gen_range(1..=10);
    println!("Guess the number");

    loop {
        let guess = get_int();
        if guess.is_none() {
            continue;
        }
        let guess = guess.unwrap();
        match guess.cmp(&random_number) {
            Ordering::Less => println!("Small"),
            Ordering::Equal => {
                println!("You got it right");
                break;
            }
            Ordering::Greater => println!("Large"),
        }
    }
}
