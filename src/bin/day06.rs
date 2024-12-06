use core::str;
use std::collections::HashSet;

use aoc_2024::Problem;

struct Day06;

impl Day06 {}

impl Problem for Day06 {
    fn part_one(&self, input: &str) -> String {
        let mut map = input
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let mut player_pos = map
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter().enumerate().find_map(
                    |(x, &cell)| {
                        if cell == '^' {
                            Some((x, y))
                        } else {
                            None
                        }
                    },
                )
            })
            .unwrap();

        let directions = [
            (0, -1), // up
            (1, 0),  // right
            (0, 1),  // down
            (-1, 0), // left
        ];

        let mut current_dir = 0;

        let mut sum = HashSet::new();
        sum.insert(player_pos);
        loop {
            let (dx, dy) = directions[current_dir];
            let (mut x, mut y) = player_pos;
            let mut exit = false;

            loop {
                x = (x as i32 + dx) as usize;
                y = (y as i32 + dy) as usize;

                if y >= map.len() || x >= map[y].len() {
                    exit = true;
                    break;
                }

                if map[y][x] == '#' {
                    current_dir = (current_dir + 1) % 4;
                    player_pos = (x.wrapping_sub(dx as usize), y.wrapping_sub(dy as usize));
                    break;
                }

                map[y][x] = 'X';
                sum.insert((x, y));
            }

            if exit {
                break;
            }
        }

        for row in &map {
            println!("{}", row.iter().collect::<String>());
        }

        println!("{:?}", player_pos);

        sum.len().to_string()
    }

    fn part_two(&self, input: &str) -> String {
        "todo".to_string()
    }
}

fn main() {
    Day06.run(6);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    #[test]
    fn part_one() {
        let day = Day06;
        assert_eq!(day.part_one(TEST_INPUT), "41");
    }

    #[test]
    fn part_two() {}
}
