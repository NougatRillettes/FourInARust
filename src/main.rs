mod ai;
mod board;
use board::*;
use std::io::{self, Write};

fn main() {
    let mut b = Board::<7, 8>::new();
    loop {
        print!("{}", b);
        //    print!("{:?}",b);
        print!(
            "Where do you want to play (you play {}) ? ",
            NonEmptySqrState::Red
        );
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        match input.trim().parse::<usize>() {
            Err(_) => {
                println!("Parse error :(");
                continue;
            }
            Ok(n) => {
                if b.add_and_check(n-1, NonEmptySqrState::Red) {
                    print!("{}", b);
                    println!("You won !!!!");
                    return;
                }
            }
        };
        let aimove = ai::make_a_move(&b);
        if b.add_and_check(aimove, NonEmptySqrState::Yellow) {
            print!("{}", b);
            println!("You lost.");
            return;
        }
    }
}
