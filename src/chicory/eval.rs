use crate::chicory::board::{Board, PieceColor};
use crate::chicory::engine::{Engine, Move};
use crate::chicory::bitboard::board_serialize;

pub fn minmax(eng: &Engine, board: Board, depth: usize, mut alpha: f32, mut beta: f32, turn: PieceColor, par_moves: usize) -> (f32, Option<Move>) {
    //println!("info string {}", depth);
    if depth == 0 {
        return (eval(&board, false), None)
    }
    //println!("info string@ {}", depth);

    let mut best = match turn {
        PieceColor::White => -f32::INFINITY,
        PieceColor::Black =>  f32::INFINITY
    };

    let moves = eng.gen_moves(board);

    if moves.len() == 0 {
        return (eval(&board, false), None)
    }

    let mut best_move = moves[0];

    let total_nodes = par_moves * moves.len();

    if depth > 2 {
        println!("info depth {} nodes {}", depth, total_nodes);
    }

        for m in moves {
            let (score, _) = minmax(&eng, m.2, depth-1, alpha, beta, !turn, total_nodes);

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

pub fn eval(board: &Board, real: bool) -> f32 {
    let mut score = 0.0;

    score += ((board_serialize(board.pawns[PieceColor::White]).len() as f32) - (board_serialize(board.pawns[PieceColor::Black]).len() as f32)) * 100.0;
    score += ((board_serialize(board.knights[PieceColor::White]).len() as f32) - (board_serialize(board.knights[PieceColor::Black]).len() as f32)) * 300.0;
    score += ((board_serialize(board.bishops[PieceColor::White]).len() as f32) - (board_serialize(board.bishops[PieceColor::Black]).len() as f32)) * 300.0;
    score += ((board_serialize(board.rooks[PieceColor::White]).len() as f32) - (board_serialize(board.rooks[PieceColor::Black]).len() as f32)) * 500.0;
    score += ((board_serialize(board.queens[PieceColor::White]).len() as f32) - (board_serialize(board.queens[PieceColor::Black]).len() as f32)) * 900.0;
    score += ((board_serialize(board.kings[PieceColor::White]).len() as f32) - (board_serialize(board.kings[PieceColor::Black]).len() as f32)) * 999999999.0;

    if real {
        println!("info string ||||||||||||||||||||||| eval {}", score);
    }


    return score;
}
