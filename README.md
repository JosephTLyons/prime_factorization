# prime_factorization
A small Rust module that calculates the prime factors of a given number.

This is an example of how the module can be used in a program:

```rust
// This program takes a command line argument in and produces the prime factors of that number

mod prime_factorization;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input: u64 = args[1].parse::<u64>().unwrap();
    let prime_vec: Vec<u64> = prime_factorization::collect_prime_factors (input);

    if prime_vec.len() > 0 {
        print! ("The prime factors of {} are: ", input);

        for x in 0..prime_vec.len() {
            if x < prime_vec.len() - 1 {
                print! ("{}, ", prime_vec[x]);
            }

            else {
                println! ("{}", prime_vec[x]);
            }
        }   
    }

    else {
        println! ("{} is prime.", input);
    }
}

// Input and output
// ./prime_factorization 6469693230
// The prime factors of 6469693230 are: 2, 3, 5, 7, 11, 13, 17, 19, 23, 29
```
