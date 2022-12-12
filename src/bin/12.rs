use pathfinding::prelude::bfs;

const NEIGHBOURS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

type Map = Vec<Vec<u8>>;

#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

impl Pos {
    fn successors(&self, map: &Map, rev: bool) -> Vec<Pos> {
        let &Pos(x, y) = self;

        let max_h = if rev { b'z' - map[y][x] } else { map[y][x] } + 1;

        let mut possible_moves = Vec::with_capacity(4);

        for (nx, ny) in NEIGHBOURS {
            if x == 0 && nx == -1 {
                continue;
            }
            if y == 0 && ny == -1 {
                continue;
            }
            if x == map[0].len() - 1 && nx == 1 {
                continue;
            }
            if y == map.len() - 1 && ny == 1 {
                continue;
            }

            let sx = ((x as i32) + nx) as usize;
            let sy = ((y as i32) + ny) as usize;

            if max_h >= (if rev { b'z' - map[sy][sx] } else { map[sy][sx] }) {
                possible_moves.push(Pos(sx, sy));
            }
        }

        possible_moves
    }
}

#[aoc::main(12)]
fn main(input: &str) -> (usize, usize) {
    let height = input.lines().count() - 1;
    let width = input.lines().next().unwrap().len() - 1;

    let mut grid: Vec<Vec<_>> = Vec::with_capacity(height);

    let (mut start, mut goal) = (Pos::default(), Pos::default());

    input.lines().enumerate().for_each(|(y, line)| {
        let mut row = Vec::with_capacity(width);

        line.as_bytes()
            .iter()
            .enumerate()
            .for_each(|(x, c)| match c {
                &b'S' => {
                    start = Pos(x, y);
                    row.push(b'a');
                }
                &b'E' => {
                    goal = Pos(x, y);
                    row.push(b'z');
                }
                h => row.push(*h),
            });

        grid.push(row);
    });

    let part1 = bfs(&start, |p| p.successors(&grid, false), |p| *p == goal)
        .unwrap()
        .len()
        - 1;

    let part2 = bfs(
        &goal,
        |p| p.successors(&grid, true),
        |p| grid[p.1][p.0] == b'a',
    )
    .unwrap()
    .len()
        - 1;

    (part1, part2)
}
