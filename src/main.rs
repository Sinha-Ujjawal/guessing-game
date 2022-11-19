use rand::Rng;
use std::{
    io::{self, Write},
    str::FromStr,
};

#[derive(Debug)]
enum MyError {
    IOError(io::Error),
    ParseError(String),
}

fn prompt(msg: &'static str) -> io::Result<String> {
    let mut out = String::new();
    print!("{}", msg);
    io::stdout().flush()?;
    io::stdin().read_line(&mut out)?;
    Ok(out.trim().to_string())
}

fn prompt_parsed<T: FromStr>(msg: &'static str) -> Result<T, MyError> {
    let input = prompt(msg).map_err(MyError::IOError)?;
    input.parse().map_err(|_| MyError::ParseError(input))
}

fn prompt_until<T: FromStr>(msg: &'static str, validation_fn: &dyn Fn(&T) -> bool) -> T {
    loop {
        match prompt_parsed::<T>(msg) {
            Err(MyError::IOError(err)) => {
                panic!("{}", err)
            }
            Err(MyError::ParseError(s)) => {
                println!("Couldn't parse {}, try again!", s)
            }
            Ok(val) => {
                if !validation_fn(&val) {
                    ()
                } else {
                    return val;
                }
            }
        }
    }
}

struct Game {
    value: u8,
}

impl Game {
    fn new(rng: &mut rand::rngs::ThreadRng) -> Self {
        Self {
            value: (rng.gen::<u8>() % 100) + 1,
        }
    }

    fn play(&self) {
        println!("Game Started!");
        println!("Computer thinks of a number between 1 and 100 and you have to guess that number using atmost 7 guesses.");
        let mut guesses = 0;
        while guesses < 7 {
            let guess: u8 = prompt_until("Enter a guess: ", &|val| {
                if *val < 1 || *val > 100 {
                    println!("Value must be between 1 and 100, try again!");
                    return false;
                }
                true
            });
            guesses += 1;
            if guess > self.value {
                println!("Too High!")
            } else if guess < self.value {
                println!("Too Low!")
            } else {
                println!(":) You guessed it, the value was: {}", self.value);
                println!("Total # of guesses: {}", guesses);
                return ();
            }
        }
        println!("You couldn't guess it using atmost 7 guesses! :(");
        println!("The value was: {}", self.value);
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let game = Game::new(&mut rng);
    game.play()
}
