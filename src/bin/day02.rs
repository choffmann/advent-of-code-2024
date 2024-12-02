use aoc_2024::Problem;

struct Day02;

#[derive(PartialEq, Debug, Clone)]
enum LineOrientation {
    NoInit,
    Bigger,
    Smaller,
}

impl Day02 {
    fn read_input(&self, input: &str) -> Vec<Vec<u32>> {
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|n| n.parse::<u32>().expect("Zahl erwartet"))
                    .collect()
            })
            .collect()
    }

    fn check_orientation(&self, current: u32, next: u32) -> LineOrientation {
        if next > current {
            LineOrientation::Bigger
        } else {
            LineOrientation::Smaller
        }
    }

    fn is_valid_transition(&self, current: u32, next: u32, orientation: &LineOrientation) -> bool {
        let diff = next.abs_diff(current);
        match orientation {
            LineOrientation::Bigger => next > current && (1..=3).contains(&diff),
            LineOrientation::Smaller => next < current && (1..=3).contains(&diff),
            _ => false,
        }
    }

    fn is_line_valid(&self, line: &[u32]) -> bool {
        let mut orientation = LineOrientation::NoInit;

        for pair in line.windows(2) {
            let (current, next) = (pair[0], pair[1]);

            if orientation == LineOrientation::NoInit {
                orientation = self.check_orientation(current, next);
            }

            if !self.is_valid_transition(current, next, &orientation) {
                return false;
            }
        }

        true
    }

    fn can_be_fixed(&self, line: &[u32]) -> bool {
        (0..line.len()).any(|i| {
            let modified_line = line
                .iter()
                .enumerate()
                .filter_map(|(idx, &val)| if idx == i { None } else { Some(val) })
                .collect::<Vec<_>>();
            self.is_line_valid(&modified_line)
        })
    }
}

impl Problem for Day02 {
    fn part_one(&self, input: &str) -> String {
        let valid_count = self.read_input(input).iter().fold(0, |acc, line| {
            if self.is_line_valid(line) {
                return acc + 1;
            }

            acc
        });

        valid_count.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let valid_count = self.read_input(input).iter().fold(0, |acc, line| {
            if self.is_line_valid(line) || self.can_be_fixed(line) {
                return acc + 1;
            }

            acc
        });

        valid_count.to_string()
    }
}

fn main() {
    Day02.run(2);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn part_one() {
        let day = Day02;
        assert_eq!(day.part_one(TEST_INPUT), "2");
    }

    #[test]
    fn part_two() {
        let day = Day02;
        assert_eq!(day.part_two(TEST_INPUT), "4");
    }
}
