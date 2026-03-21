use std::collections::HashMap;
use chicory::chicory::board::Board;
use chicory::chicory::engine::Engine;
use chicory::chicory::perft::*;

const THREAD_COUNT: usize = 8;

fn perft_from_string(eng: &Engine, board: Board, list:  String, depth: usize) -> bool {

    let lines: Vec<&str> = list.split("\n").collect();

    let mut expected: HashMap<String, usize> = HashMap::new();
    let mut results: HashMap<String, usize> = HashMap::new();

    for line in lines {
        let parts: Vec<&str> = line.split(": ").collect();
        expected.insert(parts[0].to_string(), parts[1].parse::<usize>().unwrap());
    }

    let moves = eng.gen_moves(board);
    for m in moves {
        let (_, _, new_board, _) = m;
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

    pass
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
    let board = Board::new("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1", &engine);
    let count = multi_perft(&engine, board, 4, THREAD_COUNT);

    assert_eq!(count, 4085603);
}

#[test]
fn full_hash() {

    //TODO FIX BLOCKED CASLING ATTACKS

    let input = String::from("a2a3: 94405
b2b3: 81066
g2g3: 77468
d5d6: 79551
a2a4: 90978
g2g4: 75677
g2h3: 82759
d5e6: 97464
c3b1: 84773
c3d1: 84782
c3a4: 91447
c3b5: 81498
e5d3: 77431
e5c4: 77752
e5g4: 79912
e5c6: 83885
e5g6: 83866
e5d7: 93913
e5f7: 88799
d2c1: 83037
d2e3: 90274
d2f4: 84869
d2g5: 87951
d2h6: 82323
e2d1: 74963
e2f1: 88728
e2d3: 85119
e2c4: 84835
e2b5: 79739
e2a6: 69334
a1b1: 83348
a1c1: 83263
a1d1: 79695
h1f1: 81563
h1g1: 84876
f3d3: 83727
f3e3: 92505
f3g3: 94461
f3h3: 98524
f3f4: 90488
f3g4: 92037
f3f5: 104992
f3h5: 95034
f3f6: 77838
e1d1: 79989
e1f1: 77887
e1g1: 86975
e1c1: 79803");

    let eng = Engine::new();
    let board = Board::new("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1", &eng);

    assert_eq!(perft_from_string(&eng, board, input, 4), true);
}