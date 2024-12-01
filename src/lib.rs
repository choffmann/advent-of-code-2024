pub trait Problem {
    fn part_one(&self, input: &str) -> String;
    fn part_two(&self, input: &str) -> String;

    fn run(&self, day: u8) {
        let input = read_input(day);
        println!("Part one: {}", self.part_one(&input));
        println!("Part two: {}", self.part_two(&input));
    }
}

pub fn read_input(day: u8) -> String {
    let path = format!("inputs/day{:02}.txt", day);
    std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("Could not read file {}", &path))
}
