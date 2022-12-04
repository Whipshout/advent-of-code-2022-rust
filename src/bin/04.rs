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
        .filter(|&c| {
            c.0.contains(c.1.start()) && c.0.contains(c.1.end())
                || c.1.contains(c.0.start()) && c.1.contains(c.0.end())
        })
        .count()
}

fn part2(coords: &[Coords]) -> usize {
    coords
        .iter()
        .filter(|&c| max(c.0.start(), c.1.start()) <= min(c.0.end(), c.1.end()))
        .count()
}

fn parse_point(point: &str) -> Point {
    let point = point.split_once('-').unwrap();

    point.0.parse::<u32>().unwrap()..=point.1.parse::<u32>().unwrap()
}
