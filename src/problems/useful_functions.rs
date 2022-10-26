#[allow(dead_code)]
/// This requires a vector of u32s and an input of u32 in order to work.
pub fn check_prime(num: u64, primes: &Vec<u64>) -> bool {
    for prime in primes {
        if num % *prime == 0 {
            return false;
        }
    }
    true
}

/// A way to do number theory stuff like primes, divisors, and factors
pub struct Partition {
    number: u64,
    primes: Vec<u64>,
    powers: Vec<u32>,
}
impl Partition {
    pub fn new(number: u64) -> Self {
        // Make a vector here to serve as the "memory" of found primes
        // Just a little memory heavy when it comes to solving the problem
        let mut primes: Vec<u64> = Vec::new();
        primes.push(2);
        
        // Initialize the counter that will start counting up. For more speed, only
        // consider odd numbers
        let mut counter: u64 = 3;
        while counter <= number {
            if check_prime(counter, &primes) && (counter <= number) {
                primes.push(counter);
            }
            counter += 2;
        }

        // We need to clean up and remove the primes that do not divide the number
        primes.retain(|&x| number % x == 0);

        let powers = primes.iter()
            .map(|entry| {
                let mut power_num = 1;
                while number % entry.pow(power_num) == 0 {
                    if number % entry.pow(power_num + 1) == 0 {
                        power_num += 1;
                        continue;
                    } else {
                        break;
                    }
                }
                // Finally, return the value from the mapping step
                power_num
            }).collect(); // Collect into a Vector
        
        // Return a struct
        Partition {
            number,
            primes,
            powers,
        }
    }
}



/// Tests the functionality of the Divisors struct.
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_number() {
        let x = Partition::new(36);
        assert_eq!(x.number, 36);
    }

    #[test]
    fn check_primes() {
        let x = Partition::new(36);
        assert_eq!(x.primes, vec![2, 3]);
    }

    #[test]
    fn check_powers() {
        let x = Partition::new(36);
        assert_eq!(x.powers, vec![2, 2]);
    }
}