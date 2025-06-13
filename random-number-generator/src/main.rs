use std::error::Error;
use std::io::{self, Write};
use std::process::exit;

use num::bigint::{BigUint, RandBigInt};
use num::One;
use rand::thread_rng;

fn main() -> Result<(), Box<dyn Error>> {
    let args = std::env::args().collect::<Vec<_>>();
    let mut upper_limit: String;

    match args.len() {
        (..2) => {
            print!("Enter the Upper limit: ");
            io::stdout().flush()?;
            upper_limit = String::new();
            match io::stdin().read_line(&mut upper_limit) {
                Ok(_) => {}
                Err(err) => {
                    eprintln!("An error occured: {err}");
                    exit(1)
                }
            }
            upper_limit = upper_limit.trim().to_string();
        }
        _ => {
            upper_limit = args[1].to_string();
        }
    }

    let lower_bigint = BigUint::one();
    let upper_bigint = match BigUint::parse_bytes(upper_limit.as_bytes(), 10) {
        Some(value) => {
            value + BigUint::one()
        },
        None => {
            eprintln!("Please enter a valid upper range.");
            exit(1)
        }
    };

    let mut rng = thread_rng();
    let random_biguint = rng.gen_biguint_range(&lower_bigint, &upper_bigint);
    println!("Random number between 1 and {upper_limit}: {random_biguint}");

    Ok(())
}
