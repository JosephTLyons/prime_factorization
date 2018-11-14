// This is a help function for print_prime_factors, do not call it directly
fn get_first_factor (input: u64) -> u64 {
    let upper_limit: u64 = (input as f64).sqrt() as u64;

    for possible_factor in 2..upper_limit + 1 {
        if input % possible_factor == 0 {
            return possible_factor;
        }
    }

    return 0;
}

pub fn print_prime_factors (input: u64) {
    let mut next_value: u64 = input;
    let mut first_factor: u64 = get_first_factor (input);

    while first_factor != 0 {
        print! ("{}, ", first_factor);
        next_value /= first_factor;
        first_factor = get_first_factor (next_value);
    }

    println! ("{}", next_value);
}