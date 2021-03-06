pub fn get_prime_factors(mut input: u64) -> Vec<u64> {
    let mut prime_vec: Vec<u64> = Vec::new();
    let mut first_factor: u64 = get_first_factor(input);

    while first_factor != 0 {
        prime_vec.push(first_factor);
        input /= first_factor;
        first_factor = get_first_factor(input);
    }

    if !prime_vec.is_empty() {
        prime_vec.push(input);
    }

    prime_vec
}

fn get_first_factor(input: u64) -> u64 {
    let upper_limit: u64 = (input as f64).sqrt() as u64;

    for possible_factor in 2..=upper_limit {
        if input % possible_factor == 0 {
            return possible_factor;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::get_prime_factors;

    // Test some small and large prime numbers
    #[test]
    fn test_0001() {
        // Should this pass?
        assert_eq!(0, get_prime_factors(1).len());
    }

    #[test]
    fn test_0002() {
        assert_eq!(0, get_prime_factors(2).len());
    }

    #[test]
    fn test_0003() {
        assert_eq!(0, get_prime_factors(3).len());
    }

    #[test]
    fn test_large_prime() {
        assert_eq!(0, get_prime_factors(982_451_653).len());
    }

    // Test some typical non-prime numbers
    #[test]
    fn test_0004() {
        assert_eq!(vec![2, 2], get_prime_factors(4));
    }

    #[test]
    fn test_0010() {
        assert_eq!(vec![2, 5], get_prime_factors(10));
    }

    #[test]
    fn test_0100() {
        assert_eq!(vec![2, 2, 5, 5], get_prime_factors(100));
    }

    #[test]
    fn test_1024() {
        assert_eq!(vec![2; 10], get_prime_factors(1024));
    }
}
