# prime_factorization
A small Rust module that calculates the prime factors of a given number.

This is an example of how the module can be used in a program:

```rust
// This program takes in an integer as a command line argument and produces the
// prime factors of that number

mod prime_factorization;
use std::env;

fn main() {
    let input: u64 = env::args()
        .nth(1)
        .expect("No integer given as second command line argument.")
        .parse::<u64>()
        .expect("Couldn't parse String to u64.");
    let prime_vec: Vec<u64> = prime_factorization::get_prime_factors(input);

    if !prime_vec.is_empty() {
        print!("The prime factors of {} are: ", input);

        for (i, prime) in prime_vec.iter().enumerate() {
            print!("{}", prime);

            if i != prime_vec.len() - 1 {
                print!(", ");
            }
        }

        println!();
    } else {
        println!("{} is prime.", input);
    }
}

// Input and output
// ./prime_factorization 6469693230
// The prime factors of 6469693230 are: 2, 3, 5, 7, 11, 13, 17, 19, 23, 29
```
