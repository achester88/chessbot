use std::collections::HashMap;
use chessbot::chessbot::board::Board;
use chessbot::chessbot::engine::Engine;

fn perft(eng: &Engine, board: Board, depth: usize) -> usize {
    //println!("--------------------[ Perft depth: {} ]---------------------", depth);
    let mut count = 0;

    if depth == 0 {
        return 1;
    }

    let moves = eng.gen_moves(board);
    //println!("moves count: {:?}", moves);
    for (from, to, new_board) in moves {
        //new_board.print_board();
        //println!("{}", Board::move_to_lan(&(from, to, new_board)));
        count += perft(&eng, new_board, depth - 1);
    }

    return count;
}

fn perft_from_string(fen: &str, list:  String) -> bool {

    let eng = Engine::new();
    let board = Board::new(fen, &eng);

    let lines: Vec<&str> = list.split("\n").collect();

    let mut expected: HashMap<String, usize> = HashMap::new();
    let mut results: HashMap<String, usize> = HashMap::new();

    for line in lines {
        let parts: Vec<&str> = line.split(": ").collect();
        //println!("|{} {}|", parts[0], parts[1]);
        expected.insert(parts[0].to_string(), parts[1].parse::<usize>().unwrap());
    }

    //let mut count = 0;

    let moves = eng.gen_moves(board);
    //println!("moves count: {:?}", moves);
    for m in moves {
        let (from, to, new_board) = m;
        //new_board.print_board();
        //count += perft(&eng, new_board, 1);
        results.insert(Board::move_to_lan(&m) , perft(&eng, new_board, 1));
    }

    let mut pass = true;

    for key in results.keys() {
        let result = results.get(key).unwrap();

        match expected.get(key) {
            Some(x) => {
                if result == x {
                    println!("{} | ==", key);
                } else {
                    pass = false;
                    println!("{} | E: {}, R: {}", key, x, result);
                }
                expected.remove(key);
            },
            None => {
                pass = false;
                println!("{} | Additional", key);
            }
        }

    }

    for key in expected.keys() {
        pass = false;
        print!("{} | Missing ", key);
    }

    return pass;
}

#[test]
fn ip_one() {
    let engine = Engine::new();
    let board = Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &engine);
    let count = perft(&engine, board, 1);

    assert_eq!(count, 20);
}

#[test]
fn ip_two() {
    let engine = Engine::new();
    let board = Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &engine);
    let count = perft(&engine, board, 2);

    assert_eq!(count, 400);
}

#[test]
fn ip_three() {
    let engine = Engine::new();
    let board = Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &engine);
    let count = perft(&engine, board, 3);

    assert_eq!(count, 8902);
}

#[test]
fn ip_four() {
    let engine = Engine::new();
    let board = Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &engine);
    let count = perft(&engine, board, 4);

    assert_eq!(count, 197281);
}

#[test]
fn ip_five() {
    let engine = Engine::new();
    let board = Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &engine);
    let count = perft(&engine, board, 5);

    assert_eq!(count, 4865609);
}

#[test]
fn p2_one() {
    let engine = Engine::new();
    let board = Board::new("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 1 1", &engine);
    let count = perft(&engine, board, 1);

    assert_eq!(count, 48);
}

#[test]
fn p2_two() {
    let engine = Engine::new();
    let board = Board::new("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 1 1", &engine);
    let count = perft(&engine, board, 2);

    assert_eq!(count, 2039);
}

#[test]
fn p2_three() {
    let engine = Engine::new();
    let board = Board::new("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 1 1", &engine);
    let count = perft(&engine, board, 3);

    assert_eq!(count, 97862);
}

#[test]
fn p3_one() {
    let engine = Engine::new();
    let board = Board::new("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", &engine);
    let count = perft(&engine, board, 1);

    assert_eq!(count, 14);
}

#[test]
fn p3_two() {
    let engine = Engine::new();
    let board = Board::new("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", &engine);
    let count = perft(&engine, board, 2);

    assert_eq!(count, 191);
}

#[test]
fn p3_three() { //E.P?
    let engine = Engine::new();
    let board = Board::new("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", &engine);
    let count = perft(&engine, board, 3);

    assert_eq!(count, 2812);
}

#[test]
fn p4_one() {
    let engine = Engine::new();
    let board = Board::new("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1", &engine);
    let count = perft(&engine, board, 1);

    assert_eq!(count, 6);
}

#[test]
fn p4_two() {
    let engine = Engine::new();
    let board = Board::new("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1", &engine);
    let count = perft(&engine, board, 2);

    assert_eq!(count, 264);
}

#[test]
fn p4_three() {
    let engine = Engine::new();
    let board = Board::new("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1", &engine);
    let count = perft(&engine, board, 3);

    assert_eq!(count, 9467);
}

#[test]
fn p5_one() {
    let engine = Engine::new();
    let board = Board::new("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8", &engine);
    let count = perft(&engine, board, 1);

    assert_eq!(count, 44);
}

#[test]
fn p5_two() {
    let engine = Engine::new();
    let board = Board::new("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8", &engine);
    let count = perft(&engine, board, 2);

    assert_eq!(count, 1486);
}

#[test]
fn p5_three() {
    let engine = Engine::new();
    let board = Board::new("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8", &engine);
    let count = perft(&engine, board, 3);

    assert_eq!(count, 62379);
}

#[test]
fn p6_one() {
    let engine = Engine::new();
    let board = Board::new("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10", &engine);
    let count = perft(&engine, board, 1);

    assert_eq!(count, 46);
}

#[test]
fn p6_two() {
    let engine = Engine::new();
    let board = Board::new("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10", &engine);
    let count = perft(&engine, board, 2);

    assert_eq!(count, 2079);
}

#[test]
fn p6_three() {
    let engine = Engine::new();
    let board = Board::new("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10", &engine);
    let count = perft(&engine, board, 3);

    assert_eq!(count, 89890);
}

#[test]
fn p6_four() {
    let engine = Engine::new();
    let board = Board::new("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10", &engine);
    let count = perft(&engine, board, 4);

    assert_eq!(count, 3894594);
}

#[test]
fn custom() {
    let engine = Engine::new();
    let board = Board::new("r3k2r/8/8/8/8/8/8/1R2K2R w Kkq - 0 1", &engine);
    let count = perft(&engine, board, 2);

    assert_eq!(count, 567);
}

#[test]
fn custom_c() {
    let engine = Engine::new();
    let board = Board::new("r3k2r/8/8/8/8/8/8/1R2K2R w Kkq - 0 1", &engine);

    let new_board = board.move_lan(&engine, "a1b1");
    let count = perft(&engine, board, 1);

    assert_eq!(perft(&engine, board.move_lan(&engine, "b1a1"), 1), 26);
    assert_eq!(perft(&engine, board.move_lan(&engine, "b1c1"), 1), 25);
    assert_eq!(perft(&engine, board.move_lan(&engine, "b1d1"), 1), 23);
    assert_eq!(perft(&engine, board.move_lan(&engine, "b1b2"), 1), 26);
    assert_eq!(perft(&engine, board.move_lan(&engine, "b1b3"), 1), 26);
    assert_eq!(perft(&engine, board.move_lan(&engine, "b1b4"), 1), 26);
    assert_eq!(perft(&engine, board.move_lan(&engine, "b1b5"), 1), 26);
    assert_eq!(perft(&engine, board.move_lan(&engine, "b1b6"), 1), 26);
    assert_eq!(perft(&engine, board.move_lan(&engine, "b1b7"), 1), 23);
    assert_eq!(perft(&engine, board.move_lan(&engine, "b1b8"), 1), 4);

    assert_eq!(perft(&engine, board.move_lan(&engine, "h1f1"), 1), 23);
    assert_eq!(perft(&engine, board.move_lan(&engine, "h1g1"), 1), 25);
    assert_eq!(perft(&engine, board.move_lan(&engine, "h1h2"), 1), 25);
    assert_eq!(perft(&engine, board.move_lan(&engine, "h1h3"), 1), 24);
    assert_eq!(perft(&engine, board.move_lan(&engine, "h1h4"), 1), 23);
    assert_eq!(perft(&engine, board.move_lan(&engine, "h1h5"), 1), 22);
    assert_eq!(perft(&engine, board.move_lan(&engine, "h1h6"), 1), 21);
    assert_eq!(perft(&engine, board.move_lan(&engine, "h1h7"), 1), 17);
    assert_eq!(perft(&engine, board.move_lan(&engine, "h1h8"), 1), 3);

    assert_eq!(perft(&engine, board.move_lan(&engine, "e1d1"), 1), 26);
    assert_eq!(perft(&engine, board.move_lan(&engine, "e1f1"), 1), 26);
    assert_eq!(perft(&engine, board.move_lan(&engine, "e1d2"), 1), 26);
    assert_eq!(perft(&engine, board.move_lan(&engine, "e1e2"), 1), 26);
    assert_eq!(perft(&engine, board.move_lan(&engine, "e1f2"), 1), 26);
    assert_eq!(perft(&engine, board.move_lan(&engine, "e1g1"), 1), 23);
    //let (_, _, cm) = board.castle(88);
    //assert_eq!(perft(&engine, cm, 1), 22);
}

#[test]
fn custom_c_1() {
    let engine = Engine::new();
    let board = Board::new("r3k2r/8/8/8/8/8/8/1R2K2R w Kkq - 0 1", &engine);

    let moves = engine.gen_moves(board);

    println!("{:?}", moves[0]);

    let (from, to, initial_board) = moves[0];

    println!("---------------------------------------------------------");
    let fin_moves = engine.gen_moves(initial_board);
    println!("---------------------------------------------------------");

    for m in &fin_moves {
        println!("---{}---", Board::move_to_lan(m));
        m.2.print_board();
        println!("\n\n\n\n");
    }

    println!("-----------");
    assert_eq!(fin_moves.len(), 23);


}

#[test]
fn full_hash() {

    let input = String::from("b1a1: 26
b1c1: 25
b1d1: 23
b1b2: 26
b1b3: 26
b1b4: 26
b1b5: 26
b1b6: 26
b1b7: 23
b1b8: 4
h1f1: 23
h1g1: 25
h1h2: 25
h1h3: 24
h1h4: 23
h1h5: 22
h1h6: 21
h1h7: 17
h1h8: 3
e1d1: 26
e1f1: 26
e1d2: 26
e1e2: 26
e1f2: 26
e1g1: 23");

    assert_eq!(perft_from_string("r3k2r/8/8/8/8/8/8/1R2K2R w Kkq - 0 1", input), true);
}

//Rook Capture, respawn when calsing

/*
#[test]
fn p6_five() {
    let engine = Engine::new();
    let board = Board::new("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10", &engine);
    let count = perft(board, 5);

    assert_eq!(count, 164075551);
}
*/
//https://www.chessprogramming.org/Perft_Results
//http://www.rocechess.ch/perft.html