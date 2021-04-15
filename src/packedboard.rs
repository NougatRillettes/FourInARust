use std::fmt;
use std::{char, convert::TryInto};

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
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

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
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

pub const NCOL: u8 = 7;
pub const NROW: u8 = 6;
pub const ALL_COL_IDXS : [ColIdx; NCOL as usize] = [
    Idx(0),
    Idx(1),
    Idx(2),
    Idx(3),
    Idx(4),
    Idx(5),
    Idx(6),
];
pub const ALL_ROW_IDXS : [ColIdx; NROW as usize] = [
    Idx(0),
    Idx(1),
    Idx(2),
    Idx(3),
    Idx(4),
    Idx(5),
];
const GRID_SIZE: u8 = NCOL * NROW;
const LEN_SIZE: u8 = 3;

#[derive(Debug)]
pub enum BoardError {
    ColumnIndexOutOfBounds {
        required_index: usize,
    },
    RowIndexOutOfBounds {
        required_index: usize,
    },
    ColumnFull {
        column_index: ColIdx,
        tried_to_push: NonEmptySqrState,
    },
}

impl std::fmt::Display for BoardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use BoardError::*;
        match *self {
            ColumnIndexOutOfBounds { required_index } => f.write_fmt(format_args!(
                "Column index {} is ouf of bounds (there are {} columns)",
                required_index, NCOL
            )),
            RowIndexOutOfBounds { required_index } => f.write_fmt(format_args!(
                "Row index {} is ouf of bounds (there are {} rows)",
                required_index, NROW
            )),
            ColumnFull {
                column_index,
                tried_to_push,
            } => f.write_fmt(format_args!(
                "Column {} is already full, cannot add {} token.",
                column_index, tried_to_push
            )),
        }
    }
}

impl std::error::Error for BoardError {}

pub type BoardResult<T> = Result<T, BoardError>;

#[derive(Debug, Clone, Copy)]
pub struct Idx<const MAX: u8>(u8);
pub type ColIdx = Idx<NCOL>;
pub type RowIdx = Idx<NROW>;

impl<const MAX: u8> std::fmt::Display for Idx<MAX> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<const MAX: u8> Idx<MAX> {
    pub fn new(idx: u8) -> Option<Self> {
        if idx < MAX {
            Some(Self(idx))
        } else {
            None
        }
    }

    pub fn get(self) -> u8 {
        self.0
    }

    pub fn move_by(self, offset: i8) -> Option<Self> {
        let current = self.get() as i8;
        let new = current.checked_add(offset)?.try_into().ok()?;
        Self::new(new)
    }
}

fn get_bits<S, L>(x: u64, start_from_right: S, length: L) -> u64
where
    u64: std::ops::Shr<S, Output = u64>,
    u64: std::ops::Shr<usize, Output = u64>,
    L: Into<usize>,
{
    let length = length.into();
    let shifted: u64 = x >> start_from_right;
    let mask: u64 = std::ops::Shr::<usize>::shr(!0_u64, std::mem::size_of::<u64>() * 8 - length);
    shifted & mask
}

fn set_bits<S, L>(x: &mut u64, y: u64, start_from_right: S, length: L)
where
    u64: std::ops::Shl<S, Output = u64>,
    L: Into<usize>,
    S: Copy,
{
    let length = length.into();
    let shifted = y << start_from_right;
    let mask_shifted_right: u64 =
        std::ops::Shr::<usize>::shr(!0_u64, std::mem::size_of::<u64>() * 8 - length);
    let mask = mask_shifted_right << start_from_right;
    *x = (*x & !mask) | (shifted & mask);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Board(u64);

impl Board {
    /// Creates a new board
    pub const fn new() -> Self {
        Self(1 << 63)
    }

    pub fn check_col_idx(idx: u8) -> BoardResult<ColIdx> {
        ColIdx::new(idx).ok_or(BoardError::ColumnIndexOutOfBounds{
            required_index: idx as usize,
        })
    }

    pub fn check_row_idx(idx: u8) -> BoardResult<RowIdx> {
        RowIdx::new(idx).ok_or(BoardError::RowIndexOutOfBounds{
            required_index: idx as usize,
        })
    }

    pub fn col_len(&self, coli: ColIdx) -> u8 {
        let coli = coli.get();
        let offset = GRID_SIZE + coli * LEN_SIZE;
        get_bits(self.0, offset, LEN_SIZE) as u8
    }

    pub fn occupancy(&self) -> u8 {
        ALL_COL_IDXS.iter().map(|&c| self.col_len(c)).sum()
    }

    pub fn col_first_free_row(&self, coli: ColIdx) -> Option<RowIdx> {
        RowIdx::new(self.col_len(coli))
    }

    pub fn col_is_full(&self, coli: ColIdx) -> bool {
        self.col_first_free_row(coli).is_none()
    }

    fn get_cell(&self, coli: ColIdx, rowi: RowIdx) -> SqrState {
        let col_len = self.col_len(coli);
        let coli = coli.get();
        let rowi = rowi.get();
        if rowi >= col_len {
            SqrState::Empty
        } else {
            SqrState::NonEmpty(match get_bits(self.0, rowi + coli * NROW, 1_usize) {
                0 => NonEmptySqrState::Red,
                1 => NonEmptySqrState::Yellow,
                _ => unreachable!(),
            })
        }
    }

    pub fn add_to_col(
        &mut self,
        coli: ColIdx,
        color: NonEmptySqrState,
    ) -> Result<RowIdx, BoardError> {
        match self.col_first_free_row(coli) {
            None => Err(BoardError::ColumnFull {
                column_index: coli,
                tried_to_push: color,
            }),
            Some(rowi) => {
                let coli = coli.get();
                set_bits(
                    &mut self.0,
                    (rowi.get() + 1).into(),
                    GRID_SIZE + coli * LEN_SIZE,
                    LEN_SIZE,
                );
                if color == NonEmptySqrState::Yellow {
                    set_bits(&mut self.0, 1, rowi.get() + coli * NROW, 1_usize);
                }
                Ok(rowi)
            }
        }
    }

    // Check if the given position is part of a winning line
    pub fn win_at(&self, col_i: ColIdx, row_i: RowIdx) -> (SqrState, bool) {
        let state = self.get_cell(col_i, row_i);
        match state {
            SqrState::Empty => return (state, false),
            SqrState::NonEmpty(_) => {
                for (cdir, rdir) in &[(1, 0), (1, 1), (0, 1), (-1, 1)] {
                    let mut count = 0;
                    for dir in &[1, -1] {
                        for i in 1.. {
                            let c = col_i.move_by(dir * i * cdir);
                            let r = row_i.move_by(dir * i * rdir);
                            match c.zip(r).map(|(c, r)| self.get_cell(c, r)) {
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

    pub fn add_and_check(&mut self, coli: ColIdx, color: NonEmptySqrState) -> BoardResult<bool> {
        let rowi = self.add_to_col(coli, color)?;
        Ok(self.win_at(coli, rowi).1)
    }
}

//impl Index<usize> for Board {
//    type Output = Vec<SqrState>;
//    fn index(&self, index: usize) -> &Self::Output {&self.inner[index]}
//}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 2n+1 for edges ; n + 1 for \n or numbers
        let mut res =
            String::with_capacity((2 * (NROW as usize) + 1 + 1) * (2 * (NCOL as usize) + 1 + 1));
        // What can be represented by 1-9 + A-Z to index the cols
        if NCOL <= 35 {
            res.push(' ');
            for i in 1..NCOL + 1 {
                res.push(char::from_digit(i as u32, (NCOL + 1) as u32).unwrap());
                res.push(' ');
            }
            res.push('\n');
        }
        for _ in 0..(2 * NCOL + 1) {
            res.push('-');
        }
        res.push('\n');
        for ri in (0..NROW).rev() {
            res.push('|');
            for ci in 0..NCOL {
                let ci = ColIdx::new(ci).unwrap();
                let ri = RowIdx::new(ri).unwrap();
                res.push(self.get_cell(ci, ri).to_char());
                res.push('|');
            }
            res.push('\n');
            for _ in 0..2 * NCOL + 1 {
                res.push('-');
            }
            res.push('\n');
        }
        write!(f, "{}", res)
    }
}
