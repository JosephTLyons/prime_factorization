# prime_factorization
A small Rust module that calculates the prime factorization of a given number

Example usage:

```rust
mod prime_factorization;

fn main() {
    let input: u64 = 340510170;
    let prime_vec: Vec<u64> = prime_factorization::collect_prime_factors (input);

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
```
// The prime factors of 340510170 are: 2, 3, 5, 7, 11, 13, 17, 23, 29
