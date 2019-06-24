# prime_factorization
A small Rust module that calculates the prime factors of a given number.

This is an example of how the module can be used in a program:

```rust
// This program takes in an integer as a command line argument and produces the
// prime factors of that number

mod prime_factorization;
use std::env;

fn main() {
    let user_input: u64 = match env::args().nth(1) {
        Some(string_value) => match string_value.parse::<u64>() {
            Ok(value) => value,
            Err(_) => {
                println!("Couldn't parse `String` to `u64`.");
                return;
            }
        },
        None => {
            println!("No integer given as second command line argument.");
            return;
        }
    };

    let prime_vec: Vec<u64> = prime_factorization::get_prime_factors(user_input);

    if !prime_vec.is_empty() {
        print!("The prime factors of {} are: ", user_input);

        for (i, prime) in prime_vec.iter().enumerate() {
            print!("{}", prime);

            if i != prime_vec.len() - 1 {
                print!(", ");
            }
        }

        println!();
    } else {
        println!("{} is prime.", user_input);
    }
}


// Input and output
// ./prime_factorization 6469693230
// The prime factors of 6469693230 are: 2, 3, 5, 7, 11, 13, 17, 19, 23, 29
```
