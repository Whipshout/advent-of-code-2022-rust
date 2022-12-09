use hashbrown::HashSet;

type Action = (i16, i16, i16);

#[aoc::main(09)]
fn main(input: &str) -> (usize, usize) {
    let actions: Vec<Action> = input
        .lines()
        .map(|l| {
            let (direction, steps) = l.split_once(' ').unwrap();

            let (x, y) = match direction {
                "L" => (-1, 0),
                "R" => (1, 0),
                "U" => (0, 1),
                "D" => (0, -1),
                _ => unreachable!(),
            };

            (x, y, steps.parse::<i16>().unwrap())
        })
        .collect();

    (part1(&actions), part2(&actions))
}

fn part1(actions: &[Action]) -> usize {
    move_rope(actions, 1)
}

fn part2(actions: &[Action]) -> usize {
    move_rope(actions, 9)
}

fn move_rope(actions: &[Action], knots: usize) -> usize {
    let mut rope: Vec<(i16, i16)> = vec![(0, 0); knots + 1];

    let mut visited: HashSet<(i16, i16)> = HashSet::new();
    visited.insert((0, 0));

    // Iter over actions
    actions.iter().for_each(|(x, y, steps)| {
        (0..*steps).for_each(|_| {
            rope[0].0 += x;
            rope[0].1 += y;

            // Iter over knots
            (1..rope.len()).for_each(|i| {
                let (x_ahead, y_ahead) = rope[i - 1]; // Previous knot

                // Check distance between points
                if (x_ahead - rope[i].0).abs() > 1 || (y_ahead - rope[i].1).abs() > 1 {
                    // >0, signum return 1 | <0, signum return -1 | 0, signum return 0
                    rope[i].0 += (x_ahead - rope[i].0).signum();
                    rope[i].1 += (y_ahead - rope[i].1).signum();
                }
            });

            visited.insert(*rope.last().unwrap());
        });
    });

    visited.len()
}
