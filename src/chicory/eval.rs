use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;
use crate::chicory::board::{Board, PieceColor};
use crate::chicory::engine::{Engine, Move};
use crate::chicory::bitboard::board_serialize;

const BLACK_PAWN_PS_TABLE: [i32; 64] = [
    0,  0,  0,  0,  0,  0,  0,  0,
    50, 50, 50, 50, 50, 50, 50, 50,
    10, 10, 20, 30, 30, 20, 10, 10,
    5,  5, 10, 25, 25, 10,  5,  5,
    0,  0,  0, 20, 20,  0,  0,  0,
    5, -5,-10,  0,  0,-10, -5,  5,
    5, 10, 10,-20,-20, 10, 10,  5,
    0,  0,  0,  0,  0,  0,  0,  0
];

const BLACK_KNIGHT_PS_TABLE: [i32; 64] = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -30,  0, 10, 15, 15, 10,  0,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  0, 15, 20, 20, 15,  0,-30,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50
];

const BLACK_BISHOP_PS_TABLE: [i32; 64] = [
    -20,-10,-10,-10,-10,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5, 10, 10,  5,  0,-10,
    -10,  5,  5, 10, 10,  5,  5,-10,
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10, 10, 10, 10, 10, 10, 10,-10,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -20,-10,-10,-10,-10,-10,-10,-20,
];

const BLACK_ROOK_PS_TABLE: [i32; 64] = [
    0,  0,  0,  0,  0,  0,  0,  0,
    5, 10, 10, 10, 10, 10, 10,  5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    -5,  0,  0,  0,  0,  0,  0, -5,
    0,  0,  0,  5,  5,  0,  0,  0
];

const BLACK_QUEEN_PS_TABLE: [i32; 64] = [
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5,  5,  5,  5,  0,-10,
    -5,  0,  5,  5,  5,  5,  0, -5,
    0,  0,  5,  5,  5,  5,  0, -5,
    -10,  5,  5,  5,  5,  5,  0,-10,
    -10,  0,  5,  0,  0,  0,  0,-10,
    -20,-10,-10, -5, -5,-10,-10,-20
];

const BLACK_KING_MID_PS_TABLE: [i32; 64] = [
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -20,-30,-30,-40,-40,-30,-30,-20,
    -10,-20,-20,-20,-20,-20,-20,-10,
    20, 20,  0,  0,  0,  0, 20, 20,
    20, 30, 10,  0,  0, 10, 30, 20
];

const BLACK_KING_END_PS_TABLE: [i32; 64] = [
    -50,-40,-30,-20,-20,-30,-40,-50,
    -30,-20,-10,  0,  0,-10,-20,-30,
    -30,-10, 20, 30, 30, 20,-10,-30,
    -30,-10, 30, 40, 40, 30,-10,-30,
    -30,-10, 30, 40, 40, 30,-10,-30,
    -30,-10, 20, 30, 30, 20,-10,-30,
    -30,-30,  0,  0,  0,  0,-30,-30,
    -50,-30,-30,-30,-30,-30,-30,-50
];


const WHITE_PAWN_PS_TABLE: [i32; 64] = reverse_array(BLACK_PAWN_PS_TABLE);
const WHITE_KNIGHT_PS_TABLE: [i32; 64] = reverse_array(BLACK_KNIGHT_PS_TABLE);
const WHITE_BISHOP_PS_TABLE: [i32; 64] = reverse_array(BLACK_BISHOP_PS_TABLE);
const WHITE_ROOK_PS_TABLE: [i32; 64] = reverse_array(BLACK_ROOK_PS_TABLE);
const WHITE_QUEEN_PS_TABLE: [i32; 64] = reverse_array(BLACK_QUEEN_PS_TABLE);
const WHITE_KING_MID_PS_TABLE: [i32; 64] = reverse_array(BLACK_KING_MID_PS_TABLE);
const WHITE_KING_END_PS_TABLE: [i32; 64] = reverse_array(BLACK_KING_END_PS_TABLE);



pub fn minmax(eng: &Engine, board: Board, depth: usize, mut alpha: i32, mut beta: i32, turn: PieceColor, par_moves: usize, stop_calculation: &AtomicBool, top: bool) -> (i32, Option<Move>, usize) {
    //println!("info string {}", depth);
    if depth == 0 {
        return (eval(&board, false), None, 1)
    }

    let mut best = match turn {
        PieceColor::White => i32::MIN,
        PieceColor::Black =>  i32::MAX
    };

    let moves = eng.gen_moves(board);

    if moves.len() == 0 {
        return match !board.turn {
            PieceColor::White => (i32::MAX, None, 1),
            PieceColor::Black => (i32::MIN, None, 1)
        };
        //return (eval(&board, false), None)
    }

    let mut best_move = moves[0];

    let total_nodes = par_moves * moves.len();

    let mut node_count = 0;

        for m in moves {
            if stop_calculation.load(Ordering::Relaxed) {
                break;
            }

            let test_start = Instant::now();
            let (score, _, nodes) = minmax(&eng, m.2, depth-1, alpha, beta, !turn, total_nodes, stop_calculation, false);
            node_count += nodes;
            if top {
                println!("info depth {} nodes {} score cp {} time {} pv {}", depth, node_count, score, test_start.elapsed().as_millis(), Board::move_to_lan(&m));
            }

            match turn {
                PieceColor::White => {
                    if score > best {
                        best = score;
                        best_move = m;
                    }
                    if score > alpha {
                        alpha = score;
                    }

                },
                PieceColor::Black => {
                    if score < best {
                        best = score;
                        best_move = m;
                    }
                    if score < beta {
                        beta = score;
                    }
                }
            }
            
            if beta <= alpha {
                break;
            }
        }
    
    (best, Some(best_move), node_count)
}

pub fn eval(board: &Board, real: bool) -> i32 {
    let mut score = 0;

    let white_mat_score = ((board_serialize(board.pawns[PieceColor::White]).len() as i32) * 100) +
        ((board_serialize(board.knights[PieceColor::White]).len() as i32) * 320) +
        ((board_serialize(board.bishops[PieceColor::White]).len() as i32) * 330) +
        ((board_serialize(board.rooks[PieceColor::White]).len() as i32) * 500) +
        ((board_serialize(board.queens[PieceColor::White]).len() as i32) * 900);

    let black_mat_score = ((board_serialize(board.pawns[PieceColor::Black]).len() as i32) * 100) +
        ((board_serialize(board.knights[PieceColor::Black]).len() as i32) * 320) +
        ((board_serialize(board.bishops[PieceColor::Black]).len() as i32) * 330) +
        ((board_serialize(board.rooks[PieceColor::Black]).len() as i32) * 500) +
        ((board_serialize(board.queens[PieceColor::Black]).len() as i32) * 900);

    score += white_mat_score - black_mat_score;

    score += ((board_serialize(board.kings[PieceColor::White]).len() as i32) - (board_serialize(board.kings[PieceColor::Black]).len() as i32)) * 20000;

    score += bit_cal(board.pawns[PieceColor::White], WHITE_PAWN_PS_TABLE) - bit_cal(board.pawns[PieceColor::Black], BLACK_PAWN_PS_TABLE);
    score += bit_cal(board.knights[PieceColor::White], WHITE_KNIGHT_PS_TABLE) - bit_cal(board.knights[PieceColor::Black], BLACK_KNIGHT_PS_TABLE);
    score += bit_cal(board.bishops[PieceColor::White], WHITE_BISHOP_PS_TABLE) - bit_cal(board.bishops[PieceColor::Black], BLACK_BISHOP_PS_TABLE);
    score += bit_cal(board.rooks[PieceColor::White], WHITE_ROOK_PS_TABLE) - bit_cal(board.rooks[PieceColor::Black], BLACK_ROOK_PS_TABLE);
    score += bit_cal(board.queens[PieceColor::White], WHITE_QUEEN_PS_TABLE) - bit_cal(board.queens[PieceColor::Black], BLACK_QUEEN_PS_TABLE);

    if white_mat_score <= 1000 {
        score += bit_cal(board.kings[PieceColor::White], WHITE_KING_END_PS_TABLE);
    } else {
        let endgame_level = (white_mat_score-4000) / 3000; //(pms - game max) / (game max - 1000)
        score += ( (bit_cal(board.kings[PieceColor::White], WHITE_KING_MID_PS_TABLE) * (1-endgame_level)) + (bit_cal(board.kings[PieceColor::White], WHITE_KING_END_PS_TABLE) * endgame_level) ) / 2
    }

    if black_mat_score <= 1000 {
        score += bit_cal(board.kings[PieceColor::Black], BLACK_KING_END_PS_TABLE);
    } else {
        let endgame_level = (white_mat_score-4000) / 3000; //(pms - game max) / (game max - 1000)
        score += ( (bit_cal(board.kings[PieceColor::Black], BLACK_KING_MID_PS_TABLE) * (1-endgame_level)) + (bit_cal(board.kings[PieceColor::Black], BLACK_KING_END_PS_TABLE) * endgame_level) ) / 2
    }

    if real {
        println!("info string ||||||||||||||||||||||| eval {}", score);
    }


    score
}

fn bit_cal(mut bitboard: u64, table: [i32; 64]) -> i32 {
    let mut score = 0;
    while bitboard != 0 {
        let i = bitboard.trailing_zeros() as usize;
        score += table[i];
        bitboard ^= 1 << i;
    }

    score
}

const fn reverse_array<T: Copy, const N: usize>(array: [T; N]) -> [T; N] {
    let mut out = array;
    let mut i = 0;
    while i < N {
        out[i] = array[N - 1 - i];
        i += 1;
    }

    out
}