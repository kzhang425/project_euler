pub fn solve() -> (String, u64) {
    let problem_statement = String::from(
        "Find the sum of all the multiples of 3 or 5 below 1000.");
    
    // Code solution here
    let mut sum = 0;
    for i in 0..1000 {
        if (i % 5 == 0) || (i % 3 == 0) {
            sum += i;
        }
    }
    let answer = sum;

    (problem_statement, answer as u64)
}