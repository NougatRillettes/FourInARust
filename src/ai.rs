use crate::packedboard::*;

const MAX_DEPTH: u8 = 25;

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
}

impl Score {
    const MAX: Self = Self(i8::MAX);
    const MIN: Self = Self(i8::MIN);
}

impl std::fmt::Display for Score {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let inner = self.0;
        if inner == 0 {
            return f.write_fmt(format_args!("<unknown_issue>"));
        }
        f.write_fmt(format_args!(
            "{} wins at turn n°{}",
            if inner > 0 { "player" } else { "ai" },
            22 - inner.abs()
        ))
    }
}

fn first_empty_strat(_b: &Board) -> (ColIdx, Score) {
    (ColIdx::new(0).unwrap(), Score::new(0))
}

fn turn(
    b: &Board,
    depth: u8,
    ai_turn: bool,
    cache: &mut Cache,
    mut alpha: Score,
    mut beta: Score,
) -> (ColIdx, Score) {
    if let Some(&res) = cache.get(&b) {
        return res;
    }
    let res = {
        if depth == MAX_DEPTH {
            first_empty_strat(b)
        } else {
            let color = if ai_turn { AI_COLOR } else { PLAYER_COLOR };
            let win_score_multiply = if ai_turn { -1 } else { 1 };
            let mut recurse_positions = Vec::new();
            for &candidate_col in &COLS_ORDER {
                let mut b1 = (*b).clone();
                match b1.add_and_check(candidate_col, color) {
                    Ok(true) => {
                        let remaining_tokens_p1 = (22 - (b1.occupancy() + 1) / 2) as i8;
                        return (
                            candidate_col,
                            Score::new(remaining_tokens_p1 * win_score_multiply),
                        );
                    }
                    Ok(false) => recurse_positions.push((candidate_col, b1)),
                    Err(BoardError::ColumnFull { .. }) => continue,
                    Err(e) => unreachable!("{}", e),
                }
            }
            let mut current_best_candidate = ALL_COL_IDXS[0];
            let mut current_best = if ai_turn { Score::MAX } else { Score::MIN };
            // println!("Starting: {} <= {}", alpha, beta);
            for (candidate_col, pos) in recurse_positions.into_iter() {
                let (_, score) = turn(&pos, depth + 1, !ai_turn, cache, alpha, beta);
                if ai_turn && (score < current_best) {
                    current_best_candidate = candidate_col;
                    current_best = score;
                    beta = current_best;
                } else if !ai_turn && (score > current_best) {
                    current_best_candidate = candidate_col;
                    current_best = score;
                    alpha = current_best;
                }
                if alpha >= beta {
                    // println!("Pruning: {} >= {}", alpha, beta);
                    break;
                }
            }
            (current_best_candidate, current_best)
        }
    };
    cache.insert(b.clone(), res);
    res
}

type Cache = ahash::AHashMap<Board, (ColIdx, Score)>;

pub fn make_a_move(b: &Board) -> ColIdx {
    let mut cache = Cache::new();
    let (res, reason) = turn(b, 0, true, &mut cache, Score::MIN, Score::MAX);
    println!("Cache capacity: {}", cache.capacity());
    println!("Move chosen because: {}", reason,);
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
