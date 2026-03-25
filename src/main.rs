mod chicory;

use board::*;
use chicory::*;
use engine::*;
use eval::minmax;
use uci_interface::*;
use std::io::{stdin, stdout, Write};
use std::{thread};
//use std::time::{Instant};
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;
use chicory::perft::multi_perft;
use chicory::eval::eval;
use crate::chicory::perft::multi_perft_list;

fn main() {
    //let (tx, rx): (Sender<Cmd>, Receiver<Cmd>) = mpsc::channel();

    let engine = Arc::new(Engine::new()); //replace with ref or something :(


    //let best_move = Arc::new(Mutex::new((0, 0, Board::new("8/8/8/8/8/8/8/8 w - - 0 1", &eng), None)));

    //TODO lock best move on search start and release after stop CMD

    let stop_calculation = Arc::new(AtomicBool::new(false));
    let finished_calculation = Arc::new(AtomicBool::new(false));

    let mut interface = UciInterface::new(engine.clone());

    //let main_interface_board = interface.current_board.lock().unwrap();

    loop {
        let _=stdout().flush();
        let mut input=String::new();
        stdin().read_line(&mut input).unwrap();
        let commands: Vec<&str> = input.trim_end().split(" ").collect();

        let cmd_out = match commands[0] {
            "uci" => interface.uci(),
            "isready" => interface.isready(),
            "position" => interface.position(commands),
            "go" => interface.go(),
            "ucinewgame" => interface.uci_new_game(),
            "stop" => interface.stop(),
            "quit" => interface.quit(),
            "setoption" => interface.set_option(commands),
            "perft" => interface.perft(commands),
            _ => None

        };


        if cmd_out.is_some() {

            match cmd_out.unwrap() {
                Cmd::GoInf => {
                    let stop_calculation_clone = Arc::clone(&stop_calculation);
                    let finished_calculation_clone = Arc::clone(&finished_calculation);

                    //let cal_board = interface.current_board.clone();
                    //println!("info string cal_board = {:?}", cal_board);

                    let eng = engine.clone();

                    let board_ref = interface.current_board.clone();

                    //let cal_board = interface.return_current_board().unwrap();//interface.current_board.lock().unwrap().clone().unwrap();
                    //

                
                    thread::spawn(move || {

                        let board = {
                            board_ref.lock().unwrap().clone()
                        };
                        let cal_board = board.unwrap();

                        println!("info string BOARD ~~~~~~~~~~~~~~~~~~~~ {:?}", cal_board);

                        let mut cur_best_move: Option<Move> = None;
                        let mut self_stop = false;

                        //let engine = Engine::new();

                        //while !stop_calculation_clone.load(Ordering::Relaxed) && !self_stop {

                            //cur_best_move = Some(new_move[0]);
                            //
                            //
                            let (_, best_move, _) = minmax(&eng, cal_board, interface.search_depth, i32::MIN, i32::MAX, cal_board.turn, 1, &stop_calculation_clone, true);
                            //println!("info score cp {}", eval(&best_move.unwrap().2, true));
                            cur_best_move = best_move;
                            self_stop = true;
                        //}

                        finished_calculation_clone.store(true, Ordering::Relaxed);

                        let (_, _, board, _) = cur_best_move.unwrap(); //*best_move_lock;
                        //println!("bestmove {}", Board::move_to_lan(&*best_move_lock));
                        println!("bestmove {}", Board::move_to_lan(&cur_best_move.unwrap()));
                        *board_ref.lock().unwrap() = Some(board);
                        
                        stop_calculation_clone.store(false, Ordering::Relaxed); 
                    });
                    //let start = Instant::now();

                    //while !finished_calculation.load(Ordering::Relaxed) {
                    //}


                    //if finished_calculation.load(Ordering::Relaxed) {

                    //}

                },
                Cmd::Set(board) => {
                    println!("info score cp {}", eval(&board, true));
                },

                Cmd::Stop => {
                    let stop_calculation_clone = Arc::clone(&stop_calculation);
                    stop_calculation.store(true, Ordering::Relaxed);
                },
                Cmd::Perft(depth) => {

                    let eng = engine.clone();

                    let board_ref = interface.current_board.clone();

                    thread::spawn(move || {

                        let board = {
                            board_ref.lock().unwrap().clone()
                        };
                        let cal_board = board.unwrap();

                        let time = Instant::now();
                        let (count, list) = multi_perft_list(&eng, cal_board, depth, 8);
                        let stop_time = time.elapsed().as_millis();

                        for (moves, nodes) in list {
                            println!("{}:  {}", moves, nodes);
                        }
                        println!("\n-----------------------------------\n");
                        println!("Total Nodes   : {}", count);
                        println!("Total Time    : {}", stop_time);
                        println!("Nodes per Sec : {},", (count as f64 / stop_time as f64) * 1000.0);
                    });
                }
                _ => {}
            }

       }
    }
}

/*

    (x >> n) & 1 gives you the nth bit,
    x & !(1 << n) clears the nth bit,
    and x | (1 << n) sets the nth bit.

*/

//https://shaack.com/projekte/cm-fen-editor/

//      󰡜 󰡗 󰡘 󰡙 󰡚 󰡛   NERD FONT
//

//TODO REVILED CALSING SQUARE BLOCK
//TODO MAKE CALSING INIT BETTER?
//CUSTOM COMPARE FUNCTION (IGNORE TEMP CASTLING STATE)
