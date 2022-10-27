// We can reuse a function defined in problem 7, helps us check prime in a vector of vectors in order
use super::useful_functions::e_sieve;

pub fn solve() -> (String, u64) {
    let problem_statement = String::from(
        "Find the sum of the primes below two million. This may take a long time."
    );

    let primes = e_sieve(2000000);

    let sum = primes.iter()
        .map(|&num| num as u64)
        .sum();

    (problem_statement, sum)

}