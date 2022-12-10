#[aoc::main(10)]
fn main(input: &str) -> (i16, String) {
    let instructions: Vec<(i16, i16)> = input
        .lines()
        .map(|l| match l.split_once(' ').unwrap_or((l, "")) {
            ("noop", _) => (1, 0),
            ("addx", n) => (2, n.parse().unwrap()),
            _ => unreachable!(),
        })
        .collect();

    let (_, _, part1, part2) = instructions.into_iter().fold(
        // current value, current cycle, part1, part2
        (1, 0, 0, String::with_capacity(40 * 6)),
        |(v, mut c, mut p1, mut p2), (cycles, amount)| {
            (0..cycles).for_each(|_| {
                let is_visible = (c as i16 % 40).abs_diff(v) <= 1;

                c += 1;

                p1 += (c % 40 == 20) as i16 * c * v;
                p2.push(if is_visible { '#' } else { '.' });

                if c % 40 == 0 {
                    p2.push('\n');
                }
            });

            (v + amount, c, p1, p2)
        },
    );

    (part1, part2)
}
