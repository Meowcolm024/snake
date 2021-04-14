// Copyright (c) 2021 Malcolm Law
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

mod snake;

use std::io;
use std::io::Write;

fn main() {
    let mut s = snake::newSnake((1, 1), snake::Direction::Up);
    snake::printSnake(&s);
    while s.isAlive() {
        print!("Input W/A/S/D (q to quit): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");
        print!("\x1B[2J\x1B[1;1H");
        match input.trim() {
            "q" => break,
            "w" => s.turn(snake::Direction::Up).void(),
            "s" => s.turn(snake::Direction::Down).void(),
            "a" => s.turn(snake::Direction::Left).void(),
            "d" => s.turn(snake::Direction::Right).void(),
            _ => (),
        }
        s.step();
        snake::printSnake(&s);
    }
    println!("Game over!\nYour score: {}", s.score());
}
