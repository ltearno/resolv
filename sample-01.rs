use rand::Rng;
use std::cmp::Ordering;
use std::io;

#[derive(Debug)]
struct Turn {
    secret: u32,
    guess: u32,
}

impl Turn {
    fn is_win(&self) -> bool {
        match self.guess.cmp(&self.secret) {
            Ordering::Equal => true,
            Ordering::Greater => false,
            Ordering::Less => false,
        }
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number !");
                continue;
            }
        };

        let turn = Turn {
            secret: secret_number,
            guess,
        };

        println!("${:?}", turn);

        if turn.is_win() {
            println!("You win!");
            break;
        }

        match turn.guess.cmp(&turn.secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => (),
        }
    }
}
