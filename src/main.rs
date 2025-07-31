use rand::{Rng, rng};

struct Grid {
    width: usize,
    height: usize,
    grid: Vec<Vec<bool>>,
}

impl Grid {
    fn new(height: usize, width: usize) -> Grid {
        Grid {
            width,
            height,
            grid: vec![vec![false; width]; height],
        }
    }

    fn randomize(&mut self) {
        let mut rng = rng();
        for y in 0..self.height {
            for x in 0..self.width {
                self.grid[y][x] = rng.random_bool(0.5);
            }
        }
    }

    fn get(&self, x: usize, y: usize) -> bool {
        if x < self.width && y < self.height {
            self.grid[y][x]
        } else {
            self.grid[y % self.height][x % self.width]
        }
    }

    fn neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                if self.get(((x as isize) + dx) as usize, ((y as isize) + dy) as usize) {
                    count += 1;
                }
            }
        }
        count
    }

    fn switch(&mut self) {
        let mut next_buf = self.grid.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                next_buf[y][x] = match (self.get(x, y), self.neighbors(x, y)) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                };
            }
        }
        if self.grid == next_buf {
            self.end();
        } else {
            self.grid = next_buf
        }
    }

    fn toggle(&mut self, x: usize, y: usize) {
        self.grid[y][x] = !self.grid[y][x]
    }

    fn end(&self) {
        println!("No more moves");
        std::process::exit(0)
    }
}

fn main() {
    /*let termsize::Size { rows, cols } = termsize::get().unwrap();
    println!("          width  height");
    println!("termion:  {:4}    {:4}", cols, rows);*/

    //let mut grid = Grid::new(rows as usize, cols as usize);
    let mut grid = Grid::new(16, 16);

    grid.toggle(2, 2);
    grid.toggle(2, 3);
    grid.toggle(2, 4);
    grid.toggle(1, 4);
    grid.toggle(0, 3);

    loop {
        for y in 0..grid.height {
            for x in 0..grid.width {
                print!("{}", if grid.grid[y][x] { "■" } else { "◦" });
            }
            println!();
        }
        println!();
        grid.switch();
        //std::io::stdin().read_line(&mut String::new());
        std::thread::sleep(std::time::Duration::from_millis(250));
    }
}
