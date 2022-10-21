// We can reuse a function defined in problem 7, helps us check prime in a vector of vectors in order
use super::problem7::check_prime;

pub fn solve() -> (String, u64) {
    let problem_statement = String::from(
        "Find the sum of the primes below two million"
    );

    let mut primes: Vec<u32> = Vec::new();
    primes.push(2);
    
    // Initialize the counter that will start counting up. For more speed, only
    // consider odd numbers
    let upper_lim = 2000000;
    let mut counter: u32 = 3;
    while counter < upper_lim {
        if check_prime(counter, &primes) {
            primes.push(counter);
        }
        counter += 2;
    }

    let mut sum = 0;
    for num in primes {
        sum += num as u64;
    }

    (problem_statement, sum)

}