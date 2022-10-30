use super::useful_functions::nCr;

pub fn solve() -> (String, u64) {
    let problem_statement = String::from(
        "With a square grid and starting from the top left corner, calculate the taxicab distance to the bottom right."
    );

    // Honestly this is a very easy combinatorics problem.
    let grid_x = 20;
    let grid_y = 20;
    let answer = nCr(grid_x + grid_y, grid_x);
    (problem_statement, answer)
}