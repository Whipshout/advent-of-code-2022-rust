use itertools::Itertools;

#[aoc::main(06)]
fn main(input: &str) -> (usize, usize) {
    (part1(input.as_bytes()), part2(input.as_bytes()))
}

fn part1(input: &[u8]) -> usize {
    calculate_start_marker_position(input, 4)
}

fn part2(input: &[u8]) -> usize {
    calculate_start_marker_position(input, 14)
}

fn calculate_start_marker_position(input: &[u8], len: usize) -> usize {
    // Good to know that Tuple from Itertools has a max size of 12
    // So you don't have to waste time searching why it's not working :')
    // Find using tuple_combinations().all() is x10 faster than iterating with all_unique() in a loop o_O
    input
        .windows(len)
        .enumerate()
        .find(|(_, window)| window.iter().tuple_combinations().all(|(lh, rh)| lh != rh))
        .unwrap()
        .0
        + len
}
