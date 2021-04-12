use array_init::array_init;
use std::char;
use std::fmt;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum NonEmptySqrState {
    Red,
    Yellow,
}

impl NonEmptySqrState {
    pub fn to_char(self) -> char {
        match self {
            Self::Red => 'X',
            Self::Yellow => 'O',
        }
    }
}

impl std::fmt::Display for NonEmptySqrState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}", self.to_char()))
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum SqrState {
    Empty,
    NonEmpty(NonEmptySqrState),
}

impl SqrState {
    pub fn to_char(self) -> char {
        use self::SqrState::*;
        match self {
            Empty => ' ',
            NonEmpty(ness) => ness.to_char(),
        }
    }

    pub fn is_empty(self) -> bool {
        matches!(self, Self::Empty)
    }
}

impl Default for SqrState {
    fn default() -> SqrState {
        SqrState::Empty
    }
}

impl fmt::Display for SqrState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

pub struct Column<const SIZE: usize> {
    inner: [SqrState; SIZE],
    len: usize,
}

impl<const SIZE: usize> Column<SIZE> {
    pub fn new() -> Self {
        Self {
            inner: [SqrState::Empty; SIZE],
            len: 0,
        }
    }

    pub fn is_full(&self) -> bool {
        self.len == SIZE
    }

    pub fn try_push(&mut self, x: NonEmptySqrState) -> Result<usize, NonEmptySqrState> {
        let ptr = self.inner.get_mut(self.len).ok_or(x)?;
        *ptr = SqrState::NonEmpty(x);
        let prev_len = self.len;
        self.len += 1;
        Ok(prev_len)
    }

    pub fn get(&self, i: usize) -> Option<SqrState> {
        self.inner.get(i).map(|&x| x)
    }

    pub fn get_mut(&mut self, i: usize) -> Option<&mut SqrState> {
        self.inner.get_mut(i)
    }
}

pub struct Board<const COLS: usize, const ROWS: usize> {
    states: [Column<ROWS>; COLS],
}

impl<const COLS: usize, const ROWS: usize> Board<COLS, ROWS> {
    /// Creates a new board
    pub fn new() -> Self {
        Self {
            states: array_init(|_| Column::new()),
        }
    }

    ///Checks if given Column is full. Panics if index is out of bound.
    pub fn col_is_full(&self, coli: usize) -> bool {
        let col = self.get_col(coli).expect("Column index is out of bounds");
        col.is_full()
    }

    fn get_col(&self, coli: usize) -> Option<&Column<ROWS>> {
        self.states.get(coli)
    }

    fn get_col_mut(&mut self, coli: usize) -> Option<&mut Column<ROWS>> {
        self.states.get_mut(coli)
    }

    fn get_cell(&self, coli: usize, rowi: usize) -> Option<SqrState> {
        let col = self.get_col(coli)?;
        col.get(rowi)
    }

    fn get_cell_mut(&mut self, coli: usize, rowi: usize) -> Option<&mut SqrState> {
        let col = self.get_col_mut(coli)?;
        col.get_mut(rowi)
    }

    ///Add token to a column. Panics if:
    ///  - `coli` is out of bounds
    ///  - the column is full
    /// Returns: height of added token
    pub fn add_to_col(&mut self, coli: usize, color: NonEmptySqrState) -> usize {
        self.get_col_mut(coli)
            .expect("Column index is out of bounds")
            .try_push(color)
            .expect("Trying to add to an already full column")
    }

    // Check if the given position is part of a winning line
    pub fn win_at(&self, col_i: usize, row_i: usize) -> (SqrState, bool) {
        let state = self.get_cell(col_i, row_i).unwrap();
        match state {
            SqrState::Empty => return (state, false),
            SqrState::NonEmpty(_) => {
                for (cdir, rdir) in &[(1, 0), (1, 1), (0, 1), (-1, -1)] {
                    let mut count = 0;
                    for dir in &[1, -1] {
                        for i in 1.. {
                            let c: isize = (col_i as isize) + dir * i * cdir;
                            let r: isize = (row_i as isize) + dir * i * rdir;
                            match self.get_cell(c as usize, r as usize) {
                                Some(other_state) if other_state == state => count += 1,
                                _ => break,
                            }
                        }
                    }
                    if count >= 3 {
                        return (state, true);
                    }
                }
                return (state, false);
            }
        }
    }

    pub fn add_and_check(&mut self, coli: usize, color: NonEmptySqrState) -> bool {
        let rowi = self.add_to_col(coli, color);
        self.win_at(coli, rowi).1
    }

    pub fn columns(&self) -> &[Column<ROWS>; COLS] {
        &self.states
    }
}

//impl Index<usize> for Board {
//    type Output = Vec<SqrState>;
//    fn index(&self, index: usize) -> &Self::Output {&self.inner[index]}
//}

impl<const COLS: usize, const ROWS: usize> fmt::Display for Board<COLS, ROWS> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 2n+1 for edges ; n + 1 for \n or numbers
        let mut res = String::with_capacity((2 * ROWS + 1 + 1) * (2 * COLS + 1 + 1));
        // What can be represented by 1-9 + A-Z to index the cols
        if COLS <= 35 {
            res.push(' ');
            for i in 1..COLS + 1 {
                res.push(char::from_digit(i as u32, (COLS + 1) as u32).unwrap());
                res.push(' ');
            }
            res.push('\n');
        }
        for _ in 0..(2 * COLS + 1) {
            res.push('-');
        }
        res.push('\n');
        for ri in (0..ROWS).rev() {
            res.push('|');
            for ci in 0..COLS {
                res.push(self.get_cell(ci, ri).unwrap().to_char());
                res.push('|');
            }
            res.push('\n');
            for _ in 0..2 * COLS + 1 {
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
        let strboard = " 1 2 3 \n\
         -------\n\
         |X| | |\n\
         -------\n\
         |O|X| |\n\
         -------\n";
        let mut b = Board::<3, 2>::new();
        assert!(!(b.col_is_full(0)));
        b.add_to_col(0, NonEmptySqrState::Yellow);
        b.add_to_col(0, NonEmptySqrState::Red);
        b.add_to_col(1, NonEmptySqrState::Red);
        assert!(b.col_is_full(0));
        assert!(!(b.col_is_full(1)));
        assert!(!(b.col_is_full(2)));
        let colslen: Vec<_> = b.states.iter().map(|x| x.len).collect();
        assert_eq!(colslen, vec![2, 1, 0]);
        assert!(format!("{}", b) == strboard)
    }

    #[test]
    #[should_panic]
    fn addtokenpanic1() {
        let mut b = Board::<3, 2>::new();
        b.add_to_col(3, NonEmptySqrState::Red);
    }

    #[test]
    #[should_panic]
    fn addtokenpanic3() {
        let mut b = Board::<3, 2>::new();
        b.add_to_col(0, NonEmptySqrState::Red);
        b.add_to_col(0, NonEmptySqrState::Red);
        b.add_to_col(0, NonEmptySqrState::Red);
    }
}
