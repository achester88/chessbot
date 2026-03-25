mod chicory;

use crate::chicory::perft::multi_perft_list;
use board::*;
use chicory::eval::eval;
use chicory::*;
use engine::*;
use eval::minmax;
use std::io::{stdin, stdout, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Instant;
use uci_interface::*;

fn main() {
    let engine = Arc::new(Engine::new()); //replace with ref or something :(
    let stop_calculation = Arc::new(AtomicBool::new(false));
    let finished_calculation = Arc::new(AtomicBool::new(false));

    let mut interface = UciInterface::new(engine.clone());

    loop {
        let _ = stdout().flush();
        let mut input = String::new();
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
            _ => None,
        };

        if cmd_out.is_some() {
            match cmd_out.unwrap() {
                Cmd::GoInf => {
                    let stop_calculation_clone = Arc::clone(&stop_calculation);
                    let finished_calculation_clone = Arc::clone(&finished_calculation);
                    let eng = engine.clone();
                    let board_ref = interface.current_board.clone();

                    thread::spawn(move || {
                        let board = { board_ref.lock().unwrap().clone() };
                        let cal_board = board.unwrap();

                        let (_, best_move, _) = minmax(
                            &eng,
                            cal_board,
                            interface.search_depth,
                            i32::MIN,
                            i32::MAX,
                            cal_board.turn,
                            1,
                            &stop_calculation_clone,
                            true,
                        );

                        finished_calculation_clone.store(true, Ordering::Relaxed);

                        let (_, _, board, _) = best_move.unwrap(); //*best_move_lock;
                        println!("bestmove {}", Board::move_to_lan(&best_move.unwrap()));
                        *board_ref.lock().unwrap() = Some(board);

                        stop_calculation_clone.store(false, Ordering::Relaxed);
                    });
                }
                Cmd::Set(board) => {
                    println!("info score cp {}", eval(&board));
                }

                Cmd::Stop => {
                    stop_calculation.store(true, Ordering::Relaxed);
                }
                Cmd::Perft(depth) => {
                    let eng = engine.clone();

                    let board_ref = interface.current_board.clone();

                    thread::spawn(move || {
                        let board = { board_ref.lock().unwrap().clone() };
                        let cal_board = board.unwrap();

                        let time = Instant::now();
                        let (count, list) = multi_perft_list(&eng, cal_board, depth, 8);
                        let stop_time = time.elapsed().as_millis();

                        for (moves, nodes) in list {
                            println!("{}:  {}", moves, nodes);
                        }
                        println!("\n-----------------------------------\n");
                        println!("Total Nodes   : {}", count);
                        println!("Total Time    : {}(s)", stop_time / 1000);
                        println!(
                            "Nodes per Sec : {},",
                            (count as f64 / stop_time as f64) * 1000.0
                        );
                    });
                }
            }
        }
    }
}
