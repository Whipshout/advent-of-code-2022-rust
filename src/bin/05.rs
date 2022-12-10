use itertools::Itertools;

type Stacks = Vec<Vec<char>>;
type Cmd = (usize, usize, usize);

#[aoc::main(05)]
fn main(input: &str) -> (String, String) {
    let (stacks, cmds) = input.split_once("\n\n").unwrap();

    let raw_stacks: Stacks = stacks
        .lines()
        .map(|l| l.chars().skip(1).step_by(4).collect::<Vec<char>>())
        .collect();

    let mut stacks: Stacks = transpose(raw_stacks); // column <==> row

    let cmds: Vec<Cmd> = cmds
        .lines()
        .map(|l| {
            l.split_whitespace()
                .skip(1)
                .step_by(2)
                .map(|n| n.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    (part1(&mut stacks.clone(), &cmds), part2(&mut stacks, &cmds))
}

fn part1(stacks: &mut Stacks, cmds: &[Cmd]) -> String {
    move_stacks(stacks, cmds, true);

    stacks.iter().map(|x| x.last().unwrap()).join("")
}

fn part2(stacks: &mut Stacks, cmds: &[Cmd]) -> String {
    move_stacks(stacks, cmds, false);

    stacks.iter().map(|x| x.last().unwrap()).join("")
}

fn transpose(stacks: Stacks) -> Vec<Vec<char>> {
    let len = stacks[0].len();

    let mut iters: Vec<_> = stacks.into_iter().map(|n| n.into_iter()).collect();

    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .rev()
                .filter(|c| !c.is_whitespace())
                .collect::<Vec<char>>()
        })
        .collect()
}

fn move_stacks(stacks: &mut Stacks, cmds: &[Cmd], reverse: bool) {
    cmds.iter().for_each(|c| {
        let current_count = stacks[c.1 - 1].len();

        let mut elements_to_move: Vec<char> =
            stacks[c.1 - 1].drain(current_count - c.0..).collect();

        if reverse {
            elements_to_move.reverse()
        }

        stacks[c.2 - 1].extend(elements_to_move);
    });
}
