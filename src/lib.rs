pub trait Problem {
    fn part_one(&self, input: &str) -> String;
    fn part_two(&self, input: &str) -> String;

    fn run(&self, day: u8) {
        let input = read_input(day);
        let now = std::time::Instant::now();
        let part_one = self.part_one(&input);
        let part_one_time = now.elapsed();

        let now = std::time::Instant::now();
        let part_two = self.part_two(&input);
        let part_two_time = now.elapsed();

        println!("Day {:02}:", day);
        println!("  Part 1: {} ({:?})", part_one, part_one_time);
        println!("  Part 2: {} ({:?})", part_two, part_two_time);
    }
}

pub fn read_input(day: u8) -> String {
    let path = format!("inputs/day{:02}.txt", day);
    std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("Could not read file {}", &path))
}
