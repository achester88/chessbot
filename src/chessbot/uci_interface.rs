use std::process::Command;
use crate::chessbot::board::Board;
use crate::chessbot::engine::Engine;
use crate::chessbot::uci_interface::Cmd::Set;

#[derive(Debug, Clone, PartialEq)]
pub enum Cmd {
    Set(Board),
    Stop,
    GoInf
}

pub struct UciInterface {
    pub current_board: Option<Board>,
    current_move: usize
}
impl UciInterface {

    pub fn new() -> Self {
        UciInterface {
            current_board: None,
            current_move: 0
        }
    }

    pub fn uci(&mut self) -> Option<Cmd> {
        let name = env!("CARGO_PKG_NAME");
        let authors = env!("CARGO_PKG_AUTHORS");
        let version = env!("CARGO_PKG_VERSION");
        println!("id name {} {}", name, version);
        println!("id author {}", authors);
        println!("uciok");

        None
    }

    pub fn isready(&mut self) -> Option<Cmd> {
        println!("readyok");

        None
    }

    pub fn position(&mut self, command: Vec<&str>) -> Option<Cmd> {
        let mut i = 1;

        let new_game = self.current_board.is_none();

        println!("info string New Game: {}", new_game);

        if new_game {
            if command[i] == "startpos" {

                let eng = Engine::new(); //replace with ref or something :(

                self.current_board = Some(Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &eng));

                i += 1;
            } else {
                let mut fen_tokens = vec![];


                //collect all of fen string
                while i < command.len() {
                    if command[i] == "moves" {
                        break;
                    } else {
                        fen_tokens.push(command[i]);
                        i += 1;
                    }
                }

                let eng = Engine::new(); //replace with ref or something :(

                self.current_board = Some(Board::new(&fen_tokens.join(" "), &eng));
            }


            if i < command.len() && command[i] == "moves" {
                i += 1;
                while i < command.len() {
                    println!("info string move: {} ", command[i]);
                    println!("info bb string {:?} {}", self.current_board?.turn, command[i]);
                    self.read_move(&command[i]);
                    println!("info aa string {:?} {}", self.current_board?.turn, command[i]);
                    i += 1;
                }
            }
        } else {
            if command[i] == "startpos" {
                i += 1;

                println!("info string {} < {} | {}", i, command.len(), command[i]);
                //grab last move made
                if i < command.len() && command[i] == "moves" {
                    i += 1;
                    let mut all_moves = vec![];
                    while i < command.len() {
                       println!("info string b: {:?} {}", self.current_board?.turn, command[i]);
                        all_moves.push(&command[i]);
                        println!("info string a: {:?} {}", self.current_board?.turn, command[i]);
                        i += 1;
                        println!("info string i: {} {:?}", i, all_moves);
                    }
                    println!("info string {:?}", all_moves[all_moves.len()-1]);
                    self.read_move(&all_moves[all_moves.len()-1]);
                }
            }
        }

        //Go Though moves
        //TODO Only need last move unless ucinewgame


        println!("info string {:?}", self.current_board?.turn);

        Some(Cmd::Set(self.current_board.unwrap()))
    }

    fn read_move(&mut self, str: &str) {
        println!("info string read_move");
        if str == "O-O" {
            let (_, _, board) = self.current_board.unwrap().castle(80);
            self.current_board = Some(board);
        } else if str == "O-O-O" {
            let (_, _, board) = self.current_board.unwrap().castle(88);
            self.current_board = Some(board);
        } else {
            let from = Board::lan_to_pos(&str[0..2]);
            let to = Board::lan_to_pos(&str[2..4]);
            println!("info string here");
            self.current_board = Some(self.current_board.unwrap().move_piece(to, from));
        }
    }
    /*

        **CALSING MAY NEED WORK <King From><King To>

        <move descriptor> ::= <from square><to square>[<promoted to>]
    <square>        ::= <file letter><rank number>
    <file letter>   ::= 'a'|'b'|'c'|'d'|'e'|'f'|'g'|'h'
    <rank number>   ::= '1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'
    <promoted to>   ::= 'q'|'r'|'b'|'n'
         */

    pub fn go(&mut self) -> Option<Cmd> {

        Some(Cmd::GoInf)
    }
    pub fn uci_new_game(&mut self) -> Option<Cmd> {
        let eng = Engine::new(); //replace with ref or something :(

        self.current_board = None;//Some(Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &eng));

        None
    }

    pub fn stop(&mut self) -> Option<Cmd> {

        Some(Cmd::Stop)
    }

    pub fn quit(&mut self) -> Option<Cmd> {
        std::process::exit(0);
    }

    pub fn set_option(&mut self) -> Option<Cmd> {

        None
    }



}