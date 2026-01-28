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
    for (from, to, new_board, _) in moves {
        //new_board.print_board();
        //println!("{}", Board::move_to_lan(&(from, to, new_board)));
        count += perft(&eng, new_board, depth - 1);
    }

    return count;
}

fn perft_from_string(fen: &str, list:  String, depth: usize) -> bool {

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
        let (from, to, new_board, _) = m;
        //new_board.print_board();
        //count += perft(&eng, new_board, 1);
        results.insert(Board::move_to_lan(&m) , perft(&eng, new_board, depth-1));
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

    let moves = engine.gen_moves(board);

    println!("{:?}", moves[0]);

    let (from, to, initial_board, _) = moves[0];

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

    let input = String::from("c4c5: 1409
d2d4: 1643
f3d4: 1687
b4c5: 1352
f1f2: 1623
g1h1: 1753");

    assert_eq!(perft_from_string("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1", input, 3), true);
}

#[test]
fn full_hash_down() {

    let input = String::from("c7c6: 41
d7d6: 38
c7c5: 39
d7d5: 42
b2a1q: 39
b2a1r: 39
b2a1b: 39
b2a1n: 39
b2b1q: 39
b2b1r: 39
b2b1b: 39
b2b1n: 39
g7h6: 36
a5b3: 40
a5c4: 39
a5c6: 41
f6e4: 38
f6g4: 38
f6d5: 42
f6h5: 40
f6g8: 40
b6d4: 4
b6c5: 38
b6a7: 41
g6e4: 39
g6f5: 41
g6h5: 40
a8a7: 40
a8b8: 48
a8c8: 44
a8d8: 44
h8f8: 40
h8g8: 40
a3a2: 41
a3b3: 41
a3c3: 39
a3d3: 41
a3e3: 2
a3f3: 38
a3a4: 40
a3b4: 32
e8c8: 44
e8d8: 40");

    assert_eq!(perft_from_string("r3k2r/Pppp1ppp/1b3nbN/nP6/BBPPP3/q4N2/Pp4PP/R2Q1RK1 b kq - 0 1", input, 2), true);
}

#[test]
fn full_hash_down_c() {
    let engine = Engine::new();
    let board = Board::new("r3k2r/Pppp1ppp/1b3nbN/nP6/BBPPP3/q4N2/Pp4PP/R2Q1RK1 b kq - 0 1", &engine);
    let count = perft(&engine, board, 2);

    assert_eq!(count, 1643);
}

#[test]
fn full_hash_1() {

    let input = String::from("g2g3: 1
h2h3: 1
d4d5: 1
e4e5: 1
g2g4: 1
h2h4: 1
d4c5: 1
b5c6: 1
f3e1: 1
f3d2: 1
f3h4: 1
f3e5: 1
f3g5: 1
h6g4: 1
h6f5: 1
h6f7: 1
h6g8: 1
a4c2: 1
a4b3: 1
b4e1: 1
b4d2: 1
b4a3: 1
b4c3: 1
b4a5: 1
b4c5: 1
a1b1: 1
a1c1: 1
f1e1: 1
f1f2: 1
d1b1: 1
d1c1: 1
d1e1: 1
d1c2: 1
d1d2: 1
d1e2: 1
d1b3: 1
d1d3: 1
g1h1: 1
g1f2: 1");

    assert_eq!(perft_from_string("r3k2r/Pp1p1ppp/1b3nbN/nPp5/BBPPP3/q4N2/Pp4PP/R2Q1RK1 w kq c6 0 1", input, 1), true);
}

#[test]
fn full_hash_1_c() {
    let engine = Engine::new();
    let board = Board::new("r3k2r/Pp1p1ppp/1b3nbN/nPp5/BBPPP3/q4N2/Pp4PP/R2Q1RK1 w kq c6 0 1", &engine);
    let count = perft(&engine, board, 1);

    assert_eq!(count, 39);
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