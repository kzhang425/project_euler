pub fn solve() -> (String, u64) {
    let problem_statement = String::from(
        "Find the greatest product of four adjacent numbers in this giant matrix:
08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08
49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00
81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65
52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91
22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80
24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50
32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70
67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21
24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72
21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95
78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92
16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57
86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58
19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40
04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66
88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69
04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36
20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16
20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54
01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48"
    );

    // Define the matrix of numbers to do stuff with. Trim the ends to make sure and then parse
    let matrix = String::from("
08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08
49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00
81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65
52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91
22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80
24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50
32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70
67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21
24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72
21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95
78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92
16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57
86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58
19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40
04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66
88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69
04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36
20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16
20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54
01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48
".trim()
    );

    // Now turn this monstrosity into a vector of vectors.
    let matrix_vec = Matrix::new(&matrix);
    
    // Iterate through the elements of the matrix and get the largest product there
    // Can define how many numbers multiplied together below:
    let how_many_multiplied = 4;
    let mut max = 1;
    for i in 0..matrix_vec.y_len {
        for j in 0..matrix_vec.x_len {
            let product = matrix_vec.max_product(i, j, how_many_multiplied);
            if product > max {
                max = product;
            }
        }
    }

    (problem_statement, max as u64)
}

// Here are some useful functions and aliases that make all this possible
type TwodArray = Vec<Vec<u32>>;

fn parse_matrix(matrix: &str) -> TwodArray {
    // This takes a string-looking matrix delimited by spaces and turns it into a 2D vector
    matrix.lines().map(|line| {
        line.split_whitespace().map(|num| {
            num.parse::<u32>().unwrap()
        }).collect::<Vec<u32>>()
    }).collect::<Vec<Vec<u32>>>()
}

fn get_dims(input: &Vec<Vec<u32>>) -> (usize, usize) {
    (input.len(), input[0].len())
}

// Lets actually define these things in a struct
struct Matrix {
    data: TwodArray,
    y_len: usize,
    x_len: usize,
}

impl Matrix {
    // Initialize the new struct
    fn new(array_string: &str) -> Self {
        let data = parse_matrix(array_string);
        let (y_len, x_len) = get_dims(&data);
        Matrix {
            data,
            y_len,
            x_len,
        }
    }

    fn get_value(&self, y: usize, x: usize) -> Option<u32> {
        Some(
            *self.data.get(y)?
                .get(x)?
        )
    }

    fn max_product(&self, y: usize, x: usize, len: usize) -> u32 {
        // The point of this function is to take a starting point in the matrix and then radially
        // calculate products (rows, columns, diagonals). Since get_value is Option<u32>, it is okay
        // if it indexes out of bounds because it will be None and won't panic (hopefully)

        let start = self.get_value(y, x).unwrap();

        let mut up: u32 = start;
        let mut down: u32 = start;
        let mut left: u32 = start;
        let mut right: u32 = start;
        let mut ne: u32 = start;
        let mut nw: u32 = start;
        let mut sw: u32 = start;
        let mut se: u32 = start;

        for i in 1..len {
            if y >= i {
                up *= self.get_value(y - i, x).unwrap_or_else(|| 1);
                ne *= self.get_value(y - i, x + i).unwrap_or_else(|| 1);
            }

            if x >= i {
                left *= self.get_value(y, x - i).unwrap_or_else(|| 1);
                sw *= self.get_value(y + i, x - i).unwrap_or_else(|| 1);
            }

            if (y >= i) && (x >= i) {
                nw *= self.get_value(y - i, x - i).unwrap_or_else(|| 1);
            }
            // Honestly don't even need most of them in the if statements.
            // Bare minimum I will need to check either ne or sw along with all the ones below
            down *= self.get_value(y + i, x).unwrap_or_else(|| 1);
            right *= self.get_value(y, x + i).unwrap_or_else(|| 1);
            se *= self.get_value(y + i, x + i).unwrap_or_else(|| 1);
        }

        *[up, down, left, right, ne, nw, sw, se].iter().max().unwrap()
    }
}

// We have the preliminary test cases below, inital testing of functions and methods to see
// if they would work in solving the problem

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_case() {
        let bruh = String::from(
            "01 02
            03 04".trim()
        );

        assert_eq!(parse_matrix(&bruh), vec![vec![1, 2], vec![3, 4]]);
    }

    #[test]
    fn size_getter() {
        let bruh = String::from(
            "01 02
            03 04".trim()
        );
        
        let x = parse_matrix(&bruh);

        assert_eq!(get_dims(&x), (2, 2));
    }

    #[test]
    fn workflow() {
        // Initialize a variable that will hold our max value
        let mut max = 1;
        let bruh = String::from(
            "01 02
            03 04".trim()
        );
        
        let matrix = Matrix::new(&bruh);
        for i in 0..matrix.y_len {
            for j in 0..matrix.x_len {
                if matrix.max_product(i, j, 2) > max {
                    max = matrix.max_product(i, j, 2);
                }
            }
        }

        assert_eq!(max, 12);
    }
}