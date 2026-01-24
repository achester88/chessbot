use std::process::Command;
use crate::chessbot::board::Board;
use crate::chessbot::engine::Engine;
use crate::chessbot::uci_interface::Cmd::Set;

#[derive(Debug, Clone, PartialEq)]
pub enum Cmd {
    Set(Board),
    Stop,
    Go
}

pub struct UciInterface {
    pub current_board: Option<Board>
}
impl UciInterface {

    pub fn new() -> Self {
        UciInterface {
            current_board: None
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

        if command[i] == "startpos" {
            if self.current_board.is_none() {

                let eng = Engine::new(); //replace with ref or something :(

                self.current_board = Some(Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &eng));

            }
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

        //Go Though moves
        //TODO Only need last move unless ucinewgame
        if i < command.len() && command[i] == "moves" {
            i += 1;
            while i < command.len() {
                if command[i] == "O-O" {
                    let (_, _, board) = self.current_board.unwrap().castle(80);
                    self.current_board = Some(board);
                } else if command[i] == "O-O-O" {
                    let (_, _, board) = self.current_board.unwrap().castle(88);
                    self.current_board = Some(board);
                } else {
                    let from = Board::lan_to_pos(&command[i][0..2]);
                    let to = Board::lan_to_pos(&command[i][2..4]);
                    self.current_board = Some(self.current_board.unwrap().move_piece(to, from));
                    i += 1;
                }
            }
        }

        println!("info string {:?}", self.current_board?.turn);

        Some(Cmd::Set(self.current_board.unwrap()))
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

        Some(Cmd::Go)
    }
    pub fn uci_new_game(&mut self) -> Option<Cmd> {
        let eng = Engine::new(); //replace with ref or something :(

        self.current_board = Some(Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &eng));

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