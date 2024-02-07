use super::bitboard::{print_bitboard, print_bitboard_pos};
use core::ops::Index;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PieceColor {
    White,
    Black
}

#[derive(Debug)]
pub enum PieceType {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
    Empty,
}

impl Index<PieceColor> for [u64] {
    type Output = u64;

    fn index(&self, color: PieceColor) -> &Self::Output {
        match color {
            PieceColor::White => &self[0],
            PieceColor::Black => &self[1]
        }
    }
}

#[derive(Clone)]
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
    pub half_moves: u16,
    pub full_move: u64,

    pub occupied: u64,
    pub pieces: [u64; 2],
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
        let mut i = 0;
        let fen: Vec<&str> = fen_str.split(" ").collect();

        //postion
        let pos = fen[0].replace("/", "");
        let pos_vec: Vec<char> = pos.chars().collect();
        for c in pos_vec {
            if c.is_numeric() {
                //println!("{}", c);
                i += c.to_digit(10).unwrap() as usize;
            } else {
                //println!("{}", c);
                let s = 1 << (63 - i); //set bit of i;
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

                i += 1;
            }
        }
        //turn
        wt = fen[1] == "w";

        //casling
        if fen[2] != "-" {
            let cal: Vec<char> = fen[2].chars().collect();
            for c in cal {
                casling = casling
                    | match c {
                        'Q' => 1 << 0,
                        'K' => 1 << 1,
                        'q' => 1 << 2,
                        'k' => 1 << 3,
                        _ => 0,
                    }
            }
        }

        //En Passant
        if fen[3] != "-" {
            let square: Vec<char> = fen[3].chars().collect();
            let f = (square[0].to_ascii_lowercase() as u8) - 97; //a:0, h:9
            let r = square[1].to_digit(10).unwrap() as u8;
            //println!("{}:{} = {}", r, f, (r*8)+f);
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
            knights: [wk, bk],
            rooks: [wr, br],
            queens: [wq, bq],
            kings: [wk, bk],
            turn: if wt {PieceColor::White} else {PieceColor::Black},
            casling: casling,
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

    fn lookup(&self, pos: usize) -> (PieceColor, PieceType) {
        let board = 1 << pos;
        
        let mut color: PieceColor;

        if board & self.pieces[PieceColor::White] != 0 { //White
            println!("ggggg");
            color = PieceColor::White;
        } else { //Black
            color = PieceColor::Black;
        }

        if board & (self.pawns[color] | self.bishops[color] | self.knights[color]) != 0 {
            if board & self.pawns[color] != 0 {
                return (color, PieceType::Pawn);
            } else if board & self.bishops[color] != 0 {
                return (color, PieceType::Bishop);
            } else  { //knights
                return (color, PieceType::Knight);
            }
        } else if board & (self.rooks[color] | self.queens[color] | self.kings[color]) != 0 {
            if board & self.rooks[color] != 0 {
                return (color, PieceType::Rook);
            } else if board & self.queens[color] != 0 {
                return (color, PieceType::Queen);
            } else  { //kings
                return (color, PieceType::King);
            }
        }

        return (PieceColor::White, PieceType::Empty);
    }

    pub fn move_piece(&self, to: usize, from: usize) -> Board {
        
        let mut pawns = self.pawns;
        let mut bishops = self.bishops;
        let mut knights = self.knights;
        let mut rooks = self.rooks;
        let mut queens = self.queens;
        let mut kings = self.kings;
       
        //-----------------------------

        let (pc, pt) = self.lookup(from);
        println!("{:?} : {:?}", pc, pt);
        //let (old_pc, old_pt) = self.lookup(to);
        //Delete old
        
        //Move new

        //-----------------------------

        let white_pieces = self.pawns[PieceColor::White] | self.bishops[PieceColor::White] | self.knights[PieceColor::White] |
        self.rooks[PieceColor::White] | self.queens[PieceColor::White] | self.kings[PieceColor::White];
        let black_pieces = self.pawns[PieceColor::Black] | self.bishops[PieceColor::Black] | self.knights[PieceColor::Black] |
        self.rooks[PieceColor::Black] | self.queens[PieceColor::Black] | self.kings[PieceColor::Black];
        // **** Recalc en_passant and casling *****
        return Board {
            pawns: pawns,
            bishops: bishops,
            knights: knights,
            rooks: rooks,
            queens: queens,
            kings: kings,
            turn: if self.turn == PieceColor::White {PieceColor::Black} else {PieceColor::White},
            casling: self.casling,
            en_passant: self.en_passant,
            half_moves: self.half_moves + 1,
            full_move: if self.half_moves % 2 == 0 {self.full_move + 1} else {self.full_move},
            occupied: white_pieces | black_pieces,
            pieces: [white_pieces, black_pieces],
        };
    }

    pub fn print_board(&self) {
        println!(
            "\n{} to move:",
            (if self.turn == PieceColor::White { "White" } else { "Black" })
        );
        println!("-----");
        for r in [7, 6, 5, 4, 3, 2, 1, 0] {
            //cant be bothered
            for f in 0..8 {
                let i = (r * 8) + f;

                if ((self.pawns[PieceColor::Black] >> i) & 1) == 1 {
                    print!("P ");
                } else if ((self.bishops[PieceColor::Black] >> i) & 1) == 1 {
                    print!("B ");
                } else if ((self.knights[PieceColor::Black] >> i) & 1) == 1 {
                    print!("N ");
                } else if ((self.rooks[PieceColor::Black] >> i) & 1) == 1 {
                    print!("R ");
                } else if ((self.queens[PieceColor::Black] >> i) & 1) == 1 {
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
