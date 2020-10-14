use rand::Rng;
use std::time::Instant;
use num_format::{Locale, ToFormattedString};

// Given an MxN array of “boxes”, where each box contains some number of coins C[i][j],
// you want to maximize the number of coins you can take. You take coins by traversing
// row by row, taking all of the coins from ONE box in each row. However, any time you
// hange the index of the box you take coins from, you must pay a “change fee” equal
// to ABS(x - y) where x and y are the previous and new row indices. Write a function
// that can determine the optimal set of boxes to take coins from in order to maximize
// your profit after change fees


fn main() {
    let before = Instant::now();
    let grid = generate_grid(10000, 10000);
    // let grid = vec![
    //     vec![5, 9, 2, 6, 4, 4, 9],
    //     vec![4, 5, 7, 4, 6, 8, 8],
    //     vec![8, 2, 9, 8, 8, 6, 5],
    //     vec![2, 1, 1, 3, 5, 7, 1],
    //     vec![1, 8, 1, 7, 1, 6, 8],
    // ];
    println!("Grid generated in: {:.2?}", before.elapsed());
    let before = Instant::now();
    let solution = find_max_coins(grid);
    println!("Solution found in: {:.2?}", before.elapsed());
    let sum: i32 = solution.iter().fold(0,|a, &b| a as i32 + b as i32);
    // println!("{:?}", solution);
    println!("{} coins", sum.to_formatted_string(&Locale::en))
}

fn find_max_coins(grid: Vec<Vec<i16>>) -> Vec<i16> {
    let mut solution: Vec<i16> = vec![];
    let mut prev_idx = Box::new(0usize);

    // Iterate through the grid, bottom-up
    for row in 0..grid.len() {
        // For readability
        let current_row = &grid[row];

        // For the initial setup, find the best option in the first row
        if row == 0 {
            let mut best_final_option: i16 = 0;
            for col in 0..current_row.len() {
                let total = current_row[col] - col as i16;
                if total > best_final_option {
                    best_final_option = total;
                    *prev_idx = col;
                }
            }
            solution.push(current_row[*prev_idx]);
            // println!("found index {}, target {}", prev_idx, current_row[*prev_idx]);
            continue;
        }
        // println!("row {}: using prev index {}, target value {}", row, prev_idx, grid[row - 1][*prev_idx]);
        solution.push(prev_row_max(
            current_row,
            grid[row - 1][*prev_idx],
            &mut prev_idx,
        ));
    }
    solution
}

fn prev_row_max(current_row: &Vec<i16>, target: i16, prev_idx: &mut usize) -> i16 {
    let mut current_max: i16 = 0;
    for col in 0..current_row.len() {
        let value = current_row[col] + target - (*prev_idx as i16 - col as i16).abs(); // change fee
        // println!("calc: {} + {} - {} = {}", current_row[col], target, (*prev_idx as i16 - col as i16).abs(), value);
        if value > current_max {
            current_max = value;
            *prev_idx = col; // Copy the value from this scope to the heap reference
            // println!("new max {} at index {}", current_max, col)
        }
    }
    return current_row[*prev_idx];
}

fn generate_grid(rows: usize, cols: usize) -> Vec<Vec<i16>> {
    let rand_int = || rand::thread_rng().gen_range(1, 20);
    (0..rows).map(|_| (0..cols).map(|_| rand_int()).collect()).collect()
}
