use std::collections::HashMap;
use chessbot::chessbot::board::Board;
use chessbot::chessbot::engine::Engine;
use chessbot::chessbot::perft::*;

const THREAD_COUNT: usize = 8;

fn perft_from_string(eng: &Engine, board: Board, list:  String, depth: usize) -> bool {

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
        results.insert(Board::move_to_lan(&m) , multi_perft(&eng, new_board, depth-1, THREAD_COUNT));
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
    let count = multi_perft(&engine, board, 1, THREAD_COUNT);

    assert_eq!(count, 20);
}

#[test]
fn ip_two() {
    let engine = Engine::new();
    let board = Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &engine);
    let count = multi_perft(&engine, board, 2, THREAD_COUNT);

    assert_eq!(count, 400);
}

#[test]
fn ip_three() {
    let engine = Engine::new();
    let board = Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &engine);
    let count = multi_perft(&engine, board, 3, THREAD_COUNT);

    assert_eq!(count, 8902);
}

#[test]
fn ip_four() {
    let engine = Engine::new();
    let board = Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &engine);
    let count = multi_perft(&engine, board, 4, THREAD_COUNT);

    assert_eq!(count, 197281);
}

#[test]
fn ip_five() {
    let engine = Engine::new();
    let board = Board::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &engine);
    let count = multi_perft(&engine, board, 5, THREAD_COUNT);

    assert_eq!(count, 4865609);
}

#[test]
fn p2_one() {
    let engine = Engine::new();
    let board = Board::new("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 1 1", &engine);
    let count = multi_perft(&engine, board, 1, THREAD_COUNT);

    assert_eq!(count, 48);
}

#[test]
fn p2_two() {
    let engine = Engine::new();
    let board = Board::new("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 1 1", &engine);
    let count = multi_perft(&engine, board, 2, THREAD_COUNT);

    assert_eq!(count, 2039);
}

#[test]
fn p2_three() {
    let engine = Engine::new();
    let board = Board::new("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 1 1", &engine);
    let count = multi_perft(&engine, board, 3, THREAD_COUNT);

    assert_eq!(count, 97862);
}

#[test]
fn p3_one() {
    let engine = Engine::new();
    let board = Board::new("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", &engine);
    let count = multi_perft(&engine, board, 1, THREAD_COUNT);

    assert_eq!(count, 14);
}

#[test]
fn p3_two() {
    let engine = Engine::new();
    let board = Board::new("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", &engine);
    let count = multi_perft(&engine, board, 2, THREAD_COUNT);

    assert_eq!(count, 191);
}

#[test]
fn p3_three() { //E.P?
    let engine = Engine::new();
    let board = Board::new("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", &engine);
    let count = multi_perft(&engine, board, 3, THREAD_COUNT);

    assert_eq!(count, 2812);
}

#[test]
fn p4_one() {
    let engine = Engine::new();
    let board = Board::new("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1", &engine);
    let count = multi_perft(&engine, board, 1, THREAD_COUNT);

    assert_eq!(count, 6);
}

#[test]
fn p4_two() {
    let engine = Engine::new();
    let board = Board::new("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1", &engine);
    let count = multi_perft(&engine, board, 2, THREAD_COUNT);

    assert_eq!(count, 264);
}

#[test]
fn p4_three() {
    let engine = Engine::new();
    let board = Board::new("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1", &engine);
    let count = multi_perft(&engine, board, 3, THREAD_COUNT);

    assert_eq!(count, 9467);
}

#[test]
fn p5_one() {
    let engine = Engine::new();
    let board = Board::new("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8", &engine);
    let count = multi_perft(&engine, board, 1, THREAD_COUNT);

    assert_eq!(count, 44);
}

#[test]
fn p5_two() {
    let engine = Engine::new();
    let board = Board::new("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8", &engine);
    let count = multi_perft(&engine, board, 2, THREAD_COUNT);

    assert_eq!(count, 1486);
}

#[test]
fn p5_three() {
    let engine = Engine::new();
    let board = Board::new("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8", &engine);
    let count = multi_perft(&engine, board, 3, THREAD_COUNT);

    assert_eq!(count, 62379);
}

#[test]
fn p6_one() {
    let engine = Engine::new();
    let board = Board::new("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10", &engine);
    let count = multi_perft(&engine, board, 1, THREAD_COUNT);

    assert_eq!(count, 46);
}

#[test]
fn p6_two() {
    let engine = Engine::new();
    let board = Board::new("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10", &engine);
    let count = multi_perft(&engine, board, 2, THREAD_COUNT);

    assert_eq!(count, 2079);
}

#[test]
fn p6_three() {
    let engine = Engine::new();
    let board = Board::new("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10", &engine);
    let count = multi_perft(&engine, board, 3, THREAD_COUNT);

    assert_eq!(count, 89890);
}

#[test]
fn p6_four() {
    let engine = Engine::new();
    let board = Board::new("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10", &engine);
    let count = multi_perft(&engine, board, 4, THREAD_COUNT);

    assert_eq!(count, 3894594);
}

#[test]
fn custom() {
    let engine = Engine::new();
    let board = Board::new("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 1 1", &engine);
    let count = perft(&engine, board, 3);

    assert_eq!(count, 97862);
}

#[test]
fn full_hash() {

    //TODO FIX BLOCKED CASLING ATTACKS

    let input = String::from("a2a3: 2186
b2b3: 1964
g2g3: 1882
d5d6: 1991
a2a4: 2149
g2g4: 1843
g2h3: 1970
d5e6: 2241
c3b1: 2038
c3d1: 2040
c3a4: 2203
c3b5: 2138
e5d3: 1803
e5c4: 1880
e5g4: 1878
e5c6: 2027
e5g6: 1997
e5d7: 2124
e5f7: 2080
d2c1: 1963
d2e3: 2136
d2f4: 2000
d2g5: 2134
d2h6: 2019
e2d1: 1733
e2f1: 2060
e2d3: 2050
e2c4: 2082
e2b5: 2057
e2a6: 1907
a1b1: 1969
a1c1: 1968
a1d1: 1885
h1f1: 1929
h1g1: 2013
f3d3: 2005
f3e3: 2174
f3g3: 2214
f3h3: 2360
f3f4: 2132
f3g4: 2169
f3f5: 2396
f3h5: 2267
f3f6: 2111
e1d1: 1894
e1f1: 1855
e1g1: 2059
e1c1: 1887");

    let eng = Engine::new();
    let board = Board::new("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 1 1", &eng);

    assert_eq!(perft_from_string(&eng, board, input, 3), true);
}

#[test]
fn full_hash_2() {

    let input = String::from("b4b3: 40
g6g5: 38
c7c6: 40
d7d6: 38
c7c5: 40
h3g2: 39
e6d5: 39
b4c3: 39
b6a4: 38
b6c4: 42
b6d5: 39
b6c8: 39
f6e4: 42
f6g4: 38
f6d5: 40
f6h5: 40
f6h7: 40
f6g8: 40
a6f1: 40
a6e2: 40
a6d3: 40
a6c4: 39
a6b5: 39
a6b7: 42
a6c8: 42
g7h6: 39
g7f8: 39
a8b8: 39
a8c8: 39
a8d8: 39
h8h4: 39
h8h5: 39
h8h6: 39
h8h7: 39
h8f8: 39
h8g8: 39
e7c5: 39
e7d6: 38
e7d8: 39
e7f8: 39
e8d8: 39
e8f8: 39
e8g8: 39
e8c8: 39");

    let eng = Engine::new();
    let board = Board::new("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPB1PPP/R2BK2R b KQkq - 1 1", &eng);
    //println!("{:08b}", board.casling);
    assert_eq!(perft_from_string(&eng, board, input, 2), true);
    //assert_eq!(false, true);
}

#[test]
fn full_hash_2_c() {
    let engine = Engine::new();
    let board = Board::new("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPB1PPP/R2BK2R b KQkq - 1 1", &engine);
    let count = perft(&engine, board, 2);

    assert_eq!(count, 1733);
}

#[test]
fn full_hash_1() {

    let input = String::from("a2a3: 1
b2b3: 1
g2g3: 1
d5d6: 1
a2a4: 1
g2g4: 1
g2h3: 1
d5e6: 1
c3b1: 1
c3e2: 1
c3a4: 1
c3b5: 1
e5d3: 1
e5c4: 1
e5g4: 1
e5c6: 1
e5g6: 1
e5d7: 1
e5f7: 1
d1e2: 1
d2c1: 1
d2e3: 1
d2f4: 1
d2g5: 1
d2h6: 1
a1b1: 1
a1c1: 1
h1f1: 1
h1g1: 1
f3e2: 1
f3d3: 1
f3e3: 1
f3g3: 1
f3h3: 1
f3f4: 1
f3g4: 1
f3f5: 1
f3h5: 1
f3f6: 1");

    let eng = Engine::new();
    let board = Board::new("r1n1k2r/p1ppqpb1/b3pnp1/3PN3/1p2P3/2N2Q1p/PPPB1PPP/R2BK2R w KQkq - 1 1", &eng);

    assert_eq!(perft_from_string(&eng, board, input, 1), true);
}

#[test]
fn full_hash_1_c() {
    let engine = Engine::new();
    let board = Board::new("r1n1k2r/p1ppqpb1/b3pnp1/3PN3/1p2P3/2N2Q1p/PPPB1PPP/R2BK2R w KQkq - 1 1", &engine);
    let count = perft(&engine, board, 1);

    assert_eq!(count, 39);
}

#[test]
fn custom_c() {
    let engine = Engine::new();
    let board_init =  Board::new("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 1 1", &engine);

    let moves = engine.gen_moves(board_init);

    let moves_2 = engine.gen_moves( moves[16].2);

    let board = moves_2[37].2;

    let mut i = 0;

    //while i < moves_2.len() {
    //    println!("{}: {}", i, Board::move_to_lan(&moves_2[i]));
    //    i += 1;
    //}

    println!("{:?}", moves_2[37]);

    println!("{}", Board::move_to_lan(&moves[37]));

    let input = String::from("a2a3: 1
b2b3: 1
g2g3: 1
d5d6: 1
a2a4: 1
g2g4: 1
g2h3: 1
d5e6: 1
c3b1: 1
c3e2: 1
c3a4: 1
c3b5: 1
e5d3: 1
e5c4: 1
e5g4: 1
e5c6: 1
e5g6: 1
e5d7: 1
e5f7: 1
d1e2: 1
d2c1: 1
d2e3: 1
d2f4: 1
d2g5: 1
d2h6: 1
a1b1: 1
a1c1: 1
h1f1: 1
h1g1: 1
f3e2: 1
f3d3: 1
f3e3: 1
f3g3: 1
f3h3: 1
f3f4: 1
f3g4: 1
f3f5: 1
f3h5: 1
f3f6: 1");

    let eng = Engine::new();

    assert_eq!(perft_from_string(&eng, board, input, 1), true);
    //assert_eq!(false, true);
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