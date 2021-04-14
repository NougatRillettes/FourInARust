use std::{cmp::Ordering, collections::HashMap};

use bv::BitVec;

use crate::board::*;

const MAX_DEPTH: u8 = 10;

const PLAYER_COLOR: NonEmptySqrState = NonEmptySqrState::Red;
const AI_COLOR: NonEmptySqrState = NonEmptySqrState::Yellow;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
struct Score(i8);

impl Score {
    const AI_WIN: Self = Score(i8::MIN);

    const PLAYER_WIN: Self = Score(i8::MAX);

    fn unknow(score: i8) -> Self {
        let res = Self(score);
        assert!(Self::AI_WIN < res);
        assert!(res < Self::PLAYER_WIN);
        res
    }
}

fn first_empty_strat<const COLS: usize, const ROWS: usize>(
    _b: &Board<COLS, ROWS>,
) -> (usize, i8, Score) {
    (0, 0, Score::unknow(0))
}

fn turn<const COLS: usize, const ROWS: usize>(
    b: &Board<COLS, ROWS>,
    depth: u8,
    ai_turn: bool,
    cache: &mut Cache<COLS, ROWS>,
) -> (usize, i8, Score) {
    let packed_repr = b.to_packed_repr();
    if let Some(&res) = cache.get(&packed_repr) {
        return res;
    }
    let res = if depth == MAX_DEPTH {
        first_empty_strat(b)
    } else {
        let color = if ai_turn { AI_COLOR } else { PLAYER_COLOR };
        let win_score = if ai_turn {
            Score::AI_WIN
        } else {
            Score::PLAYER_WIN
        };
        let mut recurse_positions = Vec::new();
        for candidate_col in 0..COLS {
            let mut b1 = (*b).clone();
            match b1.try_add_and_check(candidate_col, color) {
                Ok(true) => return (candidate_col, 0, win_score),
                Ok(false) => recurse_positions.push((candidate_col, b1)),
                Err(BoardError::ColumnFull { .. }) => continue,
                Err(e) => unreachable!("{}", e),
            }
        }
        let it = recurse_positions
            .into_iter()
            .map(|(col, sub_b)| (col, turn(&sub_b, depth + 1, !ai_turn, cache)));
        if ai_turn {
            it.min_by_key(|(_col, (_subcol, score_depth, score))| (*score, -*score_depth))
        } else {
            it.max_by_key(|(_col, (_subcol, score_depth, score))| (*score, *score_depth))
        }
        .map(|(col, (_subcol, score_depth, score))| (col, score_depth+1, score))
        .unwrap()
    };
    cache.insert(packed_repr, res);
    res
}

type Cache<const COLS: usize, const ROWS: usize> = ahash::AHashMap<u128, (usize, i8, Score)>;

pub fn make_a_move<const COLS: usize, const ROWS: usize>(b: &Board<COLS, ROWS>) -> usize {
    let mut cache = Cache::new();
    let (res, reason_depth, reason) = turn(b, 0, true, &mut cache);
    println!("Cache capacity: {}", cache.capacity());
    println!("Move chosen because: {:?} in {} moves", reason, reason_depth);
    res
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
