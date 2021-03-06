// Copyright (c) 2021 Malcolm Law
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use ansi_term::Colour;
use rand::{thread_rng, Rng};

const SIZE: usize = 5; // the map size

#[derive(Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn opposite(&self, a: &Direction) -> bool {
        match (self, a) {
            (Direction::Up, Direction::Down) => true,
            (Direction::Left, Direction::Right) => true,
            (Direction::Down, Direction::Up) => true,
            (Direction::Right, Direction::Left) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct Snake {
    len: u8,                  // snake length
    dir: Direction,           // head direction
    pos: (usize, usize),      // head position
    grid: [[u8; SIZE]; SIZE], // the map
    alive: bool,              // live or death
    fruit: (usize, usize),    // friut location
}

impl Snake {
    pub fn step(&mut self) -> &mut Self {
        match &self.dir {
            Direction::Up => self.pos.1 = (self.pos.1 + SIZE - 1) % SIZE,
            Direction::Down => self.pos.1 = (self.pos.1 + 1) % SIZE,
            Direction::Left => self.pos.0 = (self.pos.0 + SIZE - 1) % SIZE,
            Direction::Right => self.pos.0 = (self.pos.0 + 1) % SIZE,
        }
        if self.grid[self.pos.1][self.pos.0] > 0 {
            self.alive = false; // eat itself -> game over :P
        }
        self.updateGrid();
        self
    }
    pub fn turn(&mut self, dir: Direction) -> &mut Self {
        if !self.dir.opposite(&dir) {
            self.dir = dir; // head can only turn 90 degrees
        }
        self
    }
    pub fn isAlive(&self) -> bool {
        self.alive
    }
    pub fn score(&self) -> u8 {
        self.len
    }
    pub fn void(&self) {}
    fn updateGrid(&mut self) {
        if self.pos == self.fruit {
            self.len += 1;
            self.grid[self.pos.1][self.pos.0] = self.len;
            // if the sanke filled all the blocks, game over
            if self.len as usize == SIZE * SIZE {
                self.alive = false;
            } else {
                let mut fruit = genPos();
                while self.grid[fruit.1][fruit.0] != 0 {
                    fruit = genPos();
                }
                self.fruit = fruit; // generate new friut location
            }
        } else {
            for i in 0..SIZE {
                for j in 0..SIZE {
                    if self.grid[i][j] > 0 {
                        self.grid[i][j] -= 1; // snake move forward
                    }
                }
            }
            self.grid[self.pos.1][self.pos.0] = self.len;
        }
    }
}

pub fn newSnake(dir: Direction) -> Snake {
    let pos = genPos();
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
    println!("Direction: {:?}, Head location: {:?}", s.dir, s.pos);
    println!("Snake length: {}, Fruit location: {:?}", s.len, s.fruit);
    for i in 0..SIZE {
        for j in 0..SIZE {
            if (j, i) == s.fruit {
                print!("{}", Colour::Red.bold().paint(format!("{:^3}", "*")))
            } else {
                match s.grid[i][j] {
                    0 => print!("{:^3}", 0),
                    n => print!("{}", Colour::Green.bold().paint(format!("{:^3}", n))),
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
