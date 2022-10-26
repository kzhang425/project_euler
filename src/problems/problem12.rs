pub fn solve() -> (String, u64) {
    let problem_statement = String::from(
        "Find the first triangle number to have over 500 divisors."
    );

    let mut counter = 1;
    loop {
        if count_divisors(gen_triangle_num(counter)) > 500 {
            break;
        }

        counter += 1;
    }

    (problem_statement, gen_triangle_num(counter))
}

fn count_divisors(number: u64) -> u32 {
    // Number can be big but we're only potentially counting the divisors up to 501.
    // Don't need as much space for the count
    let mut current = number;
    let mut test_num = 2;
    
    // 
    let mut count = 0;
    while current != 1 {
        while number % test_num == 0 {
            
            count += 1;
        }
    }
    count
}

fn gen_triangle_num(index: u32) -> u64 {
    // Will break if the input is 0 or below!
    let i = index as u64;
    ((1 + i) * i) / 2
}