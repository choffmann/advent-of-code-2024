use core::str;
use std::char;

use aoc_2024::Problem;

struct Day04;

impl Day04 {
    fn find_xmas(&self, fields: &[Vec<char>]) -> usize {
        let directions = [
            (0, 1),   // horizontal right
            (0, -1),  // horizontal left
            (1, 0),   // vertical down
            (-1, 0),  // vertical up
            (1, 1),   // diagonal down right
            (-1, -1), // diagonal up left
            (1, -1),  // diagonal down left
            (-1, 1),  // diagonal up right
        ];

        let target = "XMAS";
        let mut count = 0;

        for (i, row) in fields.iter().enumerate() {
            for (j, _) in row.iter().enumerate() {
                for &(di, dj) in &directions {
                    let mut word = String::new();
                    for k in 0..target.len() {
                        let ni = i as isize + k as isize * di;
                        let nj = j as isize + k as isize * dj;

                        if ni < 0
                            || nj < 0
                            || ni >= fields.len() as isize
                            || nj >= fields[0].len() as isize
                        {
                            break;
                        }

                        word.push(fields[ni as usize][nj as usize]);
                    }

                    if word == target {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}

impl Problem for Day04 {
    fn part_one(&self, input: &str) -> String {
        let fields: Vec<Vec<char>> = input
            .trim()
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        let count = self.find_xmas(&fields);
        count.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        "todo".to_string()
    }
}

fn main() {
    Day04.run(4);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    #[test]
    fn part_one() {
        let day = Day04;
        assert_eq!(day.part_one(TEST_INPUT), "18");
    }

    #[test]
    fn part_two() {
        let day = Day04;
        assert_eq!(day.part_one(TEST_INPUT), "9");
    }
}
