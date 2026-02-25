use std::time::Instant;
use crate::chicory::board::{Board, PieceColor};
use crate::chicory::engine::{Engine, Move};
use crate::chicory::bitboard::board_serialize;

pub fn minmax(eng: &Engine, board: Board, depth: usize, mut alpha: f32, mut beta: f32, turn: PieceColor, par_moves: usize, top: bool) -> (f32, Option<Move>, usize) {
    //println!("info string {}", depth);
    if depth == 0 {
        return (eval(&board, false), None, 1)
    }

    let mut best = match turn {
        PieceColor::White => -f32::INFINITY,
        PieceColor::Black =>  f32::INFINITY
    };

    let moves = eng.gen_moves(board);

    if moves.len() == 0 {
        return match !board.turn {
            PieceColor::White => (999999999.0, None, 1),
            PieceColor::Black => (-999999999.0, None, 1)
        };
        //return (eval(&board, false), None)
    }

    let mut best_move = moves[0];

    let total_nodes = par_moves * moves.len();

    let mut node_count = 0;

        for m in moves {
            let test_start = Instant::now();
            let (score, _, nodes) = minmax(&eng, m.2, depth-1, alpha, beta, !turn, total_nodes, false);
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
    
    return (best, Some(best_move), node_count);
}

pub fn eval(board: &Board, real: bool) -> f32 {
    let mut score = 0.0;

    score += ((board_serialize(board.pawns[PieceColor::White]).len() as f32) - (board_serialize(board.pawns[PieceColor::Black]).len() as f32)) * 100.0;
    score += ((board_serialize(board.knights[PieceColor::White]).len() as f32) - (board_serialize(board.knights[PieceColor::Black]).len() as f32)) * 300.0;
    score += ((board_serialize(board.bishops[PieceColor::White]).len() as f32) - (board_serialize(board.bishops[PieceColor::Black]).len() as f32)) * 300.0;
    score += ((board_serialize(board.rooks[PieceColor::White]).len() as f32) - (board_serialize(board.rooks[PieceColor::Black]).len() as f32)) * 500.0;
    score += ((board_serialize(board.queens[PieceColor::White]).len() as f32) - (board_serialize(board.queens[PieceColor::Black]).len() as f32)) * 900.0;

    /*
    score += ((board_serialize(board.kings[PieceColor::White]).len() as f32) - (board_serialize(board.kings[PieceColor::Black]).len() as f32)) * 999999999.0;
    */

    if real {
        println!("info string ||||||||||||||||||||||| eval {}", score);
    }


    return score;
}
