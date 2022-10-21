pub fn solve() -> (String, u64) {
    // We only need to check upper bound of m, n of 500.
    let problem_statement = String::from(
        "Find a Pythagorean triple that satisfies a + b + c = 1000."
    );

    let m_n_bound = 500;
    let desired_sum = 1000;
    for m in 1..m_n_bound + 1 {
        for n in 1..m + 1 {
            if let Some((a, b, c)) = gen_triplet(m, n) {
                if (a + b + c) == desired_sum {
                    return (problem_statement, (a * b * c) as u64);
                }
            }

        }
    }

    (problem_statement, 0)
}

fn gen_triplet(m: u32, n: u32) -> Option<(u32, u32, u32)> {
    // Initialize some inputs and check for overflow values
    let m_squared = m.checked_pow(2)?;
    let n_squared = n.checked_pow(2)?;

    // Generate sides of a right triangle
    let a = diff(m_squared, n_squared);
    let b = 2 * m * n;
    let c = m_squared.checked_add(n_squared)?;

    Some((a, b, c))
}

fn diff(m: u32, n: u32) -> u32
// Just a supplementary function
    {
    if m > n {
        m - n
    } else {
        n - m
    }
}