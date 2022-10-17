pub fn solve() -> (String, u64) {
    // The difficult part is a way to generate the Fibonacci sequence
    let problem_statement = String::from(
        "For Fibonnaci numbers that do not exceed 4 million, find the sum of the even ones."
    );

    // Recursively generate but do not store (for memory conservation) the things
    // First, lets define some numbers and make sure they stay in scope for the rest of the program
    let mut a = 1;
    let mut b = 2;
    let mut sum = 0;
    while b < 4000000 {
        if b % 2 == 0 {
            sum += b;
        }
        let c = a + b;
        a = b;
        b = c;
    }

    (problem_statement, sum as u64)
}