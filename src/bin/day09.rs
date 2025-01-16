use aoc_2024::Problem;
use core::str;
use std::{collections::HashMap, str::FromStr};

struct Day09;

impl Day09 {}

enum DiskValue {
    Used(usize),
    Free,
}

#[derive(Debug, Clone)]
struct DiskGroup {
    used: usize,
    free: usize,
    value: usize,
}

impl DiskGroup {
    fn new(used: usize, free: usize) -> Self {
        DiskGroup { used, free }
    }

    fn can_fit(&self, disk: &DiskGroup) -> bool {
        self.free >= disk.used
    }

    fn can_fit_byte(&self, byte: usize) -> bool {
        self.free >= byte
    }

    fn append_byte(&mut self, byte: usize) {
        self.used += byte;
        self.free -= byte;
    }

    fn append_disk(&mut self, disk: &DiskGroup) {
        self.used += disk.used;
        self.free -= disk.used;
    }

    fn free_byte(&mut self) {
        let _ = self.used.wrapping_sub(1);
        let _ = self.free.wrapping_add(1);
    }

    fn add_byte(&mut self) {
        let _ = self.used.wrapping_add(1);
        let _ = self.free.wrapping_sub(1);
    }
}

#[derive(Debug)]
struct Disk {
    map: HashMap<usize, DiskGroup>,
}

impl Disk {
    fn shift_last(&mut self) {
        for i in (0..self.map.keys().len()).rev() {
            let disk = self.map.get(&i);
            let next_disk = self.map.get(&(i + 1));

            if next_disk.is_none() || disk.is_none() {
                continue;
            }

            let next_disk = next_disk.unwrap().clone();
            let disk = disk.unwrap();

            if next_disk.can_fit(&disk) {
                self.map.get_mut(&i).unwrap().append_disk(&next_disk);
                self.map.get_mut(&(i + 1)).unwrap().free_byte();
            } else {
                self.map.get_mut(&i).unwrap().add_byte();
                self.map.get_mut(&(i + 1)).unwrap().free_byte();
            }
        }
    }
}

impl FromStr for Disk {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<usize> = s
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();

        let map = lines
            .chunks(2)
            .enumerate()
            .fold(HashMap::new(), |mut acc, (i, chunk)| {
                let free = if chunk.len() == 1 { 0 } else { chunk[1] };
                acc.insert(i, DiskGroup::new(chunk[0], free));
                acc
            });

        Ok(Disk { map })
    }
}

impl Problem for Day09 {
    fn part_one(&self, input: &str) -> String {
        let mut disk = Disk::from_str(input).unwrap();

        println!("{:?}", disk);

        for _ in 0..disk.map.len() {
            disk.shift_last();
        }

        println!("{:?}", disk);

        "0".to_string()

        // chunks
        //     .iter()
        //     .fold(Vec::new(), |mut acc, x| {
        //         acc.push(x.clone());
        //         let last_use = acc.iter().rposition(|x| match x {
        //             Disk::Used(_) => true,
        //             _ => false,
        //         });

        //         let first_free = acc.iter().position(|x| match x {
        //             Disk::Used(_) => false,
        //             _ => true,
        //         });

        //         if last_use.is_some() && first_free.is_some() {
        //             let last_use = last_use.unwrap();
        //             let first_free = first_free.unwrap();
        //             acc[last_use] = Disk::Free;
        //             acc[first_free] = chunks[first_free].clone();
        //         }

        //         acc
        //     })
        //     .iter()
        //     .enumerate()
        //     .fold(0, |acc, (i, x)| match x {
        //         Disk::Used(v) => acc + (i * v),
        //         _ => acc,
        //     })
        //     .to_string()
    }

    fn part_two(&self, input: &str) -> String {
        let lines: Vec<usize> = input
            .trim()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();

        // let chunks = lines
        //     .chunks(2)
        //     .enumerate()
        //     .map(|(i, chunk)| {
        //         let free = if chunk.len() == 1 { 0 } else { chunk[1] };
        //         DiskGroup {
        //             id: i,
        //             used: chunk[0],
        //             free,
        //         }
        //     })
        //     .fold(Vec::new(), |acc, x| {
        //         let
        //     });

        // println!("{:?}", chunks);

        "0".to_string()
    }
}

fn main() {
    Day09.run(9);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "2333133121414131402";

    #[test]
    fn part_one() {
        let day = Day09;
        assert_eq!(day.part_one(TEST_INPUT), "1928");
    }

    #[test]
    fn part_two() {
        let day = Day09;
        assert_eq!(day.part_two(TEST_INPUT), "2858");
    }
}
