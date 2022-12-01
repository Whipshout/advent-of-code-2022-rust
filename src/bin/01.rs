use std::cmp::PartialOrd;

#[aoc::main(01)]
fn main(input: &str) -> (u32, u32) {
    let mut calories: Vec<u32> = input
        .split("\n\n")
        .map(|l| l.lines().map(|c| c.parse::<u32>().unwrap()).sum::<u32>())
        .collect();

    quick_sort::<Vec<u32>>(&mut calories);

    (part1(&calories), part2(&calories))
}

fn part1(calories: &[u32]) -> u32 {
    calories.last().unwrap().to_owned()
}

fn part2(calories: &[u32]) -> u32 {
    calories.iter().rev().take(3).sum()
}

pub fn partition<T: PartialOrd>(arr: &mut [T], lo: isize, hi: isize) -> isize {
    let pivot = hi as usize;
    let mut i = lo - 1;
    let mut j = hi;

    loop {
        i += 1;
        while arr[i as usize] < arr[pivot] {
            i += 1;
        }
        j -= 1;
        while j >= 0 && arr[j as usize] > arr[pivot] {
            j -= 1;
        }
        if i >= j {
            break;
        } else {
            arr.swap(i as usize, j as usize);
        }
    }
    arr.swap(i as usize, pivot as usize);
    i
}

fn _quick_sort<T: Ord>(arr: &mut [T], lo: isize, hi: isize) {
    if lo < hi {
        let p = partition(arr, lo, hi);
        _quick_sort(arr, lo, p - 1);
        _quick_sort(arr, p + 1, hi);
    }
}

pub fn quick_sort<T: Ord>(arr: &mut Vec<u32>) {
    let len = arr.len();
    if len > 1 {
        _quick_sort(arr, 0, (len - 1) as isize);
    }
}
