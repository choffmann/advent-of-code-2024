use aoc_2024::Problem;

struct Day03;

impl Problem for Day03 {
    fn part_one(&self, input: &str) -> String {
        let re = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let nums: Vec<(i32, i32)> = re
            .captures_iter(input)
            .map(|caps| {
                let (_, [first, second]) = caps.extract();
                let first = first.parse::<i32>().unwrap();
                let second = second.parse::<i32>().unwrap();
                (first, second)
            })
            .collect();

        let sum: i32 = nums.iter().map(|&x| x.0 * x.1).sum();
        sum.to_string()
    }

    fn part_two(&self, input: &str) -> String {
        "todo".to_string()
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
