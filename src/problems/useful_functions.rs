#[allow(dead_code)]
use std::collections::HashMap;
pub fn e_sieve(upper_limit: usize) -> Vec<usize> {
    if upper_limit < 2 {
        return vec![2];
    }
    let mut prime_bits = vec![true; upper_limit + 1];
    prime_bits[0] = false;
    prime_bits[1] = false;
    let cutoff = (upper_limit as f64).sqrt() as usize + 1;
    for i in 2..=cutoff {
        if prime_bits[i] && (i.pow(2) <= upper_limit) {
            for j in (i.pow(2)..=upper_limit).step_by(i) {
                prime_bits[j] = false;
            }
        }
    }

    // Now collect the numbers that survived into a vector
    (0..=upper_limit).into_iter().filter(|&x| prime_bits[x]).collect::<Vec<usize>>()
}

pub fn count_divisors(num: u64) -> u32 {
    let max_case = (num as f64).sqrt() as u64 + 1;
    let mut data: HashMap<u64, u32> = HashMap::new();
    let mut number = num;
    while number % 2 == 0 {
        number /= 2;
        data.entry(2).and_modify(|e| *e += 1).or_insert(1);
    }

    for i in (3..max_case).step_by(2) {
        while number % i == 0 && i != number {
            number /= i;
            data.entry(i as u64).and_modify(|e| *e += 1).or_insert(1);
        }
    }

    data.entry(number as u64).or_insert(1);

    data.into_iter().map(|(_, key)| {
        key + 1
    }).product()
}

fn into_digit_array(input: &str) -> Vec<Vec<u8>> {
    input.lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        }).collect::<Vec<Vec<u8>>>()
}

pub fn generate_collatz(start: u64) -> Vec<usize> {
    // Passes the test
    let mut number = start;
    let mut returnable: Vec<usize> = Vec::new();
    returnable.push(number as usize);
    while number != 1 {
        match number % 2 {
            0 => number /= 2,
            _ => number = number * 3 + 1,
        }
        returnable.push(number as usize);
    }

    returnable
}