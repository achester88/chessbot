use super::bitboard::{print_bitboard, print_bitboard_pos};
use core::ops::{Index, IndexMut, Not};
use crate::chessbot::bitboard::board_serialize;
use crate::chessbot::engine::Move;
use crate::chessbot::engine::Engine;

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

impl Not for PieceColor {
    type Output = PieceColor; // The output type is also MyBoolean

    fn not(self) -> Self::Output {
        match self {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
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
    pub casling_attacks: [u64; 4],
    pub en_passant: u8, //postion of avilbe en passant
    pub check_real: u64, //TODO USE BOOL AND CAL AS NEEDED
    pub check_full: u64,
    pub half_moves: u16,
    pub full_move: u64,

    pub occupied: u64,
    pub pieces: [u64; 2], //All piece of the same color
}

impl Board {
    pub fn new(fen_str: &str, engine: &Engine) -> Self {
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
                //i -= c.to_digit(10).unwrap() as usize;
                f += c.to_digit(10).unwrap() as usize;
            } else {
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
                    'Q' => 1 << 3,
                    'K' => 1 << 2,
                    'q' => 1 << 1,
                    'k' => 1 << 0,
                    _ => 0,
                }
            }
        }

        //casling |= casling << 4;

        //En Passant
        if fen[3] != "-" {
            let square: Vec<char> = fen[3].chars().collect();
            let f = (square[0].to_ascii_lowercase() as u8) - 96; //a:0, h:9
            let r = square[1].to_digit(10).unwrap() as u8;
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

        let mut new_board = Board {
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
            casling: 0b0000_0000 | casling,
            casling_attacks: [0; 4],//TODO CHECK IF FEN STRING HAS ANY ATTACKS
            check_real: 0,
            check_full: 0,
            en_passant: ep,
            half_moves: hm,
            full_move: fm,
            occupied: wp | wb | wn | wr | wq | wk | bp | bb | bn | br | bq | bk,
            pieces: [wp | wb | wn | wr | wq | wk, bp | bb | bn | br | bq | bk],
        };

        let (casl, casl_att) = engine.gen_init_casling_info(&new_board);

        new_board.casling |= casl;
        new_board.casling_attacks = casl_att;

        let king_board = new_board.kings[new_board.turn];
        if (king_board != 0) {
            let king_pos = board_serialize(king_board);
            let (cr, cf) = engine.cal_check(&new_board, king_pos[0], !new_board.turn);


            new_board.check_real = cr;
            new_board.check_full = cf;
        }

        return new_board;
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
        } else {
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

        let (pc, pt) = self.lookup(from);
        let (old_pc, old_pt) = self.lookup(to);

        //Check if en_passant needs updating
        new_board.en_passant_check(to, from, &pt);

        //CHECK FOR CHECK
        if new_board.casling != 0 && (pt == PieceType::Rook || pt == PieceType::King) {
            if pt == PieceType::King {
                let values; 
                match pc {
                    PieceColor::White => {
                        values = 0b0011_0011;
                        new_board.casling_attacks[2] = 0;
                        new_board.casling_attacks[3] = 0;
                    },
                    PieceColor::Black => {
                        values = 0b1100_1100;
                        new_board.casling_attacks[0] = 0;
                        new_board.casling_attacks[1] = 0;

                    }
                };

                new_board.casling &= values;
            } else {
                //if rook cencel side its on
                //check if from mathces
                match from {
                    0 => {
                        new_board.casling &= 0b0111_0111;
                        new_board.casling_attacks[3] = 0; 
                    }, //white queenside
                    7 => {
                        new_board.casling &= 0b1011_1011;
                        new_board.casling_attacks[2] = 0;
                    }, //white kingside
                    56 => {
                        new_board.casling &= 0b1101_1101;
                        new_board.casling_attacks[1] = 0; 
                    }, //black queenside
                    63 => {
                        new_board.casling &= 0b1110_1110;
                        new_board.casling_attacks[0] = 0; 
                    }, //black kingside
                    _ => {}
                };
            }

            //qk
            //white = 11
        }

        //Remove Opps piece from to pos
        new_board.remove_piece(to, &old_pt, old_pc);
        //Remove from pos piece
        new_board.remove_piece(from, &pt, pc);
        //Add piece to to pos
        new_board.add_piece(to, &pt, pc);

        new_board.recalc_board();

        new_board.next_turn();

        return new_board;
    }

    fn en_passant_check(&mut self, to: usize, from: usize, pt: &PieceType) {

        if pt == &PieceType::Pawn {
            if to == self.en_passant as usize {
                //remove pawn at en_pass
                match self.turn {
                    PieceColor::White => {
                        self.pawns[PieceColor::Black] =
                            self.pawns[PieceColor::Black] & !(1 << self.en_passant - 8)
                    }
                    PieceColor::Black => {
                        self.pawns[PieceColor::White] =
                            self.pawns[PieceColor::White] & !(1 << self.en_passant + 8)
                    }
                };
            }

            if to > 16 && (to - 16) == from && from > 7 && from < 16 {
                //white
                self.en_passant = (to as u8) - 8; //south_one
            } else if to > 15 && (to - 16) == from && from > 47 && from < 56 {
                //black
                self.en_passant = (to as u8) + 8; //north_one
            } else {
                self.en_passant = 65;
            }
        } else {
            self.en_passant = 65;
        }
    }

    fn remove_piece(&mut self, pos: usize, pt: &PieceType, pc: PieceColor) {
    //Remove Opps piece from to pos
        match pt {
            PieceType::Pawn => self.pawns[pc] = self.pawns[pc] & !(1 << pos),
            PieceType::Bishop => self.bishops[pc] = self.bishops[pc] & !(1 << pos),
            PieceType::Knight => self.knights[pc] = self.knights[pc] & !(1 << pos),
            PieceType::Rook => self.rooks[pc] = self.rooks[pc] & !(1 << pos),
            PieceType::Queen => self.queens[pc] = self.queens[pc] & !(1 << pos),
            PieceType::King => self.kings[pc] = self.kings[pc] & !(1 << pos),
            PieceType::Empty => (),
        };
    }

    fn add_piece(&mut self, pos: usize, pt: &PieceType, pc: PieceColor) {
        match pt {
            PieceType::Pawn => self.pawns[pc] = self.pawns[pc] | (1 << pos),
            PieceType::Bishop => self.bishops[pc] = self.bishops[pc] | (1 << pos),
            PieceType::Knight => self.knights[pc] = self.knights[pc] | (1 << pos),
            PieceType::Rook => self.rooks[pc] = self.rooks[pc] | (1 << pos),
            PieceType::Queen => self.queens[pc] = self.queens[pc] | (1 << pos),
            PieceType::King => self.kings[pc] = self.kings[pc] | (1 << pos),
            PieceType::Empty => (),
        };
    }

    pub fn recalc_board(&mut self) {
        let white_pieces = self.pawns[PieceColor::White]
            | self.bishops[PieceColor::White]
            | self.knights[PieceColor::White]
            | self.rooks[PieceColor::White]
            | self.queens[PieceColor::White]
            | self.kings[PieceColor::White];
        let black_pieces = self.pawns[PieceColor::Black]
            | self.bishops[PieceColor::Black]
            | self.knights[PieceColor::Black]
            | self.rooks[PieceColor::Black]
            | self.queens[PieceColor::Black]
            | self.kings[PieceColor::Black];

        self.occupied = white_pieces | black_pieces;
        self.pieces = [white_pieces, black_pieces];
    }

    pub fn promote(&self, from: usize, to: usize) -> Vec<Move> {
        let mut new_board = self.clone();

        let (pc, pt) = new_board.lookup(from);
        let (old_pc, old_pt) = new_board.lookup(to);

        new_board.next_turn();
        //Remove old pawn
        new_board.remove_piece(to, &old_pt, old_pc);
        //Remove from pos piece
        new_board.remove_piece(from, &pt, pc);
        //Add piece to to pos

        let mut new_boards = vec!(new_board.clone(); 4);

        new_boards[0].knights[pc] = new_board.knights[pc] | (1 << to);
        new_boards[1].bishops[pc] = new_board.bishops[pc] | (1 << to);
        new_boards[2].rooks[pc] = new_board.rooks[pc] | (1 << to);
        new_boards[3].queens[pc] = new_board.queens[pc] | (1 << to);

        //Check for check
        let mut out: Vec<Move> = vec![];

        //In case other piece is removed all case need to be run
        for i in 0..4 {
            new_boards[i].recalc_board();
            out.push((from, to, new_boards[i]));
        }

        return out;
    }

    pub fn castle(&self, code: u8) -> Move {
        let mut new_board = self.clone();

        let king_from_pos: usize;
        let rook_from_pos: usize;

        let king_to_pos: usize;
        let rook_to_pos: usize;

        match new_board.turn {
            PieceColor::White => {
                king_from_pos = 4;
                if code == 80 {
                    rook_from_pos = 7;

                    rook_to_pos = 5;
                    king_to_pos = 6;
                } else {
                    rook_from_pos = 0;

                    rook_to_pos = 3;
                    king_to_pos = 2;
                }

                new_board.casling &= 0b0011;
            },
            PieceColor::Black => {
                king_from_pos = 60;
                if code == 80 {
                    rook_from_pos = 56;

                    rook_to_pos = 61;
                    king_to_pos = 62;
                } else {
                    rook_from_pos = 63;

                    rook_to_pos = 59;
                    king_to_pos = 58;
                }

                new_board.casling &= 0b1100;
            }
        }

        new_board.remove_piece(king_from_pos, &PieceType::King, new_board.turn);
        new_board.remove_piece(rook_from_pos, &PieceType::Rook, new_board.turn);

        new_board.add_piece(king_to_pos, &PieceType::King, new_board.turn);
        new_board.add_piece(rook_to_pos, &PieceType::Rook, new_board.turn);

        new_board.recalc_board();

        new_board.next_turn();

        return (code as usize, code as usize, new_board);
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
        let set = [["P", "N", "B", "R", "Q", "K", "p", "n", "b", "r", "q", "k"], 
                   ["󰡙", "󰡘", "󰡜", "󰡛", "󰡚", "󰡗", "", "", "", "", "", ""]
                ];

        let n = 1; //Replace with cmd arg

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
                    print!("{} ", set[n][0]);
                } else if ((self.bishops[PieceColor::White] >> i) & 1) == 1 {
                    print!("{} ", set[n][2]);
                } else if ((self.knights[PieceColor::White] >> i) & 1) == 1 {
                    print!("{} ", set[n][1]);
                } else if ((self.rooks[PieceColor::White] >> i) & 1) == 1 {
                    print!("{} ", set[n][3]);
                } else if ((self.queens[PieceColor::White] >> i) & 1) == 1 {
                    print!("{} ", set[n][4]);
                } else if ((self.kings[PieceColor::White] >> i) & 1) == 1 {
                    print!("{} ", set[n][5]);
                } else if ((self.pawns[PieceColor::Black] >> i) & 1) == 1 {
                    print!("{} ", set[n][6]);
                } else if ((self.bishops[PieceColor::Black] >> i) & 1) == 1 {
                    print!("{} ", set[n][8]);
                } else if ((self.knights[PieceColor::Black] >> i) & 1) == 1 {
                    print!("{} ", set[n][7]);
                } else if ((self.rooks[PieceColor::Black] >> i) & 1) == 1 {
                    print!("{} ", set[n][9]);
                } else if ((self.queens[PieceColor::Black] >> i) & 1) == 1 {
                    print!("{} ", set[n][10]);
                } else if ((self.kings[PieceColor::Black] >> i) & 1) == 1 {
                    print!("{} ", set[n][11]);
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
            if self.casling & 0b1000 != 0 {
                "Q"
            } else {
                "-"
            },
            if self.casling & 0b0100 != 0 {
                "K"
            } else {
                "-"
            },
            if self.casling & 0b0010 != 0 {
                "q"
            } else {
                "-"
            },
            if self.casling & 0b0001 != 0 {
                "k"
            } else {
                "-"
            }
        );
        println!(
            "Temp Castling Rights: {}{} {}{}",
            if self.casling & 0b1000_0000 != 0 {
                "Q"
            } else {
                "-"
            },
            if self.casling & 0b0100_0000 != 0 {
                "K"
            } else {
                "-"
            },
            if self.casling & 0b0010_0000 != 0 {
                "q"
            } else {
                "-"
            },
            if self.casling & 0b0001_0000 != 0 {
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
