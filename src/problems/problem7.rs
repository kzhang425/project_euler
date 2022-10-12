pub fn solve() -> (String, i64) {
    let problem_statement = String::from(
        "Find the 10001st prime number."
    );

    // Make a vector here to serve as the "memory" of found primes
    // Just a little memory heavy when it comes to solving the problem
    let mut primes: Vec<u32> = Vec::new();
    primes.push(2);
    
    // Initialize the counter that will start counting up. For more speed, only
    // consider odd numbers
    let found_prime: u32;
    let index = 10001;
    let mut counter: u32 = 3;
    loop {
        if check_prime(counter, &primes) {
            primes.push(counter);
            if let Some(prime) = primes.get(index - 1) {
                found_prime = *prime;
                break;
            }
        }
        counter += 2;
    }
    (problem_statement, found_prime as i64)
}


fn check_prime(num: u32, primes: &Vec<u32>) -> bool {
    for prime in primes {
        if num % *prime == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn see_if_prime() {
        let list: Vec<u32> = vec![2, 3, 5, 7, 11];
        assert_eq!(check_prime(13, &list), true);
    }

    #[test]
    fn see_if_not_prime() {
        let list: Vec<u32> = vec![2, 3, 5, 7, 11];
        assert_eq!(check_prime(14, &list), false);
    }
}