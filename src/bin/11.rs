use itertools::Itertools;

type Operation = (u64, u64); // Type, value
type Monkey = (Vec<u64>, Operation, u64, usize, usize); // Items, operation, divisible, true case, false case

#[aoc::main(11)]
fn main(input: &str) -> (usize, usize) {
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|m| {
            let lines: Vec<&str> = m.lines().skip(1).collect();

            let items: Vec<_> = lines[0].split(':').into_iter().collect::<Vec<&str>>()[1]
                .split(',')
                .map(|s| s.trim().parse::<u64>().unwrap())
                .collect();
            let divisible = lines[2].split(' ').last().unwrap().parse().unwrap();
            let true_m = lines[3].split(' ').last().unwrap().parse().unwrap();
            let false_m = lines[4].split(' ').last().unwrap().parse().unwrap();
            let op: Operation = if lines[1].contains("old * old") {
                (0, 0) // old * old
            } else {
                let amount = lines[1].split_whitespace().last().unwrap().parse().unwrap();
                if lines[1].contains('+') {
                    (1, amount) // old + i
                } else {
                    (2, amount) // old * i
                }
            };

            (items, op, divisible, true_m, false_m)
        })
        .collect();

    (part1(&mut monkeys.clone()), part2(&mut monkeys))
}

fn part1(monkeys: &mut Vec<Monkey>) -> usize {
    play(monkeys, 20, |x| x / 3)
}

fn part2(monkeys: &mut Vec<Monkey>) -> usize {
    let common = monkeys.iter().map(|m| m.2).product::<u64>();

    play(monkeys, 10000, |x| x % common)
}

fn play(monkeys: &mut Vec<Monkey>, rounds: usize, func: impl Fn(u64) -> u64) -> usize {
    let mut inspections = vec![0; monkeys.len()];

    // Not happy with this, a lot of issues with ownership
    // Revisit to try to refactor and make the borrow checker happy
    (0..rounds).for_each(|_| {
        (0..monkeys.len()).for_each(|i| {
            let m = monkeys[i].clone();

            m.0.iter().for_each(|i| {
                let w = match m.1 .0 {
                    0 => func(i * i),
                    1 => func(i + m.1 .1),
                    2 => func(i * m.1 .1),
                    _ => unreachable!(),
                };

                monkeys[if w % m.2 == 0 { m.3 } else { m.4 }].0.push(w);
            });

            inspections[i] += monkeys[i].0.len();
            monkeys[i].0.clear();
        });
    });

    inspections
        .iter()
        .sorted_by(|a, b| b.cmp(a))
        .take(2)
        .product()
}
