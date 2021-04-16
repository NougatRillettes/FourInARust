use std::cmp::{max, min};

use ahash::AHashMap;

use crate::packedboard::*;

const MAX_DEPTH: u8 = 20;

const PLAYER_COLOR: NonEmptySqrState = NonEmptySqrState::Red;
const AI_COLOR: NonEmptySqrState = NonEmptySqrState::Yellow;

const COLS_ORDER: [ColIdx; NCOL as usize] = [
    ALL_COL_IDXS[3],
    ALL_COL_IDXS[2],
    ALL_COL_IDXS[4],
    ALL_COL_IDXS[1],
    ALL_COL_IDXS[5],
    ALL_COL_IDXS[0],
    ALL_COL_IDXS[6],
];

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
struct Score(i8);

impl Score {
    fn new(x: i8) -> Self {
        Self(x)
    }

    fn get(self) -> i8 {
        self.0
    }
}

impl Score {
    const MAX: Self = Self(i8::MAX);
    const MIN: Self = Self(i8::MIN + 1);
}

impl std::ops::Neg for Score {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.get())
    }
}

impl std::fmt::Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner = self.0;
        if inner == 0 {
            return f.write_fmt(format_args!("<unknown_issue>"));
        }
        f.write_fmt(format_args!(
            "{} win at turn n°{}",
            if inner > 0 { "I" } else { "you" },
            22 - inner.abs()
        ))
    }
}

fn negamax(
    b: &Board,
    tree_depth: u8,
    turn: u8,
    color: NonEmptySqrState,
    cache: &mut Cache,
    mut alpha: Score,
    mut beta: Score,
) -> (Option<ColIdx>, Score) {
    let remaining_tokens_p1 = Score::new(22 - (((turn + 1) / 2) as i8));
    if tree_depth == 0 || turn == 42 {
        return (None, Score::new(0))
    }
    if let Some(&cachentry) = cache.result.get(&b) {
        match cachentry {
            CacheEntry::Exact(col, score) => return (Some(col),score),
            CacheEntry::Unknown { turn: cache_turn } => {
                // dbg!(turn,tree_depth,cache_turn);
                if turn + tree_depth <= cache_turn {
                    return (None, Score::new(0));
                }
            }
        }
    }
    if let Some(&lb) = cache.lower_bounds.get(&b) {
                if lb >= beta {return (None, lb)};
                alpha = max(alpha, lb);
            }
        
    
    let (res_col,res_score) = {
        let mut recurse_positions = Vec::new();
        for &candidate_col in &COLS_ORDER {
            let mut b1 = (*b).clone();
            match b1.add_and_check(candidate_col, color) {
                Ok(true) => {
                    return (Some(candidate_col), remaining_tokens_p1);
                }
                Ok(false) => recurse_positions.push((candidate_col, b1)),
                Err(BoardError::ColumnFull { .. }) => continue,
                Err(e) => unreachable!("{}", e),
            }
        }
        let mut current_best_candidate = ALL_COL_IDXS[0];
        let mut current_best = Score::MIN;
        // println!("Starting: {} <= {}", alpha, beta);
        for (candidate_col, pos) in recurse_positions.into_iter() {
            // dbg!(alpha,beta);
            let (_, neg_score) = negamax(
                &pos,
                tree_depth - 1,
                turn + 1,
                color.other(),
                cache,
                -beta,
                -alpha,
            );
            let score = -neg_score;
            if score > current_best {
                current_best_candidate = candidate_col;
                current_best = score;
            }
            alpha = max(alpha, score);
            if alpha >= beta {
                // println!("Pruning: {} >= {}", alpha, beta);
                break;
            }
        }
        (Some(current_best_candidate), current_best)
    };
    if alpha < beta {
        if res_score.get() == 0 {
            cache.result.insert(b.clone(), CacheEntry::Unknown{turn});
        } else {
            cache.result.insert(b.clone(), CacheEntry::Exact(res_col.unwrap(), res_score));
        }
    }
    else if res_score.get() != 0 {
        cache.lower_bounds.entry(b.clone()).and_modify(|lb| *lb=min(*lb,res_score)).or_insert(res_score);
    }
    (res_col, res_score)
}
#[derive(Debug,Clone, Copy)]
enum CacheEntry {
    Exact(ColIdx,Score),
    Unknown{turn: u8},
}

struct Cache {
    result: ahash::AHashMap<Board, CacheEntry>,
    lower_bounds: ahash::AHashMap<Board, Score>,
}

impl Cache {
    fn new() -> Self {
        Self {
            result: AHashMap::with_capacity(100_000),
            lower_bounds: AHashMap::with_capacity(100_000),
        }
    }
}

pub struct AI(Cache);

impl AI {
    pub fn new() -> Self {
        AI(Cache::new())
    }

    pub fn make_a_move(&mut self, b: &Board) -> ColIdx {
        let cache = &mut self.0;
        // negamax(
        //     b,
        //     20,
        //     b.occupancy() + 1,
        //     AI_COLOR,
        //     cache,
        //     Score::MIN,
        //     Score::MAX,
        // );
        // negamax(
        //     b,
        //     30,
        //     b.occupancy() + 1,
        //     AI_COLOR,
        //     cache,
        //     Score::MIN,
        //     Score::MAX,
        // );
        let (res, reason) = negamax(
            b,
            MAX_DEPTH,
            b.occupancy() + 1,
            AI_COLOR,
            cache,
            Score::MIN,
            Score::MAX,
        );
        println!("Cache capacities: {}, {}", cache.result.capacity(), cache.lower_bounds.capacity());
        println!("Move chosen because: {}", reason,);
        res.unwrap()
    }
}

// pub fn make_a_move<const COLS: usize, const ROWS: usize>(b: &Board<COLS, ROWS>) -> usize {
//     let mut neutral_positions = vec![];
//     let mut loosing_positions = vec![];
//     'outer: for candidate_ai_col in 0..COLS {
//         let mut b1 = (*b).clone();
//         match b1.try_add_and_check(candidate_ai_col, AI_COLOR) {
//             Ok(true) => return candidate_ai_col,
//             Ok(false) => {}
//             Err(BoardError::ColumnFull { .. }) => continue 'outer,
//             Err(e) => unreachable!("{}", e),
//         }
//         for candidate_player_col in 0..COLS {
//             let mut b2 = b1.clone();
//             match b2.try_add_and_check(candidate_player_col, PLAYER_COLOR) {
//                 Ok(true) => {
//                     loosing_positions.push(candidate_ai_col);
//                     continue 'outer;
//                 }
//                 Ok(false) => {}
//                 Err(BoardError::ColumnFull { .. }) => {}
//                 Err(e) => unreachable!("{}", e),
//             }
//         }
//         neutral_positions.push(candidate_ai_col);
//     }
//     *neutral_positions
//         .first()
//         .or_else(|| loosing_positions.first())
//         .unwrap()
// }
