use super::useful_functions::generate_collatz;

pub fn solve() -> (String, u64) {
    let problem_statement = String::from(
        "Find the starting number that will yield the longest Collatz Sequence that is under 1 million"
    );

    // We can approach this problem by thinking of inverses. If we assume every number ends at 1,
    // we can backtrack by trying to keep the number under 1 million as much as possible. Or, 
    // we can approach this by testing a starting number and eliminating numbers if they show up in 
    // the chain.

    let mut map = vec![true; 999999]; // This will represent 1 to 999999
    let mut biggest_size = 0;
    let mut best_number: usize = 1;

    for i in (0..map.len()).rev() {
        // Starts counting from 9999998, to avoid confusion, use literal_i
        // skip doing calculations if the map value is false
        if map[i] {
            let literal_i = i + 1;
            let collatz_chain = generate_collatz(literal_i as u64);

            // Check if big
            if collatz_chain.len() > biggest_size {
                biggest_size = collatz_chain.len();
                best_number = literal_i;
            }

            // Look through collatz chain and mark things as false to stop checking for them
            for num in collatz_chain {
                if let Some(e) = map.get_mut(num - 1) {
                    *e = false
                }
            }

        }
    }
    
    (problem_statement, best_number as u64)
}


#[cfg(test)]
mod tests {
    use crate::problems::useful_functions::generate_collatz;

    #[test]
    fn collatz() {
        assert_eq!(generate_collatz(13), vec![13, 40, 20, 10, 5, 16, 8, 4, 2, 1]);
        assert_eq!(generate_collatz(1), vec![1])
    }
}