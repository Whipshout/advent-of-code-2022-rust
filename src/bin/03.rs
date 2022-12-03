use itertools::Itertools;

#[aoc::main(03)]
fn main(input: &str) -> (usize, usize) {
    let rucksacks: Vec<_> = input.lines().map(|l| l.as_bytes()).collect();

    (part1(&rucksacks), part2(&rucksacks))
}

fn part1(rucksacks: &[&[u8]]) -> usize {
    rucksacks
        .iter()
        .map(|&l| {
            let common_char_list = get_common_char(l.split_at(l.len() / 2));
            get_char_value(common_char_list)
        })
        .sum()
}

fn part2(rucksacks: &[&[u8]]) -> usize {
    rucksacks
        .iter()
        .tuples()
        .map(|(a, b, c)| {
            let common_char_list = get_common_char((a, &get_common_char((b, c))));
            get_char_value(common_char_list)
        })
        .sum()
}

// In the first problem, it's going to find only 1 common character
// But in the second it could find some come characters in one of the iterations
// That's why this returns a Vector instead of a u8 directly
fn get_common_char(bytes: (&[u8], &[u8])) -> Vec<u8> {
    bytes
        .0
        .iter()
        .copied()
        .filter(|char| bytes.1.contains(char))
        .collect()
}

fn get_char_value(c: Vec<u8>) -> usize {
    let c = *c.first().unwrap();
    match c {
        b'a'..=b'z' => c as usize - b'a' as usize + 1,
        b'A'..=b'Z' => c as usize - b'A' as usize + 27,
        _ => unreachable!(),
    }
}
