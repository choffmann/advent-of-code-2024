use std::{
    collections::HashSet,
    fmt::{self, Display, Formatter},
    io::{stdin, stdout, Read, Write},
};

use aoc_2024::Problem;

struct Day06;

struct Map(Vec<Vec<Point>>);

struct Point {
    x: i8,
    y: i8,
    v: char,
}

impl Point {
    fn is_obstacle(&self) -> bool {
        self.v == '#'
    }

    fn is_empty(&self) -> bool {
        self.v == '.'
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.v)
    }
}

impl Map {
    fn at(&self, x: usize, y: usize) -> &Point {
        &self.0[y][x]
    }

    fn set(&mut self, x: usize, y: usize, value: char) {
        self.0[y][x] = Point {
            x: x as i8,
            y: y as i8,
            v: value,
        };
    }

    fn row(&self, y: usize) -> Option<&[Point]> {
        if y >= self.len() {
            return None;
        }
        Some(&self.0[y])
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        for row in &self.0 {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn next(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

impl Into<(i8, i8)> for Direction {
    fn into(self) -> (i8, i8) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
}

struct Player {
    pos: Point,
    dir: Direction,
}

impl Player {
    fn step(&mut self, map: &mut Map) -> Option<&Point> {
        let (dx, dy) = self.dir.clone().into();
        let (x, y) = ((self.pos.x as i8 + dx) as usize, (self.pos.y + dy) as usize);

        if y >= map.len() || map.row(y).map_or(true, |row| x > row.len()) {
            return None;
        }

        if map.at(x, y).is_obstacle() {
            self.dir = self.dir.next();
            return Some(&self.pos);
        }

        self.pos.x += dx;
        self.pos.y += dy;
        map.set(self.pos.x as usize, self.pos.y as usize, 'X');

        Some(&self.pos)
    }
}

impl Day06 {
    fn new_map(&self, input: &str) -> Map {
        let points = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| Point { x: 0, y: 0, v: c })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Map(points)
    }

    fn new_player(&self, map: &Map) -> Player {
        let pos = map
            .0
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter().enumerate().find_map(
                    |(x, cell)| {
                        if cell.v == '^' {
                            Some((x, y))
                        } else {
                            None
                        }
                    },
                )
            })
            .unwrap();

        Player {
            pos: Point {
                x: pos.0 as i8,
                y: pos.1 as i8,
                v: '^',
            },
            dir: Direction::Up,
        }
    }
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press key to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

impl Problem for Day06 {
    fn part_one(&self, input: &str) -> String {
        let mut map = self.new_map(input);
        let mut player = self.new_player(&map);

        let mut sum = HashSet::new();
        let player_pos = &player.pos;
        sum.insert((player_pos.x, player_pos.y));
        while let Some(point) = player.step(&mut map) {
            sum.insert((point.x, point.y));
        }

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
    fn part_two() {
        let day = Day06;
        assert_eq!(day.part_two(TEST_INPUT), "6");
    }
}
