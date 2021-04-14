// Copyright (c) 2021 Malcolm Law
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use rand::{thread_rng, Rng};
use ansi_term::Colour;

const SIZE: usize = 5;

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Snake {
    len: u8,                  // snake length
    dir: Direction,           // head direction
    pos: (usize, usize),      // head position
    grid: [[u8; SIZE]; SIZE], // the map
    alive: bool,              // live or death
    fruit: (usize, usize),
}

impl Snake {
    pub fn step(&mut self) -> &mut Self {
        match &self.dir {
            Direction::Up => {
                self.pos.1 = (self.pos.1 + SIZE - 1) % SIZE;
            }
            Direction::Down => {
                self.pos.1 = (self.pos.1 + 1) % SIZE;
            }
            Direction::Left => {
                self.pos.0 = (self.pos.0 + SIZE - 1) % SIZE;
            }
            Direction::Right => {
                self.pos.0 = (self.pos.0 + 1) % SIZE;
            }
        }
        if self.grid[self.pos.1][self.pos.0] > 0 {
            self.alive = false;
        }
        self.updateGrid();
        self
    }
    pub fn turn(&mut self, dir: Direction) -> &mut Self {
        self.dir = dir;
        self
    }
    pub fn isAlive(&self) -> bool {
        self.alive
    }
    pub fn score(&self) -> u8 {
        self.len
    }
    fn updateGrid(&mut self) {
        if self.pos == self.fruit {
            self.len += 1;
            self.grid[self.pos.1][self.pos.0] = self.len;
            // generate new fruit position
            let mut friut = genPos();
            while self.grid[friut.1][friut.0] != 0 {
                friut = genPos();
            }
            self.fruit = friut;
        } else {
            for i in 0..SIZE {
                for j in 0..SIZE {
                    if self.grid[i][j] > 0 {
                        self.grid[i][j] -= 1;
                    }
                }
            }
            self.grid[self.pos.1][self.pos.0] = self.len;
        }
    }
}

pub fn newSnake(pos: (usize, usize), dir: Direction) -> Snake {
    let mut g = [[0; SIZE]; SIZE];
    g[pos.1][pos.0] = 1;
    let mut friut = genPos();
    while g[friut.1][friut.0] != 0 {
        friut = genPos();
    }
    Snake {
        len: 1,
        dir: dir,
        pos: pos,
        grid: g,
        alive: true,
        fruit: friut,
    }
}

pub fn printSnake(s: &Snake) {
    println!(
        "Direction: {:?}, Head location: ({}, {})",
        s.dir, s.pos.0, s.pos.1
    );
    println!(
        "Snake length: {}, Friut location: ({}, {})",
        s.len, s.fruit.0, s.fruit.1
    );
    for i in 0..SIZE {
        for j in 0..SIZE {
            if (j, i) == s.fruit {
                print!("{} ", Colour::Red.bold().paint("*"))
            } else {
                match s.grid[i][j] {
                    0 => print!("0 "),
                    n => print!("{} ", Colour::Green.bold().paint(format!("{}", n)))
                }
            }
        }
        println!();
    }
}

fn genPos() -> (usize, usize) {
    let mut rng = thread_rng();
    (rng.gen_range(0..SIZE), rng.gen_range(0..SIZE))
}
