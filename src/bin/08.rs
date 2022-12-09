use std::cmp::max;

use itertools::Itertools;

#[aoc::main(08)]
fn main(input: &str) -> (usize, usize) {
    let forest: Vec<Vec<u8>> = input.lines().map(|l| l.as_bytes().to_vec()).collect();

    (0..forest.len())
        .cartesian_product(0..forest[0].len())
        .fold((0, 0), |(p1, p2), (row, column)| {
            let tree = forest[row][column];

            // ----- PART 2 -----
            let left = calculate_score(tree, forest[row][0..column].iter().rev(), column);
            let right = calculate_score(
                tree,
                &forest[row][column + 1..forest[0].len()],
                forest[0].len() - (column + 1),
            );
            let up = calculate_score(tree, forest[0..row].iter().map(|a| &a[column]).rev(), row);
            let down = calculate_score(
                tree,
                forest[row + 1..forest.len()].iter().map(|a| &a[column]),
                forest.len() - (row + 1),
            );
            // ----- PART 2 -----

            (
                p1 + calculate_visibility(tree, &forest, row, column) as usize, // ----- PART 1 -----
                max(p2, left * right * up * down),
            )
        })
}

fn calculate_visibility(tree: u8, forest: &Vec<Vec<u8>>, row: usize, column: usize) -> bool {
    forest[row][0..column].iter().all(|&e| e < tree) // Left
        || forest[row][column + 1..forest[0].len()] // Right
            .iter()
            .all(|&e| e < tree)
        || forest[0..row].iter().all(|e| e[column] < tree) // Up
        || forest[row + 1..forest.len()] // Down
            .iter()
            .all(|e| e[column] < tree)
}

fn calculate_score<'a, I: IntoIterator<Item = &'a u8>>(tree: u8, trees: I, size: usize) -> usize {
    trees
        .into_iter()
        .position(|&a| a >= tree)
        .map_or(size, |a| a + 1)
}
