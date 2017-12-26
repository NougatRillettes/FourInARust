use board::*;

pub fn make_a_move(b : &Board) -> usize {
    let lmax = b.lines();
    let mut doables = b.colslen()
            .iter()
            .enumerate()
            .filter(|&(_,l)| *l < lmax)
            .map(|(i,_)| i);
    doables.next().unwrap()
}
