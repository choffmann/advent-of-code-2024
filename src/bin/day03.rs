use aoc_2024::Problem;

struct Day03;

impl Day03 {
    fn extract_muls(&self, input: &str) -> Vec<(i32, i32, bool)> {
        let re = regex::Regex::new(r"(do(?:n't)?\(\)|mul\((\d{1,3}),(\d{1,3})\))").unwrap();
        let mut is_on = true;

        re.captures_iter(input)
            .filter_map(|caps| match caps.get(0).map(|m| m.as_str()) {
                Some("don't()") => {
                    is_on = false;
                    None
                }
                Some("do()") => {
                    is_on = true;
                    None
                }
                Some(_) => caps.get(2).zip(caps.get(3)).map(|(first, second)| {
                    (
                        first.as_str().parse::<i32>().unwrap(),
                        second.as_str().parse::<i32>().unwrap(),
                        is_on,
                    )
                }),
                None => None,
            })
            .collect()
    }
}

impl Problem for Day03 {
    fn part_one(&self, input: &str) -> String {
        self.extract_muls(input)
            .iter()
            .map(|&(first, second, _)| first * second)
            .sum::<i32>()
            .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        self.extract_muls(input)
            .iter()
            .map(|&(first, second, is_on)| if is_on { first * second } else { 0 })
            .sum::<i32>()
            .to_string()
    }
}

fn main() {
    Day03.run(3);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const TEST_INPUT_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn part_one() {
        let day = Day03;
        assert_eq!(day.part_one(TEST_INPUT), "161");
    }

    #[test]
    fn part_two() {
        let day = Day03;
        assert_eq!(day.part_two(TEST_INPUT_2), "48");
    }
}

