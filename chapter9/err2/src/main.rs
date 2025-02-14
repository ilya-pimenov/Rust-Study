use std::error::Error;
use std::fs::File;
//use std::net::IpAddr;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())

    //let home: IpAddr = "127.0.0.1".parse().expect("Hardcoded IP address should be valid");

    // loop {
    //     // -- snip --

    //     let guess: i32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };

    //     if guess < 1 || guess > 100 {
    //         println!("The secret number will be between 1 and 100.");
    //         continue
    //     }
    // }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100; got {value}.")
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
