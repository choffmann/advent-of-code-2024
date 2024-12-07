use core::str;

use aoc_2024::Problem;

struct Day07;

impl Day07 {
    fn parse_input(&self, input: &str) -> Vec<(usize, Vec<usize>)> {
        input
            .lines()
            .map(|line| {
                let mut parts = line.split(": ");
                let sum = parts.next().unwrap().parse::<usize>().unwrap();
                let parts = parts
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();
                (sum, parts)
            })
            .collect()
    }
}

fn check(numbers: &[usize], idx: usize, current_sum: usize, result: usize) -> bool {
    if idx == numbers.len() {
        return current_sum == result;
    } else {
        let op1 = current_sum + numbers[idx];
        let op2 = current_sum * numbers[idx];
        return check(numbers, idx + 1, op1, result) || check(numbers, idx + 1, op2, result);
    }
}

impl Problem for Day07 {
    fn part_one(&self, input: &str) -> String {
        self.parse_input(input)
            .iter()
            .filter_map(|(sum, parts)| {
                for i in 0..parts.len() {
                    if check(&parts, i, 0, *sum) {
                        return Some(*sum);
                    }
                }

                None
            })
            .sum::<usize>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        "todo".to_string()
    }
}

fn main() {
    Day07.run(7);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn part_one() {
        let day = Day07;
        assert_eq!(day.part_one(TEST_INPUT), "3749");
    }

    #[test]
    fn part_two() {}
}
