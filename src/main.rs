mod board;
use board::*;
use std::io::{self,Write};

fn main() {
    let mut b = Board::new(3,2);
    loop {
        print!("{}",b);
        print!("Where do you want to play ? ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .expect("Failed to read line");
        println!("{}",input);
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
    }
}
