mod snake;

use std::io;
use std::io::Write;

fn main() {
    let mut s = snake::newSnake((1, 1), snake::Direction::Up);
    snake::printSnake(&s);
    while s.isAlive() {
        print!("Input W/A/S/D or Nothing (q to quit): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("error: unable to read user input");
        print!("\x1B[2J\x1B[1;1H");
        match input.as_str() {
            "q\n" => break,
            "w\n" => {
                s.turn(snake::Direction::Up);
                ()
            }
            "s\n" => {
                s.turn(snake::Direction::Down);
                ()
            }
            "a\n" => {
                s.turn(snake::Direction::Left);
                ()
            }
            "d\n" => {
                s.turn(snake::Direction::Right);
                ()
            }
            _ => (),
        }
        s.step();
        snake::printSnake(&s);
    }
    println!("Game over!\nYour score: {}", s.score());
}
