extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() -> () {
    let mut gen = rand::thread_rng();
    let n = gen.gen_range(1, 21);
    loop {
        let guess = read_guess();
        match guess.cmp(&n) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn read_guess() -> u32 {
    println!("Guess : ");
    let mut guess = String::new();

    match io::stdin().read_line(&mut guess) {
        Ok(_) => match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => read_guess(),
        }
        Err(_) => panic!("Can't read")
    }
}

