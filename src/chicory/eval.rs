use crate::chicory::board::{Board, PieceColor};
use crate::chicory::engine::{Engine, Move};
use crate::chicory::bitboard::board_serialize;

pub fn minmax(eng: &Engine, board: Board, depth: usize, mut alpha: f32, mut beta: f32, turn: PieceColor) -> (f32, Option<Move>) {
    //println!("info string {}", depth);
    if depth == 0 {
        return (eval(board), None) 
    }
    //println!("info string@ {}", depth);

    let mut best = match turn {
        PieceColor::White => -f32::INFINITY,
        PieceColor::Black =>  f32::INFINITY
    };

    let moves = eng.gen_moves(board);
    let mut best_move = moves[0];

        for m in moves {
            let (score, _) = minmax(&eng, m.2, depth-1, alpha, beta, !turn);
            match turn {
                PieceColor::White => {
                    if score > best {
                        best = score;
                        best_move = m;
                    }
                    if alpha > score {
                        alpha = score;
                    }

                },
                PieceColor::Black => {
                    if score < best {
                        best = score;
                        best_move = m;
                    }
                    if beta < score {
                        beta = score;
                    }
                }
            }
            
            if beta <= alpha {
                break;
            }
        }
    
    return (best, Some(best_move));
}

pub fn eval(board: Board) -> f32 {
    let mut score = 0.0;

    score += (board_serialize(board.pawns[PieceColor::White]).len() * 100) as f32;
    score -= (board_serialize(board.pawns[PieceColor::Black]).len() * 100) as f32;

    score += (board_serialize(board.knights[PieceColor::White]).len() * 300) as f32;
    score -= (board_serialize(board.knights[PieceColor::Black]).len() * 300) as f32;

    score += (board_serialize(board.bishops[PieceColor::White]).len() * 300) as f32;
    score -= (board_serialize(board.bishops[PieceColor::Black]).len() * 300) as f32;

    score += (board_serialize(board.rooks[PieceColor::White]).len() * 500) as f32;
    score -= (board_serialize(board.rooks[PieceColor::Black]).len() * 500) as f32;

    score += (board_serialize(board.queens[PieceColor::White]).len() * 900) as f32;
    score -= (board_serialize(board.queens[PieceColor::Black]).len() * 900) as f32;

    score += (board_serialize(board.kings[PieceColor::White]).len() * 9999999) as f32;
    score -= (board_serialize(board.kings[PieceColor::Black]).len() * 9999999) as f32;
    return score;
}
