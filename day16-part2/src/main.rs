use std::fs;
use std::str;

struct Grid {
    grid: Vec<char>,
    visited: Vec<Vec<Dir>>,
    xlen: usize,
    ylen: usize
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Dir {
    North,
    East,
    West,
    South
}

impl Grid {
    fn get(&self, coord: (usize, usize)) -> char {
        let (x, y) = coord;
        self.grid[y * self.xlen + x]
    }

    fn visit(&mut self, coord: (usize, usize), dir: Dir) -> bool {
        let (x, y) = coord;
        let dirs = &mut self.visited[y * self.xlen + x];
        if dirs.contains(&dir) {
            true
        } else {
            dirs.push(dir);
            false
        }
    }

    fn reset_visited(&mut self) {
        self.visited = vec![vec![]; self.xlen * self.ylen];
    }

    fn get_edges(&self) -> Vec<((usize, usize), Dir)> {
        let mut points = vec![];
        points.extend((1..self.xlen).map(|x| ((x, 0), Dir::South)));
        points.extend((1..self.xlen).map(|x| ((x, self.ylen-1), Dir::North)));
        points.extend((1..self.ylen).map(|y| ((0, y), Dir::East)));
        points.extend((1..self.ylen).map(|y| ((self.xlen-1, y), Dir::West)));
        points.append(&mut vec![
            ((0, 0), Dir::South), 
            ((0, 0), Dir::East), 
            ((self.xlen-1, 0), Dir::South),
            ((self.xlen-1, 0), Dir::West), 
            ((0, self.ylen-1), Dir::North),
            ((0, self.ylen-1), Dir::East), 
            ((self.xlen-1, self.ylen-1), Dir::North),
            ((self.xlen-1, self.ylen-1), Dir::East)]);
        points
    }

    fn total_visited(&self) -> usize {
        let mut total = 0;
        for e in self.visited.iter() {
            if e.len() > 0 {
                total += 1;
            }
        }
        total
    }

    fn next_coord(&self, coord: (usize, usize), dir: Dir) -> Option<(usize, usize)> {
        let (x, y) = coord;
        let x = x as i32;
        let y = y as i32;
        let next = match dir {
            Dir::North => (x, y - 1),
            Dir::East => (x + 1, y),
            Dir::South => (x, y + 1),
            Dir::West => (x - 1, y)
        };
        let (x, y) = next;
        if x < 0 || x >= self.xlen as i32 {
            return None
        }
        if y < 0 || y >= self.ylen as i32 {
            return None
        }
        Some((next.0 as usize, next.1 as usize))
    }

    fn propagate(&mut self, coord: (usize, usize), dir: Dir) {
        if self.visit(coord, dir) {
            return
        }
        let new_dirs = match (self.get(coord), dir) {
            ('|', Dir::East | Dir::West) => vec![Dir::South, Dir::North],
            ('-', Dir::North | Dir::South) => vec![Dir::East, Dir::West],
            ('\\', Dir::North) => vec![Dir::West],
            ('\\', Dir::East) => vec![Dir::South],
            ('\\', Dir::South) => vec![Dir::East],
            ('\\', Dir::West) => vec![Dir::North],
            ('/', Dir::North) => vec![Dir::East],
            ('/', Dir::East) => vec![Dir::North],
            ('/', Dir::South) => vec![Dir::West],
            ('/', Dir::West) => vec![Dir::South],
            _ => vec![dir]
        };

        for d in new_dirs {
            if let Some(next) = self.next_coord(coord, d) {
                self.propagate(next, d);
            }
        }
    }
}

fn parse_grid(input: &str) -> Grid {
    let grid = input.chars().filter(|c| *c != '\n').collect();
    let xlen = input.lines().nth(0).unwrap().len();
    let ylen = input.lines().count();
    let visited = vec![vec![]; xlen * ylen];
    Grid {grid: grid, xlen: xlen, ylen: ylen, visited: visited}
}

fn main() {
    let contents = fs::read_to_string("input/input.txt").unwrap();
    let contents = contents.trim();
    let mut grid = parse_grid(contents);
    let points = grid.get_edges();
    let answer = points.into_iter().map(|(coord, dir)| {
        grid.propagate(coord, dir);
        let total = grid.total_visited();
        grid.reset_visited();
        total
    }).max().unwrap();

    println!("answer = {}", answer);
}
