use std::{
    fmt,
    io::{stdout, Write},
    mem, thread,
    time::Duration,
};

use clap::Parser;
use crossterm::{
    cursor, style,
    terminal::{self, ClearType},
};
use rand::Rng;

struct Field {
    cells: Vec<Vec<bool>>,
    width: u16,
    height: u16,
}

impl Field {
    fn new(width: u16, height: u16) -> Self {
        Self {
            cells: vec![vec![false; width as usize]; height as usize],
            width,
            height,
        }
    }

    fn set(&mut self, x: u16, y: u16, val: bool) -> Result<(), &'static str> {
        if x >= self.width || y >= self.height {
            return Err("coordinates are out of bounds");
        }

        self.cells[y as usize][x as usize] = val;
        Ok(())
    }

    fn is_alive(&self, mut x: i32, mut y: i32) -> bool {
        x = x.rem_euclid(self.width as i32);
        y = y.rem_euclid(self.height as i32);
        self.cells[y as u16 as usize][x as u16 as usize]
    }

    fn next(&self, x: i32, y: i32) -> bool {
        const NEIGHBORS: [[i32; 2]; 8] = [
            [-1, -1],
            [0, -1],
            [1, -1],
            [-1, 0],
            [1, 0],
            [-1, 1],
            [0, 1],
            [1, 1],
        ];

        let mut alive_neighbors = 0u8;
        for [i, j] in NEIGHBORS {
            if self.is_alive(x + i, y + j) {
                alive_neighbors += 1;
            }
        }

        alive_neighbors == 3 || alive_neighbors == 2 && self.is_alive(x, y)
    }
}

struct Life {
    current: Field,
    next: Field,
    width: u16,
    height: u16,
}

impl Life {
    fn new(width: u16, height: u16) -> Self {
        let mut current = Field::new(width, height);
        let mut rng = rand::thread_rng();
        for _ in 0..width * height / 4 {
            current
                .set(rng.gen_range(0..width), rng.gen_range(0..height), true)
                .unwrap();
        }

        Self {
            current,
            next: Field::new(width, height),
            width,
            height,
        }
    }

    fn step(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.next
                    .set(x, y, self.current.next(x as i32, y as i32))
                    .unwrap();
            }
        }

        mem::swap(&mut self.current, &mut self.next);
    }
}

impl fmt::Display for Life {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(
                    f,
                    "{}",
                    if self.current.is_alive(x as i32, y as i32) {
                        "*"
                    } else {
                        " "
                    }
                )?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let config = Config::parse();

    let mut life = Life::new(config.width, config.height);

    loop {
        let mut stdout = stdout();
        crossterm::queue!(
            stdout,
            terminal::Clear(ClearType::All),
            terminal::Clear(ClearType::Purge),
            cursor::MoveTo(0, 0),
            style::Print(&life),
        )
        .unwrap();
        stdout.flush().unwrap();

        life.step();
        thread::sleep(Duration::from_secs(1) / config.fps);
    }
}

/// Conway's Game of Life
#[derive(Parser)]
#[command(about)]
struct Config {
    /// Width of the field
    #[arg(long, default_value_t = 40)]
    width: u16,
    /// Height of the field
    #[arg(long, default_value_t = 15)]
    height: u16,
    /// Approximate steps per second
    #[arg(long, default_value_t = 10)]
    fps: u32,
}
