mod ai;
mod board;
mod packedboard;
use packedboard::*;
use io::BufRead;
use std::io::{self, Write};

fn main() -> anyhow::Result<()> {
    let mut b = Board::new();
    let file = std::fs::File::open("input.txt")?;
    let mut bufreader = std::io::BufReader::new(file);
    // let bufreader = std::io::stdin();
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
        match input.trim().parse::<u8>() {
            Err(_) => {
                println!("Parse error :(");
                anyhow::bail!("Parse error");
                continue;
            }
            Ok(n) => {
                let colidx = Board::check_col_idx(n-1)?;
                if b.add_and_check(colidx, NonEmptySqrState::Red)? {
                    print!("{}", b);
                    println!("You won !!!!");
                    return Ok(());
                }
            }
        };
        let aimove = ai::make_a_move(&b);
        if b.add_and_check(aimove, NonEmptySqrState::Yellow)? {
            print!("{}", b);
            println!("You lost.");
            return Ok(());
        }
    }
}
