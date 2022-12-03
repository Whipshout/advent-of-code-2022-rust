#[aoc::main(02)]
fn main(input: &str) -> (u32, u32) {
    let games: Vec<(u32, u32)> = input
        .lines()
        .filter_map(|l| {
            let bytes = l.as_bytes();
            Some(((bytes.first()? - 65) as u32, (bytes.last()? - 88) as u32))
        })
        .collect();

    (part1(&games), part2(&games))
}

fn part1(games: &[(u32, u32)]) -> u32 {
    games.iter().map(|play| calculate_score(*play)).sum()
}

fn part2(games: &[(u32, u32)]) -> u32 {
    games
        .iter()
        .map(|play| {
            let next_move = match play.1 {
                0 => (play.0 + 2) % 3,
                1 => play.0,
                2 => (play.0 + 1) % 3,
                _ => unreachable!(),
            };
            calculate_score((play.0, next_move))
        })
        .sum()
}

fn calculate_score(play: (u32, u32)) -> u32 {
    let score = (3 - (2 + play.0 - play.1) % 3) % 3 * 3;
    score + play.1 + 1
}
