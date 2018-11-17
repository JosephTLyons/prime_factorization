pub fn collect_prime_factors (mut input: u64) -> Vec<u64> {
    let mut prime_vec: Vec<u64> = Vec::new();
    let mut first_factor: u64 = get_first_factor (input);

    while first_factor != 0 {
        prime_vec.push (first_factor);
        input /= first_factor;
        first_factor = get_first_factor (input);
    }

    if prime_vec.len() > 0 {
        prime_vec.push (input);
    }

    return prime_vec;
}

fn get_first_factor (input: u64) -> u64 {
    let upper_limit: u64 = (input as f64).sqrt() as u64;

    for possible_factor in 2..upper_limit + 1 {
        if input % possible_factor == 0 {
            return possible_factor;
        }
    }

    return 0;
}