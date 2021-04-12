use crate::board::*;

pub fn make_a_move<const COLS: usize, const ROWS: usize>(b : &Board<COLS, ROWS>) -> usize {
    b.columns()
            .iter()
            .enumerate()
            .find(|&(_,col)| !col.is_full())
            .map(|(i,_)| i)
            .unwrap()
}
