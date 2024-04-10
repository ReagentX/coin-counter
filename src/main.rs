use num_format::{Locale, ToFormattedString};
use rand::Rng;
use std::time::Instant;

// Given an MxN array of “boxes”, where each box contains some number of coins C[i][j],
// you want to maximize the number of coins you can take. You take coins by traversing
// row by row, taking all of the coins from ONE box in each row. However, any time you
// change the index of the box you take coins from, you must pay a “change fee” equal
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
    let sum: i32 = solution.iter().fold(0, |a, &b| a + b as i32);
    // println!("{:?}", solution);
    println!("{} coins", sum.to_formatted_string(&Locale::en))
}

fn find_max_coins(grid: Vec<Vec<i16>>) -> Vec<i16> {
    let mut solution: Vec<i16> = vec![];
    let prev_idx = &mut 0usize;

    // Iterate through the grid
    for row in 0..grid.len() {
        // For readability
        let current_row = &grid[row];

        // For the initial setup, find the best option in the first row
        if row == 0 {
            let mut best_final_index: usize = 0;
            let mut max_found: i16 = 0;
            (0..current_row.len()).for_each(|possible_best_index| {
                *prev_idx = possible_best_index; // Reset after each loop because `possible_option` writes over it
                let possible_option =
                    prev_row_max(&grid[row + 1], current_row[possible_best_index], prev_idx);
                if possible_option + current_row[possible_best_index] > max_found {
                    max_found = possible_option + current_row[possible_best_index];
                    best_final_index = possible_best_index;
                }
            });
            *prev_idx = best_final_index;
            solution.push(current_row[best_final_index]);
            // println!("row {}: best option at index {}, target value {}", row, prev_idx, current_row[*prev_idx]);
            continue;
        }
        // For all subsequent rows, greedily pick the best path
        // println!("row {}: using prev index {}, target value {}", row, prev_idx, grid[row - 1][*prev_idx]);
        solution.push(prev_row_max(
            current_row,
            grid[row - 1][*prev_idx],
            prev_idx,
        ));
    }
    solution
}

fn prev_row_max(current_row: &[i16], target: i16, prev_idx: &mut usize) -> i16 {
    let mut current_max: i16 = 0;
    // Don't modify the heap value until we have a value to replace it with
    let mut local_max_idx: usize = *prev_idx;
    (0..current_row.len()).for_each(|col| {
        // Determine value less change fee
        let value = current_row[col] + target - (*prev_idx as i16 - col as i16).abs();
        // println!("calc: {} + {} - (|{} - {}|) = {}", current_row[col], target, *prev_idx as i16, col as i16, value);
        if value > current_max {
            current_max = value;
            local_max_idx = col;
            // println!("new max {} at index {}", current_max, local_max_idx)
        }
    });
    // Copy the final value back into the borrowed variable
    *prev_idx = local_max_idx;
    current_row[*prev_idx]
}

fn generate_grid(rows: usize, cols: usize) -> Vec<Vec<i16>> {
    let rand_int = || rand::thread_rng().gen_range(1, 20);
    (0..rows)
        .map(|_| (0..cols).map(|_| rand_int()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::find_max_coins;

    #[test]
    fn right_hand_side() {
        let grid = vec![
            vec![9, 1, 1, 1, 1, 1, 9],
            vec![1, 1, 1, 1, 1, 1, 6],
            vec![1, 1, 1, 1, 1, 1, 6],
            vec![1, 1, 1, 1, 1, 1, 6],
            vec![1, 1, 1, 1, 1, 1, 6],
        ];
        let solution = find_max_coins(grid);
        assert_eq!(solution, vec![9, 6, 6, 6, 6])
    }

    #[test]
    fn normal_distribution() {
        let grid = vec![
            vec![5, 9, 2, 6, 4, 4, 9],
            vec![4, 5, 7, 4, 6, 8, 8],
            vec![8, 2, 9, 8, 8, 6, 5],
            vec![2, 1, 1, 3, 5, 7, 1],
            vec![1, 8, 1, 7, 1, 6, 8],
        ];
        let solution = find_max_coins(grid);
        assert_eq!(solution, vec![9, 8, 8, 7, 8])
    }

    #[test]
    fn eetai_distribution() {
        let grid = vec![
            vec![0, 0, 1, 0, 0],
            vec![0, 2, 0, 3, 0],
            vec![5, 0, 0, 0, 3],
            vec![0, 0, 0, 0, 0],
        ];
        let solution = find_max_coins(grid);
        assert_eq!(solution, vec![1, 3, 5, 0])
    }

    #[test]
    fn eetai_distribution_2() {
        let grid = vec![
            vec![0, 0, 10, 0, 0],
            vec![0, 2, 0, 3, 0],
            vec![5, 0, 0, 0, 3],
        ];
        let solution = find_max_coins(grid);
        assert_eq!(solution, vec![10, 3, 5])
    }

    #[test]
    fn end_profit() {
        let grid = vec![
            vec![1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1, 1, 1000],
        ];
        let solution = find_max_coins(grid);
        assert_eq!(solution, vec![1, 1, 1, 1, 1000])
    }
}
