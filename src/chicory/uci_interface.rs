use crate::chicory::board::Board;
use crate::chicory::engine::Engine;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Debug, Clone, PartialEq)]
pub enum Cmd {
    Set(Board),
    Stop,
    GoInf,
    Perft(usize),
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
            engine: engine,
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

        if new_game {
            if command[i] == "startpos" {
                cur_board = Some(Board::new(
                    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                    &self.engine,
                ));

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

                cur_board = Some(Board::new(&fen_tokens.join(" "), &self.engine));
            }

            if i < command.len() && command[i] == "moves" {
                i += 1;
                while i < command.len() {
                    self.read_move(&command[i]);

                    i += 1;
                }
            }
        } else {
            if command[i] == "startpos" {
                i += 1;

                if i < command.len() && command[i] == "moves" {
                    i += 1;
                    let mut all_moves = vec![];
                    while i < command.len() {
                        all_moves.push(&command[i]);
                        i += 1;
                    }

                    self.read_move(&all_moves[all_moves.len() - 1]);

                    cur_board = Some(
                        cur_board
                            .unwrap()
                            .make_move(&all_moves[all_moves.len() - 1]),
                    );
                }
            }
        }

        *self.current_board.lock().unwrap() = cur_board;

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

            *self.current_board.lock().unwrap() = Some(old_board.unwrap().move_piece(to, from));
        }
    }

    pub fn go(&mut self) -> Option<Cmd> {
        Some(Cmd::GoInf)
    }
    pub fn uci_new_game(&mut self) -> Option<Cmd> {
        *self.current_board.lock().unwrap() = None;

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
                }
                _ => {}
            }
        }
        None
    }

    pub fn perft(&mut self, command: Vec<&str>) -> Option<Cmd> {
        let depth = command[1].parse::<usize>().unwrap();

        Some(Cmd::Perft(depth))
    }
}
