use std::sync::Arc;
use std::sync::Mutex;
use crate::chicory::board::Board;
use crate::chicory::engine::Engine;

#[derive(Debug, Clone, PartialEq)]
pub enum Cmd {
    Set(Board),
    Stop,
    GoInf
}

pub struct UciInterface {
    pub current_board: Arc<Mutex<Option<Board>>>,
    //current_move: usize,
    pub search_depth: usize,
    engine: Arc<Engine>,
}
impl UciInterface {

    pub fn new(engine: Arc<Engine>) -> Self {
        UciInterface {
            current_board: Arc::new(Mutex::new(None)),
            search_depth: 4,
            engine: engine
        }
    }

    pub fn uci(&mut self) -> Option<Cmd> {
        let name = env!("CARGO_PKG_NAME");
        let authors = env!("CARGO_PKG_AUTHORS");
        let version = env!("CARGO_PKG_VERSION");
        println!("id name {} {}", name, version);
        println!("id author {}", authors);
        println!("option name SearchDepth type spin default 4 min 1 max 99");
        println!("uciok");

        None
    }

    pub fn isready(&mut self) -> Option<Cmd> {
        println!("readyok");

        None
    }

    pub fn position(&mut self, command: Vec<&str>) -> Option<Cmd> {
        let mut i = 1;

        let mut cur_board = self.current_board.lock().unwrap().clone();

        let new_game = cur_board.is_none();

        println!("info string New Game: {}", new_game);


        println!("info string =========================== {:?}", self.current_board);
        if new_game {
            if command[i] == "startpos" {

                cur_board = Some(Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &self.engine));

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

                cur_board = Some(Board::new(&fen_tokens.join(" "), &eng));
            }


            if i < command.len() && command[i] == "moves" {
                i += 1;
                while i < command.len() {
                    //println!("info string move: {} ", command[i]);
                    //println!("info bb string {:?} {}", self.current_board?.turn, command[i]);
                    self.read_move(&command[i]);
                    //println!("info aa string {:?} {}", self.current_board?.turn, command[i]);
                    i += 1;
                }
            }
        } else {
            if command[i] == "startpos" {
                i += 1;

                //let eng = Engine::new(); //replace with ref or something :(

                //cur_board = Some(Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &self.engine));

                //println!("info string {} < {} | {}", i, command.len(), command[i]);
                //grab last move made
                if i < command.len() && command[i] == "moves" {
                    i += 1;
                    let mut all_moves = vec![];
                    while i < command.len() {
                       //println!("info string b: {:?} {}", self.current_board?.turn, command[i]);
                        all_moves.push(&command[i]);
                        //self.read_move(&command[i]);
                        //println!("info string a: {:?} {}", self.current_board?.turn, command[i]);
                        i += 1;
                        //println!("info string i: {} {:?}", i, all_moves);
                    }
                    //println!("info string {:?}", all_moves[all_moves.len()-1]);
                    self.read_move(&all_moves[all_moves.len()-1]);

                    cur_board = Some(cur_board.unwrap().make_move(&all_moves[all_moves.len()-1]));
                }
            }
        }

        //Go Though moves
        //TODO Only need last move unless ucinewgame


        //println!("info string {:?}", self.current_board?.turn);

        *self.current_board.lock().unwrap() = cur_board;


        println!("info string AFTER =========================== {:?}", self.current_board);

        Some(Cmd::Set(cur_board.unwrap()))
    }

    fn read_move(&mut self, str: &str) {
        //println!("info string read_move");
        let old_board = *self.current_board.lock().unwrap();

        if str == "O-O" {
            *self.current_board.lock().unwrap() = Some(old_board.unwrap().castle(80));
        } else if str == "O-O-O" {
            *self.current_board.lock().unwrap() = Some(old_board.unwrap().castle(88));
        } else {
            let from = Board::lan_to_pos(&str[0..2]);
            let to = Board::lan_to_pos(&str[2..4]);
            //println!("info string here");
            *self.current_board.lock().unwrap() = Some(old_board.unwrap().move_piece(to, from));
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
        //let eng = Engine::new(); //replace with ref or something :(

        *self.current_board.lock().unwrap() = None;
        //self.current_board = None;//Some(Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &eng));

        None
    }

    pub fn stop(&mut self) -> Option<Cmd> {

        Some(Cmd::Stop)
    }

    pub fn quit(&mut self) -> Option<Cmd> {
        std::process::exit(0);
    }

    pub fn set_option(&mut self, command: Vec<&str>) -> Option<Cmd> {
        if command[1] == "name" {
            match command[2] {
                "SearchDepth" => {
                    if command[3] == "value" {
                        self.search_depth = command[4].parse().unwrap();
                    }
                },
                _ => {}
            }
        }
        None
    }

}
