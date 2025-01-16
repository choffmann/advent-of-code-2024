use aoc_2024::Problem;
use core::str;

struct Day16;

impl Day16 {}

impl Problem for Day16 {
    fn part_one(&self, input: &str) -> String {
        let mut maze: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
        let player = maze
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter().enumerate().find_map(
                    |(x, &c)| {
                        if c == 'S' {
                            Some((x, y))
                        } else {
                            None
                        }
                    },
                )
            })
            .unwrap();

        let end = maze
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter().enumerate().find_map(
                    |(x, &c)| {
                        if c == 'E' {
                            Some((x, y))
                        } else {
                            None
                        }
                    },
                )
            })
            .unwrap();

        let mut queue = vec![(player, 0)];

        while let Some((pos, steps)) = queue.pop() {
            println!("{} {}", pos.0, pos.1);
            let (x, y) = pos;
            if pos == end {
                return steps.to_string();
            }

            if maze[y][x] == '#' {
                continue;
            }

            maze[y][x] = '#';

            queue.push(((x + 1, y), steps + 1));
            queue.push(((x - 1, y), steps + 1));
            queue.push(((x, y + 1), steps + 1));
            queue.push(((x, y - 1), steps + 1));
        }

        "0".to_string()
    }

    fn part_two(&self, input: &str) -> String {
        "todo".to_string()
    }
}

fn main() {
    Day16.run(16);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

    #[test]
    fn part_one() {
        let day = Day16;
        assert_eq!(day.part_one(TEST_INPUT), "1928");
    }

    #[test]
    fn part_two() {}
}
