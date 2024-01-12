//use super::bitboard::{};

pub struct Board {
    pub white_pawns: u64,
    pub white_bishops: u64,
    pub white_knights: u64,
    pub white_rooks: u64,
    pub white_queens: u64,
    pub white_king: u64,

    pub black_pawns: u64,
    pub black_bishops: u64,
    pub black_knights: u64,
    pub black_rooks: u64,
    pub black_queens: u64,
    pub black_king: u64,

    pub white_turn: bool,
    pub casling: u8,    //white, black | queenside, kingside QKqk
    pub en_passant: u8, //postion of avilbe en passant
    pub half_moves: u16,
    pub full_move: u64,
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
            white_pawns: wp,
            white_bishops: wb,
            white_knights: wn,
            white_rooks: wr,
            white_queens: wq,
            white_king: wk,
            black_pawns: bp,
            black_bishops: bb,
            black_knights: bn,
            black_rooks: br,
            black_queens: bq,
            black_king: bk,
            white_turn: wt,
            casling: casling,
            en_passant: ep,
            half_moves: hm,
            full_move: fm,
        };
    }

    pub fn occupied(&self) -> u64 {
        self.white_pawns
            | self.white_bishops
            | self.white_knights
            | self.white_rooks
            | self.white_queens
            | self.white_king
            | self.black_pawns
            | self.black_bishops
            | self.black_knights
            | self.black_rooks
            | self.black_queens
            | self.black_king
    }

    pub fn black_pieces(&self) -> u64 {
        self.black_pawns
            | self.black_bishops
            | self.black_knights
            | self.black_rooks
            | self.black_queens
            | self.black_king
    }

    pub fn white_pieces(&self) -> u64 {
        self.white_pawns
            | self.white_bishops
            | self.white_knights
            | self.white_rooks
            | self.white_queens
            | self.white_king
    }

    pub fn print_board(&self) {
        println!(
            "\n{} to move:",
            (if self.white_turn { "White" } else { "Black" })
        );
        println!("-----");
        for r in [7, 6, 5, 4, 3, 2, 1, 0] {
            //cant be bothered
            for f in 0..8 {
                let i = (r * 8) + f;

                if ((self.white_pawns >> i) & 1) == 1 {
                    print!("P ");
                } else if ((self.white_bishops >> i) & 1) == 1 {
                    print!("B ");
                } else if ((self.white_knights >> i) & 1) == 1 {
                    print!("N ");
                } else if ((self.white_rooks >> i) & 1) == 1 {
                    print!("R ");
                } else if ((self.white_queens >> i) & 1) == 1 {
                    print!("Q ");
                } else if ((self.white_king >> i) & 1) == 1 {
                    print!("K ");
                } else if ((self.black_pawns >> i) & 1) == 1 {
                    print!("p ");
                } else if ((self.black_bishops >> i) & 1) == 1 {
                    print!("b ");
                } else if ((self.black_knights >> i) & 1) == 1 {
                    print!("n ");
                } else if ((self.black_rooks >> i) & 1) == 1 {
                    print!("r ");
                } else if ((self.black_queens >> i) & 1) == 1 {
                    print!("q ");
                } else if ((self.black_king >> i) & 1) == 1 {
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
