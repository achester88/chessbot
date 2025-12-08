use super::bitboard::{print_bitboard, print_bitboard_pos};
use core::ops::{Index, IndexMut};
use crate::chessbot::engine::Move;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Debug, PartialEq)]
pub enum PieceType {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
    Empty,
}

enum OP {
    AND,
    OR,
}

impl Index<PieceColor> for [u64] {
    type Output = u64;

    fn index(&self, color: PieceColor) -> &Self::Output {
        match color {
            PieceColor::White => &self[0],
            PieceColor::Black => &self[1],
        }
    }
}

impl IndexMut<PieceColor> for [u64; 2] {
    fn index_mut(&mut self, color: PieceColor) -> &mut Self::Output {
        match color {
            PieceColor::White => &mut self[0],
            PieceColor::Black => &mut self[1],
        }
    }
}

#[derive(Clone, PartialEq, Debug, Copy)]
pub struct Board {
    pub pawns: [u64; 2],
    pub bishops: [u64; 2],
    pub knights: [u64; 2],
    pub rooks: [u64; 2],
    pub queens: [u64; 2],
    pub kings: [u64; 2],

    pub turn: PieceColor,
    pub casling: u8,    //white, black | queenside, kingside QKqk
    pub en_passant: u8, //postion of avilbe en passant
    pub check: u64,
    pub half_moves: u16,
    pub full_move: u64,

    pub occupied: u64,
    pub pieces: [u64; 2], //All piece of the same color
}

impl Board {
    pub fn new(fen_str: &str) -> Self {
        //println!("{}", fen_str);
        let mut wp = 0;
        let mut wb = 0;
        let mut wn = 0;
        let mut wr = 0;
        let mut wq = 0;
        let mut wk = 0;
        let mut bp = 0;
        let mut bb = 0;
        let mut bn = 0;
        let mut br = 0;
        let mut bq = 0;
        let mut bk = 0;
        let mut wt = true;
        let mut casling: u8 = 0;
        let mut ep: u8 = 65;
        let mut hm = 0;
        let mut fm = 1;
        let mut i = 64;
        let mut f = 0;
        let mut r = 7;
        let fen: Vec<&str> = fen_str.split(" ").collect();

        //postion
        let pos = fen[0]; //fen[0].replace("/", "");
        let pos_vec: Vec<char> = pos.chars().collect();
        for c in pos_vec {
            if c == '/' {
                r -= 1;
                f = 0;
            } else if c.is_numeric() {
                //println!("{}", c);
                //i -= c.to_digit(10).unwrap() as usize;
                f += c.to_digit(10).unwrap() as usize;
            } else {
                //println!("{}", c);
                let s = 1 << ((r * 8) + f); //set bit of i;
                                            //(1 << n)
                                            //print_bitboard(s);
                match c {
                    'P' => wp = wp | s,
                    'B' => wb = wb | s,
                    'N' => wn = wn | s,
                    'R' => wr = wr | s,
                    'Q' => wq = wq | s,
                    'K' => wk = wk | s,

                    'p' => bp = bp | s,
                    'b' => bb = bb | s,
                    'n' => bn = bn | s,
                    'r' => br = br | s,
                    'q' => bq = bq | s,
                    'k' => bk = bk | s,
                    _ => {}
                };

                //i -= 1;
                f += 1;
            }
        }
        //turn
        wt = fen[1] == "w";

        //casling
        if fen[2] != "-" {
            let cal: Vec<char> = fen[2].chars().collect();
            for c in cal {
                casling |= match c {
                    'q' => 1 << 0,
                    'k' => 1 << 1,
                    'Q' => 1 << 2,
                    'K' => 1 << 3,
                    _ => 0,
                }
            }
        }

        //En Passant
        if fen[3] != "-" {
            let square: Vec<char> = fen[3].chars().collect();
            let f = (square[0].to_ascii_lowercase() as u8) - 96; //a:0, h:9
            let r = square[1].to_digit(10).unwrap() as u8;
            println!("{}:{} = {}", r, f, (r * 8) + f);
            ep = ((r - 1) * 8) + (f - 1);
        }

        //Halfmove
        if fen.len() > 3 {
            hm = fen[4].parse::<u16>().unwrap();
        }
        //Fullmove
        if fen.len() > 4 {
            fm = fen[5].parse::<u64>().unwrap();
        }
        //println!("{:?}", fen);
        return Board {
            pawns: [wp, bp],
            bishops: [wb, bb],
            knights: [wn, bn],
            rooks: [wr, br],
            queens: [wq, bq],
            kings: [wk, bk],
            turn: if wt {
                PieceColor::White
            } else {
                PieceColor::Black
            },
            casling: casling,
            check: 0, //TODO CHECK IF ANY KING IS IN CHECK AND BY WHO
            en_passant: ep,
            half_moves: hm,
            full_move: fm,
            occupied: wp | wb | wn | wr | wq | wk | bp | bb | bn | br | bq | bk,
            pieces: [wp | wb | wn | wr | wq | wk, bp | bb | bn | br | bq | bk],
        };
    }

    /*fn get_pieces(&self, color: PieceColor, type_of: PieceType) -> u64 {
        match type_of {
            PieceType::Pawn => self.pawns[color],
            _ => 0
        }
    }*/

    pub fn lookup(&self, pos: usize) -> (PieceColor, PieceType) {
        let board = 1 << pos;

        let mut color: PieceColor;

        if board & self.pieces[PieceColor::White] != 0 {
            //White
            //println!("ggggg");
            color = PieceColor::White;
        } else {
            //Black
            color = PieceColor::Black;
        }

        if board & (self.pawns[color] | self.bishops[color] | self.knights[color]) != 0 {
            if board & self.pawns[color] != 0 {
                return (color, PieceType::Pawn);
            } else if board & self.bishops[color] != 0 {
                return (color, PieceType::Bishop);
            } else {
                //knights
                return (color, PieceType::Knight);
            }
        } else if board & (self.rooks[color] | self.queens[color] | self.kings[color]) != 0 {
            if board & self.rooks[color] != 0 {
                return (color, PieceType::Rook);
            } else if board & self.queens[color] != 0 {
                return (color, PieceType::Queen);
            } else {
                //kings
                return (color, PieceType::King);
            }
        }

        return (PieceColor::White, PieceType::Empty);
    }

    pub fn move_piece(&self, to: usize, from: usize) -> Board {
        let mut new_board = self.clone();

        //-----------------------------
        //self.print_board();
        //print_bitboard(self.queens[PieceColor::White]);
        let (pc, pt) = self.lookup(from);
        //println!("{:?} : {:?}", pc, pt);
        //print_bitboard(1 << to);
        let (old_pc, old_pt) = self.lookup(to);
        //println!("{:?} : {:?}", old_pc, old_pt);
        //Delete old

        //SET CHECK BY AND WITH PIECE ATTACKS AND KING POS
        //STROGE POS OF ATTACK IN CHECK

        //Check if en_passant needs updating
        if pt == PieceType::Pawn {

            if to == new_board.en_passant as usize {
                //remove pawn at en_pass
                match new_board.turn {
                    PieceColor::White => {
                        new_board.pawns[PieceColor::Black] =
                            new_board.pawns[PieceColor::Black] & !(1 << new_board.en_passant - 8)
                    }
                    PieceColor::Black => {
                        new_board.pawns[PieceColor::White] =
                            new_board.pawns[PieceColor::White] & !(1 << new_board.en_passant + 8)
                    }
                };
            }

            if (to - 16) == from && from > 7 && from < 16 {
                //white
                new_board.en_passant = (to as u8) - 8; //south_one
            } else if to > 15 && (to - 16) == from && from > 47 && from < 56 {
                //black
                new_board.en_passant = (to as u8) + 8; //north_one
            } else {
                new_board.en_passant = 65;
            }
        } else {
            new_board.en_passant = 65;
        }
        //Update in new board


        //CHECK FOR CHECK
        if new_board.casling != 0 && (pt == PieceType::Rook || pt == PieceType::King) {
            let values = match pc {
                PieceColor::White => 0b0011,
                PieceColor::Black => 0b1100,
            };

            if pt == PieceType::King {
                new_board.casling &= values
            }

            //if rook cencel side its on
            //check if from mathces
            match from {
                0 => new_board.casling &= 0b1011, //white queenside
                7 => new_board.casling &= 0b0111, //white kingside
                56 => new_board.casling &= 0b1011, //black queenside
                63 => new_board.casling &= 0b0111, //black kingside
                _ => {}
            };

            //qk
            //white = 11
        }

        //Remove Opps piece from to pos
        match old_pt {
            PieceType::Pawn => new_board.pawns[old_pc] = new_board.pawns[old_pc] & !(1 << to),
            PieceType::Bishop => new_board.bishops[old_pc] = new_board.bishops[old_pc] & !(1 << to),
            PieceType::Knight => new_board.knights[old_pc] = new_board.knights[old_pc] & !(1 << to),
            PieceType::Rook => new_board.rooks[old_pc] = new_board.rooks[old_pc] & !(1 << to),
            PieceType::Queen => new_board.queens[old_pc] = new_board.queens[old_pc] & !(1 << to),
            PieceType::King => new_board.kings[old_pc] = new_board.kings[old_pc] & !(1 << to),
            PieceType::Empty => (),
        };

        //Remove from pos piece
        match pt {
            PieceType::Pawn => new_board.pawns[pc] = new_board.pawns[pc] & !(1 << from),
            PieceType::Bishop => new_board.bishops[pc] = new_board.bishops[pc] & !(1 << from),
            PieceType::Knight => new_board.knights[pc] = new_board.knights[pc] & !(1 << from),
            PieceType::Rook => new_board.rooks[pc] = new_board.rooks[pc] & !(1 << from),
            PieceType::Queen => new_board.queens[pc] = new_board.queens[pc] & !(1 << from),
            PieceType::King => new_board.kings[pc] = new_board.kings[pc] & !(1 << from),
            PieceType::Empty => (),
        };
        //Add piece to to pos
        match pt {
            PieceType::Pawn => new_board.pawns[pc] = new_board.pawns[pc] | (1 << to),
            PieceType::Bishop => new_board.bishops[pc] = new_board.bishops[pc] | (1 << to),
            PieceType::Knight => new_board.knights[pc] = new_board.knights[pc] | (1 << to),
            PieceType::Rook => new_board.rooks[pc] = new_board.rooks[pc] | (1 << to),
            PieceType::Queen => new_board.queens[pc] = new_board.queens[pc] | (1 << to),
            PieceType::King => new_board.kings[pc] = new_board.kings[pc] | (1 << to),
            PieceType::Empty => (),
        };

        //-----------------------------

        let white_pieces = new_board.pawns[PieceColor::White]
           | new_board.bishops[PieceColor::White]
           | new_board.knights[PieceColor::White]
           | new_board.rooks[PieceColor::White]
           | new_board.queens[PieceColor::White]
           | new_board.kings[PieceColor::White];
        let black_pieces = new_board.pawns[PieceColor::Black]
           | new_board.bishops[PieceColor::Black]
           | new_board.knights[PieceColor::Black]
           | new_board.rooks[PieceColor::Black]
           | new_board.queens[PieceColor::Black]
           | new_board.kings[PieceColor::Black];

        new_board.next_turn();

        new_board.occupied = white_pieces | black_pieces;
        new_board.pieces = [white_pieces, black_pieces];

        return new_board;
    }

    pub fn promote(&self, from: usize, to: usize) -> Vec<Move> {
        let mut new_board = self.clone();

        let (pc, pt) = new_board.lookup(from);
        let (old_pc, old_pt) = new_board.lookup(to);

        new_board.next_turn();
        //Remove old pawn
        match pt {
            PieceType::Pawn => new_board.pawns[pc] = new_board.pawns[pc] & !(1 << from),
            PieceType::Bishop => new_board.bishops[pc] = new_board.bishops[pc] & !(1 << from),
            PieceType::Knight => new_board.knights[pc] = new_board.knights[pc] & !(1 << from),
            PieceType::Rook => new_board.rooks[pc] = new_board.rooks[pc] & !(1 << from),
            PieceType::Queen => new_board.queens[pc] = new_board.queens[pc] & !(1 << from),
            PieceType::King => new_board.kings[pc] = new_board.kings[pc] & !(1 << from),
            PieceType::Empty => (),
        };

        //Remove Opps Piece
        match old_pt {
            PieceType::Pawn => new_board.pawns[old_pc] = new_board.pawns[old_pc] & !(1 << to),
            PieceType::Bishop => new_board.bishops[old_pc] = new_board.bishops[old_pc] & !(1 << to),
            PieceType::Knight => new_board.knights[old_pc] = new_board.knights[old_pc] & !(1 << to),
            PieceType::Rook => new_board.rooks[old_pc] = new_board.rooks[old_pc] & !(1 << to),
            PieceType::Queen => new_board.queens[old_pc] = new_board.queens[old_pc] & !(1 << to),
            PieceType::King => new_board.kings[old_pc] = new_board.kings[old_pc] & !(1 << to),
            PieceType::Empty => (),
        };

        let mut new_boards = vec!(new_board.clone(); 4);

        new_boards[0].knights[pc] = new_board.knights[pc] | (1 << to);
        new_boards[1].bishops[pc] = new_board.bishops[pc] | (1 << to);
        new_boards[2].rooks[pc] = new_board.rooks[pc] | (1 << to);
        new_boards[3].queens[pc] = new_board.queens[pc] | (1 << to);

        //Check for check
        let mut out: Vec<Move> = vec![];

        //In case other piece is removed all case need to be run
        for i in 0..4 {
            let white_pieces = new_boards[i].pawns[PieceColor::White]
                | new_boards[i].bishops[PieceColor::White]
                | new_boards[i].knights[PieceColor::White]
                | new_boards[i].rooks[PieceColor::White]
                | new_boards[i].queens[PieceColor::White]
                | new_boards[i].kings[PieceColor::White];
            let black_pieces = new_board.pawns[PieceColor::Black]
                | new_boards[i].bishops[PieceColor::Black]
                | new_boards[i].knights[PieceColor::Black]
                | new_boards[i].rooks[PieceColor::Black]
                | new_boards[i].queens[PieceColor::Black]
                | new_boards[i].kings[PieceColor::Black];

            new_boards[i].occupied = white_pieces | black_pieces;
            new_boards[i].pieces = [white_pieces, black_pieces];

            out.push((from, to, new_boards[i]));
        }

        return out;
    }

    fn next_turn(&mut self)  {

        if self.turn == PieceColor::Black {
            self.turn = PieceColor::White;
            self.full_move += 1;
        } else {
            self.turn = PieceColor::Black;
        };

        self.half_moves = self.half_moves + 1;

    }

    pub fn print_board(&self) {
        println!(
            "\n{} to move:",
            (if self.turn == PieceColor::White {
                "White"
            } else {
                "Black"
            })
        );
        println!("-----");
        for r in [7, 6, 5, 4, 3, 2, 1, 0] {
            //cant be bothered
            for f in 0..8 {
                let i = (r * 8) + f;

                if ((self.pawns[PieceColor::White] >> i) & 1) == 1 {
                    print!("P ");
                } else if ((self.bishops[PieceColor::White] >> i) & 1) == 1 {
                    print!("B ");
                } else if ((self.knights[PieceColor::White] >> i) & 1) == 1 {
                    print!("N ");
                } else if ((self.rooks[PieceColor::White] >> i) & 1) == 1 {
                    print!("R ");
                } else if ((self.queens[PieceColor::White] >> i) & 1) == 1 {
                    print!("Q ");
                } else if ((self.kings[PieceColor::White] >> i) & 1) == 1 {
                    print!("K ");
                } else if ((self.pawns[PieceColor::Black] >> i) & 1) == 1 {
                    print!("p ");
                } else if ((self.bishops[PieceColor::Black] >> i) & 1) == 1 {
                    print!("b ");
                } else if ((self.knights[PieceColor::Black] >> i) & 1) == 1 {
                    print!("n ");
                } else if ((self.rooks[PieceColor::Black] >> i) & 1) == 1 {
                    print!("r ");
                } else if ((self.queens[PieceColor::Black] >> i) & 1) == 1 {
                    print!("q ");
                } else if ((self.kings[PieceColor::Black] >> i) & 1) == 1 {
                    print!("k ");
                } else {
                    if i == self.en_passant {
                        print!("# ");
                    } else {
                        print!(". ");
                    }
                }
            }
            println!();
        }
        println!("-----");
        println!(
            "Castling Rights: {}{} {}{}",
            if ((self.casling >> 0) & 1) == 1 {
                "Q"
            } else {
                "-"
            },
            if ((self.casling >> 1) & 1) == 1 {
                "K"
            } else {
                "-"
            },
            if ((self.casling >> 2) & 1) == 1 {
                "q"
            } else {
                "-"
            },
            if ((self.casling >> 3) & 1) == 1 {
                "k"
            } else {
                "-"
            }
        );
        println!("Halfmoves: {}", self.half_moves);
        println!("Fullmoves: {}", self.full_move);

        println!();
    }
}
