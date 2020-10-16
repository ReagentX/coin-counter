# Coin Counter

Given an MxN array of “boxes”, where each box contains some number of coins `C[i][j]`, you want to maximize the number of coins you can take.

You take coins by traversing row by row, taking all of the coins from ONE box in each row. However, any time you change the index of the box you take coins from, you must pay a “change fee” equal to `ABS(x - y)` where `x` and `y` are the previous and new row indices.

Write a function that can determine the optimal set of boxes to take coins from in order to maximize your profit after change fees.

## Example

The matrix:

    [
        [5, 9, 2, 6, 4, 4, 9],
        [4, 5, 7, 4, 6, 8, 8],
        [8, 2, 9, 8, 8, 6, 5],
        [2, 1, 1, 3, 5, 7, 1],
        [1, 8, 1, 7, 1, 6, 8],
    ]

has optimal path:

    [9, 8, 8, 7, 8]

## Performance

For a 10,000x10,000 matrix, `cargo run --release` results in the following timing:

    Grid generated in: 644.20ms
    Solution found in: 148.04ms
    169,976 coins
