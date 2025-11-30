use chessbot::chessbot::{board, engine};
use board::{Board, PieceType, PieceColor};
use engine::{Move};

pub fn fen_arr(from: usize, new_boards: Vec<(usize, &str)> ) -> Vec<Move> {
    let mut boards: Vec<Move> = vec![];
    for (to, fen) in new_boards.iter() {
        boards.push( (from, *to, Board::new(fen)) );
    }

    return boards;
}

pub fn one_piece_move(init_board: &Board, from: usize, new_squares: Vec<usize>) -> Vec<Move> {
    let mut boards: Vec<Move> = vec![];

    let mut start_board = init_board.clone();
    let (pc, pt) = start_board.lookup(from);
    
    //Remove from spot
    match pt {
            PieceType::Pawn   => start_board.pawns[pc]   = start_board.pawns[pc]   & !(1 << from),
            PieceType::Bishop => start_board.bishops[pc] = start_board.bishops[pc] & !(1 << from),
            PieceType::Knight => start_board.knights[pc] = start_board.knights[pc] & !(1 << from),
            PieceType::Rook   => start_board.rooks[pc]   = start_board.rooks[pc]    & !(1 << from),
            PieceType::Queen  => start_board.queens[pc]  = start_board.queens[pc]   & !(1 << from),
            PieceType::King   => start_board.kings[pc]   = start_board.kings[pc]    & !(1 << from),
            PieceType::Empty  => (),
        };

    //Update half **Will not calc for captures
    start_board.half_moves += 1;
    //Update full moves and color
    if pc == PieceColor::Black {
        start_board.full_move += 1;
        start_board.turn = PieceColor::White;
    } else {
        start_board.turn = PieceColor::Black;
    }

    for to in new_squares {
        let mut board = start_board.clone();
        match pt {
            PieceType::Pawn   => board.pawns[pc] = board.pawns[pc]   | (1 << to),
            PieceType::Bishop => board.bishops[pc]  = board.bishops[pc] | (1 << to),
            PieceType::Knight => board.knights[pc]  = board.knights[pc] | (1 << to),
            PieceType::Rook   => board.rooks[pc]    = board.rooks[pc]   | (1 << to),
            PieceType::Queen  => board.queens[pc]   = board.queens[pc]  | (1 << to),
            PieceType::King   => board.kings[pc]    = board.kings[pc]   | (1 << to),
            PieceType::Empty  => (),
        };

        board.print_board();
        boards.push( (from, to, board ) );
    }

    return boards;
}
