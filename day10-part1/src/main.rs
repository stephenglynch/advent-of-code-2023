use core::panic;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Tile {
    tile_type: char,
    steps: usize
}

struct Maze {
    len_x: usize,
    grid: Vec<Tile>
}

impl Maze {
    fn get_start(&self) -> (usize, usize) {
        for (i, t) in self.grid.iter().enumerate() {
            if t.tile_type == 'S' {
                return (i % self.len_x, i / self.len_x)
            }
        }
        panic!()
    }

    fn get(&self, x: usize, y: usize) -> Tile {
        let i = y * self.len_x + x;
        self.grid[i]
    }

    fn step(&self, prev_x: usize, prev_y: usize, x: usize, y: usize) -> Option<Tile> {
        for (step_x, step_y) in [(-1isize, -1isize), (-1, 1), (1, 1), (1, -1)] {
            let next_x = usize::try_from((x as isize + step_x));
            let next_y = usize::try_from((y as isize + step_y));

            // Account for walls
            let next_x = match next_x {
                Ok(x) => x,
                Err(e) => return None
            };
            let next_y = match next_y {
                Ok(y) => y,
                Err(e) => return None
            };

            // Skip the previous step
            if next_x == prev_x && next_y == prev_y {
                continue;
            }

            let tile = self.get(next_x, next_y);
            
        }
        None
    }

    fn explore(&self, x: usize, y: usize) {
        let (prev_x, prev_y) = self.get_start();
        loop {

        }
    }

    fn find_furthest_point(self) -> usize {
        // try north
        
    }
}

fn parse_maze(input: &str) -> Maze {
    let mut grid = vec![];
    for line in input.lines() {
        for c in line.chars() {
            grid.push(Tile {tile_type: c, steps: 0});
        }
    }

    let len_x = input.lines().next().unwrap().chars().count();

    Maze {len_x: len_x, grid: grid}
}

fn main() {
    let contents = fs::read_to_string("input/input.txt").unwrap();
    let contents = contents.trim();

    let maze = parse_maze(contents);
}
