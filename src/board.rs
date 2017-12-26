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

#[derive(Copy,Clone,Debug,Default)]
pub struct SqrCount {
    n : u32, //allegedly the fastest of uX types
    nw : u32,
    w: u32,
    sw : u32,
    s: u32,
    se: u32,
    e: u32,
    ne : u32
}

impl SqrCount {
    // return mut ref to the corresponding part of the struct, panics if not valid offsets
    pub fn fromoffset_mut(&mut self, oc: isize, ol : isize) -> &mut u32 {
        match (oc,ol) {
            (0, 1) => &mut self.n,
            (-1,1) => &mut self.nw,
            (-1,0) => &mut self.w,
            (-1,-1) => &mut self.sw,
            (0,-1) => &mut self.s,
            (1,-1) => &mut self.se,
            (1,0) => &mut self.e,
            (1,1) => &mut self.ne,
            _ => panic!("At least one of the offsets was invalid")
        }
    }
    pub fn fromoffset(&self, oc: isize, ol : isize) -> u32 {
        match (oc,ol) {
            (0, 1) => self.n,
            (-1,1) => self.nw,
            (-1,0) => self.w,
            (-1,-1) => self.sw,
            (0,-1) => self.s,
            (1,-1) => self.se,
            (1,0) => self.e,
            (1,1) => self.ne,
            _ => panic!("At least one of the offsets was invalid")
        }
    }

}

pub struct Board {
    cols : usize,
    lines : usize,
    // Vec of the columns (which are of size `lines` !)
    states: Vec<Vec<SqrState>>,
    countred: Vec<Vec<SqrCount>>,
    countyellow: Vec<Vec<SqrCount>>,
    colslen : Vec<usize>
}

impl Board {
    /// Creates a new board
    pub fn new(cols: usize, lines: usize) -> Board {
        let mut states = Vec::with_capacity(cols);
        let mut counts = Vec::with_capacity(cols);
        for _ in 0..cols {
            states.push(vec![SqrState::Empty; lines]);
            counts.push(vec![Default::default();lines])
        }
        Board{
            cols: cols,
            lines: lines,
            states: states,
            countred : counts.clone(),
            countyellow : counts,
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
        self.states[coli][li] = color;
        self.colslen[coli] += 1;
        for &(cdir,ldir) in &[(1,1),(0,1),(-1,1),(-1,0),(-1,-1),(0,-1),(1,-1),(1,0)] {
            let mut coff : isize = 0;
            let mut loff : isize = 0;
            loop {
                coff += cdir;
                loff += ldir;
                let ln = (li as isize) + loff;
                let coln = (coli as isize) + coff;
                //print!("In dir c:{} l:{}, offset c:{}, l:{}, at c:{}, l:{}", cdir, ldir, coff, loff, coln, ln);
                if 0 <= ln && 0 <= coln && (ln as usize) < self.lines && (coln as usize) < self.cols {
                    if color == SqrState::Red {
                        *self.countred[coln as usize][ln as usize]
                        .fromoffset_mut(-cdir, -ldir) +=
                        1 + self.countred[coli][li].fromoffset(-cdir,-ldir);
                    } else {
                        *self.countyellow[coln as usize][ln as usize]
                        .fromoffset_mut(-cdir, -ldir) +=
                        1 + self.countyellow[coli][li].fromoffset(-cdir,-ldir);;
                    }
                    if self.states[coln as usize][ln as usize] != color {
                    //    println!(" end of dir", );
                        break;
                    }
                //    println!("", );
                } else {
                //    println!(" out of bounds", );
                    break;
                }
            }
        }
    }

    // Check if the given position is part of a winning line
    pub fn win_at(& self, ci: usize, li:usize) -> bool {
        use SqrState::*;
        let counter = match self.states[ci][li] {
            Empty => return false,
            Red => &self.countred[ci][li],
            Yellow => &self.countyellow[ci][li]
        };
        // only half of the directions because they are summed with their opposite
        [(1,1),(0,1),(-1,1),(-1,0)]
        .iter()
        .map(|&(cdir,ldir)| {counter.fromoffset(cdir,ldir) + counter.fromoffset(-cdir,-ldir)})
        .any(|x| x >= 3)
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

//impl Index<usize> for Board {
//    type Output = Vec<SqrState>;
//    fn index(&self, index: usize) -> &Self::Output {&self.inner[index]}
//}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 2n+1 for edges ; n + 1 for \n or numbers
        let mut res = String::with_capacity((2*self.lines+1+1)*(2*self.cols+1+1));
        // What can be represented by 1-9 + A-Z to index the cols
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
                res.push(self.states[ci][li].to_char());
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

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        [&self.countred,&self.countyellow]
        .iter()
        .map(|countingvec| {
            let mut res = String::new();
            res.push(' ');
            for i in 0..self.cols {
                    res.push_str(&format!("{:3}",self.colslen[i]));
                    res.push(' ');
            }
            res.push('\n');
            for _ in 0..4*self.cols + 1{
                res.push('-');
            }
            res.push('\n');
            for li in (0..self.lines).rev() {
                res.push('|');
                for ci in 0..self.cols {
                    let count : &SqrCount = &countingvec[ci][li];
                    res.push_str(&format!("{}{}{}",count.nw,count.n,count.ne));
                    res.push('|');
                }
                res.push('\n');
                res.push('|');
                for ci in 0..self.cols {
                    let count : &SqrCount = &countingvec[ci][li];
                    res.push_str(&format!("{}{}{}",count.w,self.states[ci][li],count.e));
                    res.push('|');
                }
                res.push('\n');
                res.push('|');
                for ci in 0..self.cols {
                    let count : &SqrCount = &countingvec[ci][li];
                    res.push_str(&format!("{}{}{}",count.sw,count.s,count.se));
                    res.push('|');
                }
                res.push('\n');
                for _ in 0..4*self.cols + 1{
                    res.push('-');
                }
                res.push('\n');
            }
            write!(f, "{}", res)
        })
        .fold(Ok(()),|x,y| x.and(y))
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
