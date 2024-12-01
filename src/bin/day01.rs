use std::collections::HashMap;

use aoc_2024::Problem;

struct Day01;

impl Problem for Day01 {
    fn part_one(&self, input: &str) -> String {
        let (mut lhs, mut rhs): (Vec<u32>, Vec<u32>) = input
            .lines()
            .filter_map(|line| {
                let mut parts = line.split_whitespace();
                Some((
                    parts.next()?.parse::<u32>().ok()?,
                    parts.next()?.parse::<u32>().ok()?,
                ))
            })
            .unzip();

        lhs.sort_unstable();
        rhs.sort_unstable();

        let distance: u32 = lhs.iter().zip(&rhs).map(|(l, r)| l.abs_diff(*r)).sum();
        distance.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let (lhs, rhs): (Vec<u32>, Vec<u32>) = input
            .lines()
            .filter_map(|line| {
                let mut parts = line.split_whitespace();
                Some((
                    parts.next()?.parse::<u32>().ok()?,
                    parts.next()?.parse::<u32>().ok()?,
                ))
            })
            .unzip();

        let rhs_counts: HashMap<u32, usize> =
            rhs.iter().copied().fold(HashMap::new(), |mut acc, x| {
                *acc.entry(x).or_insert(0) += 1;
                acc
            });

        let sum: usize = lhs.iter().fold(0, |acc, &l| {
            acc + rhs_counts.get(&l).copied().unwrap_or(0) * l as usize
        });

        sum.to_string()
    }
}

fn main() {
    Day01.run(1);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn part_one() {
        let day01 = Day01;
        assert_eq!(day01.part_one(TEST_INPUT), "11");
    }

    #[test]
    fn part_two() {
        let day01 = Day01 {};
        assert_eq!(day01.part_two(TEST_INPUT), "31");
    }
}
