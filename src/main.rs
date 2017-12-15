mod board;
use board::*;

fn main() {
    let mut b = Board::new(7,6);
    assert!(!(b.col_is_full(0)));
    b.add_to_col(3,SqrState::Yellow);
    b.add_to_col(3,SqrState::Red);
    b.add_to_col(4,SqrState::Red);
    print!("{}",b);
}
