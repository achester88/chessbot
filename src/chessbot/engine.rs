use super::{bitboard, board, utils};
use bitboard::*;
use board::*;
use utils::*;

type Move = (usize, usize, Board);

#[derive(Debug, Clone, Copy)]
pub enum Dir {
    North,
    South,
    West,
    East,
    NOEA,
    NOWE,
    SOWE,
    SOEA,
}

pub struct Engine {
    ray_attacks: Vec<Vec<u64>>,
    pawn_attacks: Vec<Vec<u64>>,
    king_attacks: Vec<u64>,
    knight_attacks: Vec<u64>,
}

impl Engine {
    pub fn new() -> Self {
        return Engine {
            ray_attacks: gen_ray_attacks(),
            pawn_attacks: gen_pawn_attacks(),
            king_attacks: gen_king_attacks(),
            knight_attacks: gen_knight_attacks(),
        };
    }
    //from, to, new board
    pub fn gen_moves(&self, board: Board) -> Vec<Move> {
        println!("---------- Gen Moves {} -----------", board.half_moves);

        board.print_board();
        println!("\n");

        let mut possable: Vec<(usize, u64)> = vec![];

        let queens = board_serialize(board.queens[board.turn]);
        for i in queens {
            //possable.push(self.gen_queen_moves(&board, i, board.pieces[board.turn]));
        }

        let bishops = board_serialize(board.bishops[board.turn]);
        for i in bishops {
            //possable.push(self.gen_bishops_moves(&board, i, board.pieces[board.turn]));
        }

        let rooks = board_serialize(board.rooks[board.turn]);
        for i in rooks {
            //possable.push(self.gen_rooks_moves(&board, i, board.pieces[board.turn]));
        }

        let pawns = board_serialize(board.pawns[board.turn]);
        for i in pawns {
            //possable.push(self.gen_pawn_moves(&board, i));
        }

        let kings = board_serialize(board.kings[board.turn]);
        for i in kings {
            //possable.push(self.gen_king_moves(&board, i));
        }

        let knight = board_serialize(board.knights[board.turn]);
        for i in knight {
            possable.push(self.gen_knight_moves(&board, i));
        }

        let mut all_moves: Vec<Move> = vec![];

        for i in 0..possable.len() {
            let (from, moves) = possable[i];
            let moves_to = board_serialize(moves);
            //board.print_board();
            for i in 0..moves_to.len() {
                let to = moves_to[i];
                let new_board = board.move_piece(to, from);
                new_board.print_board();
                all_moves.push((from, to, new_board));
            }
        }

        let pawns = board_serialize(board.pawns[board.turn]);
        //print_bitboard(board.pawns[board.turn]);

        println!("---------- END {} -----------", board.half_moves);

        return all_moves;
    }

    pub fn gen_knight_moves(&self, board: &Board, sq: usize) -> (usize, u64) {

        let opp = match board.turn {
        PieceColor::White => board.pieces[PieceColor::Black],
        PieceColor::Black => board.pieces[PieceColor::White],
        };

        let mut attacks = self.knight_attacks[sq] & (!board.occupied | opp);

        return (sq, attacks);
    }
    
    pub fn gen_king_moves(&self, board: &Board, sq: usize) -> (usize, u64) {

        let opp = match board.turn {
        PieceColor::White => board.pieces[PieceColor::Black],
        PieceColor::Black => board.pieces[PieceColor::White],
        };
        
        let mut attacks = self.king_attacks[sq] & (!board.occupied | opp);
        
        return (sq, attacks);
    }
    
    pub fn gen_pawn_moves(&self, board: &Board, sq: usize) -> (usize, u64) {
        let mut moves = 0;
        let piece = 1 << sq;

        if board.turn == PieceColor::White {
            moves = moves | (postshift::nort_one(piece) & !board.occupied);
            if (piece & 0xff00) != 0 {
                moves = moves | (postshift::nort_one(moves) & !board.occupied);
            }

            let mut en_pass: u64 = 0;

            if board.en_passant != 65 {
                en_pass = 1 << board.en_passant;
            }
            
            moves = moves
                | (self.pawn_attacks[PieceColor::White as usize][sq]
                & (board.pieces[PieceColor::Black as usize] | en_pass));

        } else { //Black
            moves = moves | (postshift::sout_one(piece) & !board.occupied);
            if (piece & 0xff000000000000) != 0 {
                moves = moves | (postshift::sout_one(moves) & !board.occupied);
            }

            let mut en_pass: u64 = 0;

            if board.en_passant != 65 {
                en_pass = 1 << board.en_passant;
            }

            moves = moves
                | (self.pawn_attacks[PieceColor::Black as usize][sq]
                & (board.pieces[PieceColor::White as usize] | en_pass));
        }
      
        //print_bitboard_pos(moves, sq);
        //print_bitboard_pos(0, board.en_passant as usize);

        //
        return (sq, moves);
    }

    pub fn gen_rook_moves(&self, board: &Board, sq: usize, pieces: u64) -> (usize, u64) {
        let all_moves = self.gen_ray_attacks(board.occupied, Dir::North, sq)
            | self.gen_ray_attacks(board.occupied, Dir::South, sq)
            | self.gen_ray_attacks(board.occupied, Dir::East, sq)
            | self.gen_ray_attacks(board.occupied, Dir::West, sq);

        let attack = all_moves & !pieces;

        //print_bitboard_pos(all_moves & !self.board.pieces[self.board.turn], sq);
        return (sq, attack); //board_serialize(attack);
    }

    pub fn gen_bishop_moves(&self, board: &Board, sq: usize, pieces: u64) -> (usize, u64) {
        let all_moves = self.gen_ray_attacks(board.occupied, Dir::NOEA, sq)
            | self.gen_ray_attacks(board.occupied, Dir::NOWE, sq)
            | self.gen_ray_attacks(board.occupied, Dir::SOEA, sq)
            | self.gen_ray_attacks(board.occupied, Dir::SOWE, sq);

        let attack = all_moves & !pieces;

        //print_bitboard_pos(attack, sq);
        return (sq, attack); //board_serialize(attack);
    }

    pub fn gen_queen_moves(&self, board: &Board, sq: usize, pieces: u64) -> (usize, u64) {
        let attack =
            self.gen_rook_moves(board, sq, pieces).1 | self.gen_bishop_moves(board, sq, pieces).1;

        //let attack = all_moves & !pieces;

        //print_bitboard_pos(attack, sq);
        return (sq, attack); //board_serialize(attack);
    }

    pub fn gen_ray_attacks(&self, occupied: u64, dir: Dir, square: usize) -> u64 {
        //print_bitboard(occupied);
        //print_bitboard(1 << square);
        let set = &self.ray_attacks[dir as usize];
        let mut attack: u64 = set[square];
        let block = attack & occupied;
        if block != 0 {
            let stop: usize;
            if dir_is_pos(dir) {
                stop = bit_scan(block);
            } else {
                stop = bit_scan_neg(block);
            }
            attack = attack ^ set[stop];
        }
        return attack;
    }
    //move fuction that return a new board after that move
}

//https://www.chessprogramming.org/On_an_empty_Board#Rays_by_Line

fn dir_is_pos(dir: Dir) -> bool {
    match dir {
        Dir::North | Dir::NOWE | Dir::NOEA | Dir::East => true,
        Dir::South | Dir::West | Dir::SOWE | Dir::SOEA => false,
    }
}

fn gen_ray_attacks() -> Vec<Vec<u64>> {
    let mut attacks: Vec<Vec<u64>> = vec![vec![0; 64]; 8];

    //North
    let mut nort: u64 = 0x0101010101010100;
    for i in 0..64 {
        attacks[Dir::North as usize][i as usize] = nort;
        nort <<= 1;
    }
    //South
    let mut south: u64 = 0x80808080808080;
    for i in (0..64).rev() {
        attacks[Dir::South as usize][i as usize] = south;
        south >>= 1;
    }

    //East
    let mut east: u64;
    for i in 0..8 {
        east = 0xfe << (8 * i);
        for ii in 0..8 {
            attacks[Dir::East as usize][((i * 8) + ii) as usize] = east;
            east = postshift::east_one(east);
        }
    }

    //West
    let mut west: u64;
    for i in 0..8 {
        west = 0x7f << (8 * i);
        for ii in 0..8 {
            attacks[Dir::West as usize][((i * 8) + (7 - ii)) as usize] = west;
            west = postshift::west_one(west);
        }
    }

    //noea
    let mut noea;
    for i in 0..8 {
        noea = 0x8040201008040200 << (8 * i);
        for ii in 0..8 {
            attacks[Dir::NOEA as usize][((i * 8) + ii) as usize] = noea;

            noea = postshift::east_one(noea);
        }
    }

    //nowe
    let mut nowe: u64 = 0x8040201008040200;
    for i in 0..8 {
        nowe = 0x102040810204000 << (8 * i);
        for ii in 0..8 {
            attacks[Dir::NOWE as usize][((i * 8) + (7 - ii)) as usize] = nowe;

            nowe = postshift::west_one(nowe);
        }
    }

    //soea
    let mut soea: u64;
    for i in (0..8).rev() {
        soea = 0x2040810204080 >> (8 * (7 - i));
        for ii in 0..8 {
            attacks[Dir::SOEA as usize][((i * 8) + ii) as usize] = soea;

            soea = postshift::east_one(soea);
        }
    }

    //sowe
    let mut sowe: u64;
    for i in (0..8).rev() {
        sowe = 0x40201008040201 >> (8 * (7 - i));
        for ii in 0..8 {
            attacks[Dir::SOWE as usize][((i * 8) + (7 - ii)) as usize] = sowe;

            sowe = postshift::west_one(sowe);
        }
    }

    /*for i in 0..64 {
      print_bitboard_pos(attacks[Dir::SOWE as usize][i],i);
    }*/

    return attacks;
}

fn gen_pawn_attacks() -> Vec<Vec<u64>> {
    let mut attacks: Vec<Vec<u64>> = vec![vec![0; 64]; 2];

    for i in 8..56 {
        let bb = 1 << i;

        attacks[PieceColor::White as usize][i] =
            postshift::no_ea_one(bb) | postshift::no_we_one(bb);
    }

    for i in 8..56 {
        let bb = 1 << i;

        attacks[PieceColor::Black as usize][i] =
            postshift::so_ea_one(bb) | postshift::so_we_one(bb);
    }

    return attacks;
}

fn gen_king_attacks() -> Vec<u64> {
    
    let mut attacks: Vec<u64> = vec![0; 64];

    for i in 0..63 {
        let mut pos = 1 << i;

        pos |= postshift::east_one(pos) | postshift::west_one(pos);
        pos |= postshift::nort_one(pos) | postshift::sout_one(pos);

        attacks[i] = pos;
    }

    return attacks;
}

fn gen_knight_attacks() -> Vec<u64> {

    let mut attacks: Vec<u64> = vec![0; 64];

    for i in 0..63 {
        let pos = 1 << i;
        let mut attack = 0;
        attack |= postshift::no_ea_one(postshift::nort_one(pos));
        attack |= postshift::no_we_one(postshift::nort_one(pos));
        attack |= postshift::so_ea_one(postshift::sout_one(pos));
        attack |= postshift::so_we_one(postshift::sout_one(pos));

        attack |= postshift::no_ea_one(postshift::east_one(pos));
        attack |= postshift::no_we_one(postshift::west_one(pos));
        attack |= postshift::so_ea_one(postshift::east_one(pos));
        attack |= postshift::so_we_one(postshift::west_one(pos));

        attacks[i] = attack;
    }

    return attacks;
}

//https://tearth.dev/bitboard-viewer/
