pub fn solve() -> (String, u64) {
    let problem_statement = String::from(
        "Find the largest prime factor of 600851475143"
    );

    // Recall the square root rule, where the largest prime factor cannot be greater than the square
    // root of the entire number. This will limit our iterative loop.
    let number: u64 = 600851475143;

    println!("Solving problem 3: Calculating upper bound");
    // Find an upper bound for the square root by iterating
    let mut upperbound = 0;
    while upperbound*upperbound < number {
        upperbound += 1;
    }

    // Now upperbound is a proper upper bound for a potential prime factor
    // Proceed to test numbers while checking all the way until the upperbound. If nothing
    // works to divide the original number, call the original number prime.
    println!("Upper bound found, finding largest prime");
    let mut largest_prime = 1;
    for i in 2..upperbound {
        if check_prime(i) {
            match number % i {
                0 => largest_prime = i,
                _ => continue,
            }
        }
    }

    if largest_prime == 1 { 
        // This means the number is prime itself
        largest_prime = number 
    }

    (problem_statement, largest_prime as u64)


}

fn check_prime(number: u64) -> bool {
    for i in 2..number {
        if number % i == 0 {
            return false;
        }
    }
    true
}