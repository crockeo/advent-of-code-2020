use std::fs::File;
use std::io;
use std::io::Read;
use std::path::Path;

use crate::solution::ProblemSolution;

pub struct Solution {}

impl ProblemSolution for Solution {
    fn name(&self) -> &'static str {
        return "problem_03";
    }

    fn part1(&self) -> io::Result<i64> {
        let mut file = File::open(Path::new(".").join("data").join("03.txt").as_path())?;
        let tree_map = TreeMap::new(&mut file)?;

        Ok(tree_map.hits(3, 1) as i64)
    }

    fn part2(&self) -> io::Result<i64> {
        let mut file = File::open(Path::new(".").join("data").join("03.txt").as_path())?;
        let tree_map = TreeMap::new(&mut file)?;

        let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

        Ok(slopes
            .iter()
            .map(|(dx, dy)| tree_map.hits(*dx, *dy) as i64)
            .product())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum TreeTile {
    Open,
    Tree,
}

impl TreeTile {
    fn from_char(c: char) -> Option<TreeTile> {
        match c {
            '.' => Some(TreeTile::Open),
            '#' => Some(TreeTile::Tree),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
struct TreeMap {
    width: usize,
    height: usize,
    tiles: Vec<TreeTile>,
}

impl TreeMap {
    fn from_str(s: &str) -> Option<TreeMap> {
        let mut height = 0;
        let mut width = 0;
        let mut tiles = Vec::new();
        for line in s.lines() {
            if width == 0 {
                width = line.len()
            } else if width != line.len() {
                // the map is non-uniform so we just abort here
                return None;
            }
            height += 1;

            tiles.extend(line.chars().flat_map(TreeTile::from_char));
        }

        Some(TreeMap {
            width: width,
            height: height,
            tiles: tiles,
        })
    }

    fn new(file: &mut File) -> io::Result<TreeMap> {
        let mut s = String::new();
        file.read_to_string(&mut s)?;

        match TreeMap::from_str(s.as_str()) {
            None => Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "failed to parse input",
            )),
            Some(tree_map) => Ok(tree_map),
        }
    }

    fn at(&self, x: usize, y: usize) -> Option<TreeTile> {
        if y >= self.height {
            return None;
        }

        let x = x % self.width;
        Some(self.tiles[y * self.width + x])
    }

    fn hits(&self, dx: usize, dy: usize) -> u64 {
        let mut hit_count = 0;
        let mut x = 0;
        let mut y = 0;

        while y < self.height {
            // safe bc we check the height
            let tile = self.at(x, y).unwrap();
            if tile == TreeTile::Tree {
                hit_count += 1;
            }

            x += dx;
            y += dy;
        }

        hit_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_input() {
        let input = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

        let maybe_tree_map = TreeMap::from_str(input);
        assert_ne!(maybe_tree_map, None);

        let tree_map = maybe_tree_map.unwrap();
        assert_eq!(tree_map.hits(3, 1), 7);
    }
}
