pub fn solve() -> (String, u64) {
    let problem_statement = String::from(
        "Find the smallest number evenly divisible by all the numbers 1 to 20."
    );

    // Instead of testing all the numbers one by one until they are all divisible, try a constructive
    // approach. Use the gcf * lcm = total trick. Stack the lcm calculations
    let mut current_lcm = 1;
    for i in 2..21 {
        current_lcm = lcm(i, current_lcm);
    }

    (problem_statement, current_lcm)

}

pub fn gcf(a: u64, b: u64) -> u64 {
    // Implement Euclid's algorithm in here
    // Make mutable values to work with so that the inputs dont have to be
    let mut x = a;
    let mut y = b;
    while x != 0 && y != 0 {
        // catch equality case as well
        if x >= y {
            x = x - y;
        }

        if y > x {
            y = y - x;
        }
    }
    // One of these is 0 anyways, so this will be the gcf
    x + y
}

pub fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcf(a, b)
}