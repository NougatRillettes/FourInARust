mod ai;
mod board;
use board::*;
use io::BufRead;
use std::io::{self, Write};

fn main() -> anyhow::Result<()> {
    let mut b = Board::<7, 6>::new();
    let file = std::fs::File::open("input.txt")?;
    let mut bufreader = std::io::BufReader::new(file);
    loop {
        print!("{}", b);
        //    print!("{:?}",b);
        print!(
            "Where do you want to play (you play {}) ? ",
            NonEmptySqrState::Red
        );
        io::stdout().flush()?;
        let mut input = String::new();
        bufreader.read_line(&mut input)?;
        match input.trim().parse::<usize>() {
            Err(_) => {
                println!("Parse error :(");
                anyhow::bail!("Parse error");
                continue;
            }
            Ok(n) => {
                if b.try_add_and_check(n - 1, NonEmptySqrState::Red)? {
                    print!("{}", b);
                    println!("You won !!!!");
                    return Ok(());
                }
            }
        };
        let aimove = ai::make_a_move(&b);
        if b.try_add_and_check(aimove, NonEmptySqrState::Yellow)? {
            print!("{}", b);
            println!("You lost.");
            return Ok(());
        }
    }
}
