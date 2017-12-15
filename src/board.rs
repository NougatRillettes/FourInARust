use std::fmt;
use std::char;
use std::ops::{Index};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum SqrState {
  Empty,
  Red,
  Yellow
}
impl SqrState {
    pub fn to_char(&self) -> char {
        use self::SqrState::*;
        match *self {
            Empty => ' ',
            Red => 'X',
            Yellow => 'O',
        }
    }
}
impl Default for SqrState {
    fn default() -> SqrState {SqrState::Empty}
}
impl fmt::Display for SqrState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result  {
            write!(f,"{}",self.to_char())
        }
}

#[derive(Debug)]
pub struct Board {
    cols : usize,
    lines : usize,
    // Vec of the columns (which are of size `lines` !)
    inner: Vec<Vec<SqrState>>,
    colslen : Vec<usize>
}

impl Board {
    /// Creates a new board
    pub fn new(cols: usize, lines: usize) -> Board {
        let mut inner = Vec::with_capacity(cols);
        for _ in 0..cols {
            inner.push(vec![SqrState::Empty; lines]);
        }
        Board{
            cols: cols,
            lines: lines,
            inner: inner,
            colslen: vec![0; cols]
        }
    }

    ///Checks if given Column is full. Panics if index is out of bound.
    pub fn col_is_full(&self, coli :usize) -> bool {
        if coli >= self.cols {panic!("Column index is out of bounds")}
        self.colslen[coli] >= self.lines
    }

    ///Add token to a column. Panics if:
    ///  - `coli` is out of bounds
    ///  - `color` is [Empty]
    ///  - the column is full
    pub fn add_to_col(&mut self, coli : usize, color: SqrState) {
        if color == SqrState::Empty {panic!("Trying to add an empty token")};
        if self.col_is_full(coli) {panic!("Trying to add to an already full column")}
        let li = self.colslen[coli];
        self.inner[coli][li] = color;
        self.colslen[coli] += 1;
    }

    //getters
    pub fn colslen(&self) -> &Vec<usize> {
        &self.colslen
    }
    pub fn cols(&self) -> usize {
        self.cols
    }
    pub fn lines(&self) -> usize {
        self.lines
    }
}

impl Index<usize> for Board {
    type Output = Vec<SqrState>;
    fn index(&self, index: usize) -> &Self::Output {&self.inner[index]}
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 2n+1 for edges ; n + 1 for \n or numbers
        let mut res = String::with_capacity((2*self.lines+1+1)*(2*self.cols+1+1));
        if self.cols <= 35 {

            res.push(' ');
            for i in 1..self.cols+1 {
                res.push(char::from_digit(i as u32,(self.cols+1) as u32).unwrap());
                res.push(' ');
            }
            res.push('\n');
        }
        for _ in 0..2*self.cols + 1{
            res.push('-');
        }
        res.push('\n');
        for li in (0..self.lines).rev() {
            res.push('|');
            for ci in 0..self.cols {
                res.push(self[ci][li].to_char());
                res.push('|');
            }
            res.push('\n');
            for _ in 0..2*self.cols + 1{
                res.push('-');
            }
            res.push('\n');
        }
        write!(f, "{}", res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generaltest() {
        //different cols and lines to be a bit less easy to pass
        let strboard =
        " 1 2 3 \n\
         -------\n\
         |X| | |\n\
         -------\n\
         |O|X| |\n\
         -------\n";
        let mut b = Board::new(3,2);
        assert!(!(b.col_is_full(0)));
        b.add_to_col(0,SqrState::Yellow);
        b.add_to_col(0,SqrState::Red);
        b.add_to_col(1,SqrState::Red);
        assert!(b.col_is_full(0));
        assert!(!(b.col_is_full(1)));
        assert!(!(b.col_is_full(2)));
        assert_eq!(*b.colslen(),vec![2,1,0]);
        assert!(format!("{}",b) == strboard)
    }

    #[test]
    #[should_panic]
    fn addtokenpanic1() {
        let mut b = Board::new(3,2);
        b.add_to_col(3,SqrState::Red);
    }

    #[test]
    #[should_panic]
    fn addtokenpanic2() {
        let mut b = Board::new(3,2);
        b.add_to_col(0,SqrState::Empty);
    }

    #[test]
    #[should_panic]
    fn addtokenpanic3() {
        let mut b = Board::new(3,2);
        b.add_to_col(0,SqrState::Red);
        b.add_to_col(0,SqrState::Red);
        b.add_to_col(0,SqrState::Red);
    }
}
