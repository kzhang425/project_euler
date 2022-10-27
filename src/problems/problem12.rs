use super::useful_functions::count_divisors;

pub fn solve() -> (String, u64) {
    let problem_statement = String::from(
        "Find the first triangle number to have over 500 divisors."
    );

    println!("This can take a while!");
    let mut i = 1;
    while count_divisors(gen_triangle_num(i)) < 500 {
        i += 1;
    }

    (problem_statement, gen_triangle_num(i))
}


fn gen_triangle_num(index: u64) -> u64 {
    // Will break if the input is 0 or below!
    let i = index as u64;
    ((1 + i) * i) / 2
}