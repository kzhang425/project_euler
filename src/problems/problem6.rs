pub fn solve() -> (String, u64) {
    let problem_statement = String::from(
        "Find the difference between the sum of the squares of the first hundred natural numbers
        and the square of the sum"
    );

    let answer = (sum_squared(100) - squares_summed(100)).abs();

    (problem_statement, answer as u64)
}

fn sum_squared(num: i64) -> i64 {
    // The sum of numbers 1 to num all squared.
    i64::pow(((1 + num) * num) / 2, 2)
}

fn squares_summed(num: i64) -> i64 {
    // The sum of the squares of the numbers 1 to num.
    let mut sum = 0;
    for i in 1..num + 1 {
        sum += i.pow(2);
    }
    sum
}