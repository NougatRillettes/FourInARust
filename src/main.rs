mod board;
mod ai;
use board::*;
use std::io::{self,Write};

fn main() {
    let mut b = Board::new(7,8);
    loop {
        print!("{}",b);
    //    print!("{:?}",b);
        print!("Where do you want to play (you play {}) ? ",SqrState::Red);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        match input.trim().parse::<usize>() {
            Err(_) => {println!("Parse error :("); continue;},
            Ok(n) => {
                if 1 <= n && n <= b.cols() {
                    if b.col_is_full(n-1) {
                        println!("This column is full, isn't it ?");
                        continue;
                    }
                    b.add_to_col(n-1,SqrState::Red)
                }
                else {
                    println!("Can't play that far mate.");
                    continue;
                }
            },
        };
        let aimove = ai::make_a_move(&b);
        b.add_to_col(aimove,SqrState::Yellow);
    }
}
