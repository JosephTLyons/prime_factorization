# prime_factorization
A small Rust module that calculates the prime factors of a given number.

Example usage:

```rust
mod prime_factorization;

fn main() {
    let input: u64 = 223092870;
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

The prime factors of 223092870 are: 2, 3, 5, 7, 11, 13, 17, 19, 23
```
