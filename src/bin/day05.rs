use core::str;
use std::cmp::Ordering;

use aoc_2024::Problem;

struct Day05;

impl Day05 {
    fn parse_rules(&self, input: &str) -> Vec<(u32, u32)> {
        input
            .lines()
            .take_while(|l| !l.is_empty())
            .map(|l| {
                let mut parts = l.split('|');
                let a = parts.next().unwrap().parse().unwrap();
                let b = parts.next().unwrap().parse().unwrap();
                (a, b)
            })
            .collect()
    }

    fn parse_prints(&self, input: &str) -> Vec<Vec<u32>> {
        input
            .lines()
            .skip_while(|l| !l.is_empty())
            .skip(1)
            .map(|l| {
                l.split(',')
                    .map(|n| n.parse().unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect()
    }

    fn is_valid(&self, rules: &[(u32, u32)], print: &[u32]) -> bool {
        print
            .windows(2)
            .all(|x| rules.iter().any(|(a, b)| x[0] == *a && x[1] == *b))
    }
}

impl Problem for Day05 {
    fn part_one(&self, input: &str) -> String {
        let rules = self.parse_rules(input);
        let prints = self.parse_prints(input);

        prints
            .iter()
            .filter(|p| self.is_valid(&rules, p))
            .map(|p| p[p.len() / 2])
            .sum::<u32>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let rules = self.parse_rules(input);
        let prints = self.parse_prints(input);

        prints
            .into_iter()
            .filter(|p| !self.is_valid(&rules, p))
            .map(|mut p| {
                p.sort_by(|&a, &b| {
                    if rules.iter().any(|(x, y)| a == *x && b == *y) {
                        Ordering::Less
                    } else if rules.iter().any(|(x, y)| a == *y && b == *x) {
                        Ordering::Greater
                    } else {
                        Ordering::Equal
                    }
                });

                p
            })
            .map(|p| p[p.len() / 2])
            .sum::<u32>()
            .to_string()
    }
}

fn main() {
    Day05.run(5);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    fn part_one() {
        let day = Day05;
        assert_eq!(day.part_one(TEST_INPUT), "143");
    }

    #[test]
    fn part_two() {
        let day = Day05;
        assert_eq!(day.part_two(TEST_INPUT), "123");
    }
}
