use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Display, Formatter},
    io::{stdin, stdout, Read, Write},
};

use aoc_2024::Problem;

struct Day06;

#[derive(Clone)]
struct Map(Vec<Vec<Point>>);

#[derive(Clone, Copy)]
struct Point {
    x: i8,
    y: i8,
    v: char,
    visited_dir: Option<Direction>,
}

impl Point {
    fn is_obstacle(&self) -> bool {
        self.v == '#' || self.v == 'O'
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.v)
    }
}

impl Map {
    fn at(&self, x: usize, y: usize) -> Option<&Point> {
        if y >= self.len() || x >= self.0[y].len() {
            return None;
        } else {
            Some(&self.0[y][x])
        }
    }

    fn player_start(&self) -> (usize, usize) {
        self.0
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
            .unwrap()
    }

    fn set(&mut self, x: usize, y: usize, value: char) {
        self.0[y][x] = Point {
            x: x as i8,
            y: y as i8,
            v: value,
            visited_dir: None,
        };
    }

    fn visit(&mut self, x: usize, y: usize, dir: Direction) {
        self.set(x, y, 'X');
        self.0[y][x].visited_dir = Some(dir);
    }

    fn place_obstacle(&mut self, x: usize, y: usize) {
        self.set(x, y, 'O');
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

#[derive(Clone, Copy, PartialEq, Debug)]
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
    fn step(&mut self, map: &mut Map, write: bool) -> Option<Point> {
        let (dx, dy) = self.dir.clone().into();
        let (x, y) = ((self.pos.x as i8 + dx) as usize, (self.pos.y + dy) as usize);

        if y > map.len() || map.row(y).map_or(true, |row| x > row.len()) {
            return None;
        }

        if map.at(x, y)?.is_obstacle() {
            self.dir = self.dir.next();
            let point = map.at(x, y).unwrap().clone(); // TODO: Fix this
            return Some(point);
        }

        self.pos.x += dx;
        self.pos.y += dy;
        if write {
            map.visit(x, y, self.dir.clone());
        }
        let point = map.at(x, y).unwrap().clone(); // TODO: Fix this
        return Some(point);
    }

    fn peek(&self, map: &Map) -> Option<Point> {
        let (dx, dy) = self.dir.clone().into();
        let (x, y) = ((self.pos.x as i8 + dx) as usize, (self.pos.y + dy) as usize);

        if y > map.len() || map.row(y).map_or(true, |row| x > row.len()) {
            return None;
        }

        map.at(x, y).cloned()
    }
}

impl Day06 {
    fn new_map(&self, input: &str) -> Map {
        let points = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| Point {
                        x: 0,
                        y: 0,
                        v: c,
                        visited_dir: None,
                    })
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
                visited_dir: None,
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
        while let Some(point) = player.step(&mut map, true) {
            sum.insert((point.x, point.y));
        }

        sum.len().to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let mut map = self.new_map(input);
        let mut obstacles = self.new_player(&map);

        let mut sum = 0;
        while obstacles.step(&mut map, false).is_some() {
            let mut new_map = map.clone();
            let mut player = self.new_player(&new_map);

            new_map.place_obstacle(obstacles.pos.x as usize, obstacles.pos.y as usize);

            let mut visited = HashMap::new();
            while let Some(point) = player.step(&mut new_map, true) {
                println!(
                    "Player: x: {}, y: {}, dir: {:?})",
                    player.pos.x, player.pos.y, player.dir
                );
                println!(
                    "Point: x: {}, y: {}, v: {}, visited: {:?})",
                    point.x, point.y, point.v, point.visited_dir
                );
                println!("Loops: {}", sum);

                println!("{}", new_map);
                pause();
                visited.insert((point.x, point.y), player.dir);

                if let Some(peek_point) = player.peek(&new_map) {
                    if let Some(dir) = visited.get(&(point.x, point.y)) {
                        if let Some(peek_point_dir) = peek_point.visited_dir {
                            if peek_point_dir == *dir {
                                println!("Loop detected");
                                sum += 1;
                                break;
                            }
                        }
                    }
                }
            }
        }

        sum.to_string()
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
