pub fn solve() -> (String, u64) {
    let problem_statement = String::from(
        "Find the largest palindrome made from the product of two 3-digit numbers."
    );

    // Start solving here, use external functions if need be.
    // Note that this is a product of two 3-digit numbers, so test the three digit numbers from
    // 100 to 999 maybe
    let mut palindrome = 0;
    for i in (100..1000).rev() {
        for j in (100..1000).rev() {
            if check_palindrome(i * j) &&  (i * j) > palindrome {
                palindrome = i * j;
                break;
            }
        }
    }
    // Maybe try cases starting from 999, since we are looking for the largest.
    // Then the first case that works would be the maximum


    (problem_statement, palindrome as u64)
}

fn check_palindrome(number: u32) -> bool {
    let numstring = number.to_string();
    let revstring = numstring.chars().rev().collect::<String>();
    if &numstring == &revstring {
        return true;
    }
    false
}