use std::cmp::{max, min};
use std::ops::RangeInclusive;

type Point = RangeInclusive<u32>;
type Coords = (Point, Point);

#[aoc::main(04)]
fn main(input: &str) -> (usize, usize) {
    let coords: Vec<Coords> = input
        .lines()
        .map(|l| {
            let coords = l.split_once(',').unwrap();
            (parse_point(coords.0), parse_point(coords.1))
        })
        .collect();

    (part1(&coords), part2(&coords))
}

fn part1(coords: &[Coords]) -> usize {
    coords
        .iter()
        .filter(|&coord| {
            coord.0.contains(coord.1.start()) && coord.0.contains(coord.1.end())
                || coord.1.contains(coord.0.start()) && coord.1.contains(coord.0.end())
        })
        .count()
}

fn part2(coords: &[Coords]) -> usize {
    coords
        .iter()
        .filter(|&coord| max(coord.0.start(), coord.1.start()) <= min(coord.0.end(), coord.1.end()))
        .count()
}

fn parse_point(point: &str) -> Point {
    let point = point.split_once('-').unwrap();

    point.0.parse::<u32>().unwrap()..=point.1.parse::<u32>().unwrap()
}
