use std::{collections::VecDeque, num, os::windows::process};

fn get_rock_types() -> Vec<Vec<Vec<u8>>> {
    vec![
        vec![vec![1, 1, 1, 1]],
        vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 1, 0]],
        vec![vec![0, 0, 1], vec![0, 0, 1], vec![1, 1, 1]], // y coordinate goes up like a regular graph, so here it's reversed
        vec![vec![1], vec![1], vec![1], vec![1]],
        vec![vec![1, 1], vec![1, 1]],
    ]
}

const ROCK_HEIGHTS: [i64; 5] = [1, 3, 3, 4, 2];

// already shifted into place
const ROCK_BITS: [u32; 5] = [
    0b00000000000000000000000000111100,
    0b00000000000010000001110000001000,
    0b00000000000100000001000000011100,
    0b00000100000001000000010000000100,
    0b00000000000000000000110000001100,
];

// computer did the math for me:
fn get_bit_rock_types() -> Vec<u32> {
    let mut bit_rocks: Vec<u32> = vec![];
    for rock in get_rock_types() {
        let mut bitfield = 0u32;
        for (y, row) in rock.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if *col == 1 {
                    bitfield = bitfield | (1 << (x + y * 8));
                    // means the rocks will look flipped when printing them out but for some reason I like the idea of numbering
                    // starting at 0
                    // it also means that < in the stream is >> and > in the stream is << - otherwise the L shaped one breaks
                }
            }
        }
        // Each rock appears so that its left edge is two units away from the left wall
        bitfield <<= 2;
        bit_rocks.push(bitfield);
        println!("rock {bitfield:b}");
    }

    bit_rocks
}

#[test]
fn test_get_bit_rock_types() {
    get_bit_rock_types();
}

type Board = VecDeque<Vec<u8>>;

fn main() {
    println!("Hello, world!");
    let input = get_puzzle_input();
    println!("Puzzle input len {}", input.len());
    let answer = do_the_thing(get_puzzle_input(), 1000000000000);
    println!("boom? {}", answer)
}

fn do_the_thing(input: &str, num_rocks: usize) -> i64 {
    let jet_streams = parse_input(input);

    bitwise_simulation(&jet_streams, num_rocks, &build_processed_rock_cache())
}

#[test]
fn test_do_part_1() {
    let answer = do_the_thing(get_puzzle_input(), 2022);
    assert_eq!(3109, answer);
}

// #[test]
// fn test_part_2_on_sample() {
//     let answer = do_the_thing(get_puzzle_input(), 1000000000000);
//     assert_eq!(1514285714288, answer);
// }

fn simulate(jet_streams: &Vec<i64>, num_rocks: usize) -> (Board, i64) {
    let mut board = VecDeque::from(vec![vec![0u8; 7]; 58]); // determined from unit testing that you need this for first 2022 results
    let mut stream_idx = 0;
    let mut bottom_level: i64 = 0;
    let mut highest_level: i64 = 0;
    for i in 0..num_rocks {
        if i % 100000 == 0 {
            println!(
                "Rocks dropped {i}: {}% done",
                100.0 * (i as f32) / (num_rocks as f32)
            );
        }
        stream_idx = drop_rock(
            &mut board,
            &mut bottom_level,
            &mut highest_level,
            i % get_rock_types().len(),
            jet_streams,
            stream_idx,
        );
        // if i % 10 == 0 {
        //     for row in &board {
        //         println!("{:?}", row);
        //     }
        //     println!();
        //     println!();
        // }
    }

    (board, bottom_level)
}

#[test]
fn test_sample_input() {
    let answer = do_the_thing(get_sample_input(), 2022);
    assert_eq!(3068, answer);
}

fn highest_level(board: &Board) -> usize {
    match board
        .iter()
        .position(|row| row.iter().find(|cell| **cell != 0).is_none())
    {
        Some(row_idx) => row_idx,
        None => {
            panic!();
        }
    }
}

#[test]
fn test_highest_level() {
    let mut board = VecDeque::from(vec![vec![0u8; 7]; 15]);
    assert_eq!(0, highest_level(&board));
    board[0][3] = 1u8;
    assert_eq!(1, highest_level(&board));
    for i in 0..11 {
        board[i][3] = 1u8;
    }
    assert_eq!(11, highest_level(&board));
}

// struct CachedResult {
//     let new_top = [u8;16];
//     let
// }

// returns the position in the jet_stream
fn drop_rock(
    board: &mut Board,
    bottom_level: &mut i64,
    highest_level: &mut i64,
    rock_type: usize,
    jet_streams: &[i64],
    mut jet_stream_idx: usize,
) -> usize {
    let rock = get_rock_types()[rock_type].clone();
    let mut rock_x = 2i64;
    // frex, highest level is row[0], rock is 1-3, +2 makes for 3 for flattie, 5 for square, (0-indexed means 3 empty rows between)
    let mut rock_y = *highest_level + rock.len() as i64 + 2; // origin is top left
    while rock_y - *bottom_level >= (board.len() - 1) as i64 {
        board.push_back(vec![0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8]);
        board.pop_front();
        *bottom_level += 1;
    }
    let mut collision = false;
    while !collision {
        // sideways
        let check_x = rock_x + jet_streams[jet_stream_idx % jet_streams.len()];
        jet_stream_idx += 1;
        let check_y = rock_y;
        let mut side_collision = false;
        for (y, row) in rock.iter().enumerate() {
            let pixel_y = (check_y - *bottom_level) as usize - y;
            for (x, pixel) in row.iter().enumerate() {
                let pixel_x = check_x as usize + x;
                if pixel_x as usize >= board[0].len() {
                    // unsigned automatically checks <0 case
                    side_collision = true;
                    break;
                }
                if *pixel != 0u8 {
                    if board[pixel_y][(pixel_x as usize)] != 0 {
                        side_collision = true;
                        break;
                    }
                }
            }
        }
        if !side_collision {
            rock_y = check_y;
            rock_x = check_x;
        }
        // and vertical
        let check_x = rock_x;
        let check_y = rock_y - 1;
        for (y, row) in rock.iter().enumerate() {
            if (check_y - y as i64) < 0 {
                collision = true;
                break;
            }
            let pixel_y = (check_y - *bottom_level) as usize - y;
            for (x, pixel) in row.iter().enumerate() {
                if *pixel != 0u8 {
                    if board[pixel_y][(check_x as usize + x)] != 0 {
                        collision = true;
                        break;
                    }
                }
            }
        }
        if !collision {
            rock_x = check_x;
            rock_y = check_y;
        }
    }
    // and place permanently
    for (y, row) in rock.iter().enumerate() {
        for (x, pixel) in row.iter().enumerate() {
            board[(rock_y - *bottom_level) as usize - y][rock_x as usize + x] |= rock[y][x]; // frex, rock_y -> 3 ; rock row 0 goes on 3, rock row 1 goes on 2, rock row 2 goes on 1
            *highest_level = std::cmp::max(*highest_level, rock_y - y as i64 + 1);
        }
    }

    jet_stream_idx
}

const BUFFER_SIZE: usize = 1048576;

fn build_processed_rock_cache() -> [[[[[u32;2];2];2];2];5] {
    let mut processed_rock_cache = [[[[[0;2];2];2];2];5];
    for i in 0..5 {
        for stream0 in 0..2 { // 0 is left, 1 is right (in puzzle space - it means bit shifting right and left the way I implemented)
            for stream1 in 0..2 {
                for stream2 in 0..2 {
                    for stream3 in 0..2 {
                        let mut rock = ROCK_BITS[i];
                        let streams = [stream0,stream1,stream2,stream3];
                        for stream_shift in streams {
                            if stream_shift == 1 {
                                // shift "left" for the game is "right" for our representation
                                // don't shift if it'll go out of bounds; test if bits are in last column (6 - 7 is always empty))
                                if (rock & 0b01000000010000000100000001000000) != 0 {
                                    // don't shift
                                } else {
                                    rock = rock << 1;
                                }
                            } else {
                                if (rock & 0b00000001000000010000000100000001) != 0 {
                                    // don't shift
                                } else {
                                    rock = rock >> 1;
                                }
                            }
                        }
                        assert_ne!(0, rock);
                        processed_rock_cache[i][stream0][stream1][stream2][stream3] = rock;
                    }
                }
            }
        }
    }

    processed_rock_cache
}

#[test]
fn test_build_processed_rock_cache() {
    let processed_rock_cache = build_processed_rock_cache();
    // spot checks
    assert_eq!(processed_rock_cache[0][0][0][0][0], 0x0000000fu32);
    assert_eq!(processed_rock_cache[0][1][1][1][1], 0b00000000000000000000000001111000);
    assert_eq!(processed_rock_cache[0][0][1][0][1], 0b00000000000000000000000000111100);
    assert_eq!(processed_rock_cache[1][0][1][0][1], 0b00000000000010000001110000001000);
    assert_eq!(processed_rock_cache[2][1][0][1][0], 0b00000000000100000001000000011100);
    assert_eq!(processed_rock_cache[3][0][1][0][1], 0b00000100000001000000010000000100);
    assert_eq!(processed_rock_cache[4][1][0][1][0], 0b00000000000000000000110000001100);
    
}


fn bitwise_simulation(jet_streams: &Vec<i64>, num_rocks: usize, processed_rock_cache: &[[[[[u32;2];2];2];2];5]) -> i64 {
    let mut board = [0xffu8; BUFFER_SIZE]; // doing my own circular buffer; starting it uncleared to test clearing of circle
    let mut jet_stream_idx = 0;
    let mut first_empty_level: i64 = 0;  // can be thought of as the bounds of highest level
    let mut cleared_to: i64 = 0;
    let mut rock_flavor = 0;
    for i in 0..num_rocks {
        if i & 0x0000000000ffffff == 0 {
            println!(
                "Rocks dropped {i}: {}% done",
                100.0 * (i as f32) / (num_rocks as f32)
            );
        }

        // clear the path in the circular buffer - usually this is going to be just one line a frame,
        // so doing a trick where we check to see if it crosses the buffer boundary and writing in two slices
        // probably not much of an optimization
        while cleared_to <= first_empty_level + 4 {
            board[cleared_to as usize % BUFFER_SIZE] = 0;
            cleared_to += 1;
        }

        let mut collision = false;
        // consume 4 elements of stream now
        let stream0 = jet_streams[jet_stream_idx % jet_streams.len()] as usize;
        let stream1 = jet_streams[(jet_stream_idx+1) % jet_streams.len()] as usize;
        let stream2 = jet_streams[(jet_stream_idx+2) % jet_streams.len()] as usize;
        let stream3 = jet_streams[(jet_stream_idx+3) % jet_streams.len()] as usize;
        jet_stream_idx += 4;
        
        let mut rock = processed_rock_cache[rock_flavor][stream0][stream1][stream2][stream3];
        let mut rock_y = first_empty_level;  
        
        // we've just simulated sideways-vertical-sideways-vertical-sideways-vertical-sideways.
            
        while !collision {
            // start with vertical
            if rock_y == 0 {
                collision = true; // very first rocks special
                break;
            } else {
                let check_y = rock_y - 1;
                for i in 0..ROCK_HEIGHTS[rock_flavor] {
                    let board_bits = board[(check_y + i) as usize % BUFFER_SIZE];
                    let rock_bits = rock.to_ne_bytes()[i as usize];
                    if board_bits & rock_bits != 0 {
                        collision = true;
                        break;
                    }
                }
                if collision {
                    break;
                }
                rock_y = check_y;
            }

            // sideways
            let stream_shift = jet_streams[jet_stream_idx % jet_streams.len()]; // optimize away div
            jet_stream_idx += 1;
            let mut rock_preview = rock;
            if stream_shift == 1 {
                // shift "left" for the game is "right" for our representation
                // don't shift if it'll go out of bounds; test if bits are in last column (6 - 7 is always empty))
                if (rock & 0b01000000010000000100000001000000) != 0 {
                    // don't shift
                } else {
                    rock_preview = rock << 1;
                }
            } else {
                if (rock & 0b00000001000000010000000100000001) != 0 {
                    // don't shift
                } else {
                    rock_preview = rock >> 1;
                }
            }

            let mut side_collision = false;
            for i in 0..ROCK_HEIGHTS[rock_flavor] {
                let board_bits = board[(rock_y + i) as usize % BUFFER_SIZE];
                let rock_bits = rock_preview.to_ne_bytes()[i as usize];
                if board_bits & rock_bits != 0 {
                    side_collision = true;
                    break;
                }
            }
            if !side_collision {
                rock = rock_preview;
            }

        }
        
        // we've landed - stamp us on the map
        for i in 0..ROCK_HEIGHTS[rock_flavor] {
            let rock_bytes = rock.to_ne_bytes();
            board[(rock_y + i) as usize % BUFFER_SIZE] |= rock_bytes[i as usize];
        }

        // and its bottom edge is three units above the highest rock in the room (or the floor, if there isn't one).
        first_empty_level = std::cmp::max(first_empty_level, rock_y + ROCK_HEIGHTS[rock_flavor]);
        // if true {//i % 1 == 0 {
        //     for i in (0..20).rev() {
        //         println!("{:07b}", board[i]);
        //     }
        //     println!();
        //     println!();
        // }
        rock_flavor += 1;
        if rock_flavor==5 { rock_flavor=0; }
    }

    first_empty_level
}

#[test]
fn test_bitwise_simulation() {
    let highest_level = bitwise_simulation(&parse_input(get_sample_input()), 100000000, &build_processed_rock_cache());
    //assert_eq!(bottom_level, 0);
}

#[test]
fn test_drop_rocks_no_stream() {
    let mut board = VecDeque::from(vec![vec![0u8; 7]; 15]);
    let stream = vec![0; 3000];
    let mut bottom_level = 0;
    let mut highest_level: i64 = 0;
    let jet_stream_idx = drop_rock(
        &mut board,
        &mut bottom_level,
        &mut highest_level,
        0,
        &stream,
        0,
    );
    assert_eq!(4, jet_stream_idx);
    assert_eq!(vec![0, 0, 1, 1, 1, 1, 0], board[0]);
    assert_eq!(vec![0, 0, 0, 0, 0, 0, 0], board[1]);
    let jet_stream_idx = drop_rock(
        &mut board,
        &mut bottom_level,
        &mut highest_level,
        1,
        &stream,
        jet_stream_idx,
    );
    assert_eq!(8, jet_stream_idx);
    assert_eq!(vec![0, 0, 1, 1, 1, 1, 0], board[0]);
    assert_eq!(vec![0, 0, 0, 1, 0, 0, 0], board[1]);
    assert_eq!(vec![0, 0, 1, 1, 1, 0, 0], board[2]);
    assert_eq!(vec![0, 0, 0, 1, 0, 0, 0], board[3]);
    assert_eq!(vec![0, 0, 0, 0, 0, 0, 0], board[4]);
    let jet_stream_idx = drop_rock(
        &mut board,
        &mut bottom_level,
        &mut highest_level,
        2,
        &stream,
        jet_stream_idx,
    );
    assert_eq!(12, jet_stream_idx);
    assert_eq!(vec![0, 0, 1, 1, 1, 1, 0], board[0]);
    assert_eq!(vec![0, 0, 0, 1, 0, 0, 0], board[1]);
    assert_eq!(vec![0, 0, 1, 1, 1, 0, 0], board[2]);
    assert_eq!(vec![0, 0, 0, 1, 0, 0, 0], board[3]);
    assert_eq!(vec![0, 0, 1, 1, 1, 0, 0], board[4]);
    assert_eq!(vec![0, 0, 0, 0, 1, 0, 0], board[5]);
    assert_eq!(vec![0, 0, 0, 0, 1, 0, 0], board[6]);
    assert_eq!(vec![0, 0, 0, 0, 0, 0, 0], board[7]);
    let jet_stream_idx = drop_rock(
        &mut board,
        &mut bottom_level,
        &mut highest_level,
        3,
        &stream,
        jet_stream_idx,
    );
    assert_eq!(18, jet_stream_idx); // this one travelled extra two spaces, conumsed 4+2 streams
    assert_eq!(vec![0, 0, 1, 1, 1, 1, 0], board[0]);
    assert_eq!(vec![0, 0, 0, 1, 0, 0, 0], board[1]);
    assert_eq!(vec![0, 0, 1, 1, 1, 0, 0], board[2]);
    assert_eq!(vec![0, 0, 0, 1, 0, 0, 0], board[3]);
    assert_eq!(vec![0, 0, 1, 1, 1, 0, 0], board[4]);
    assert_eq!(vec![0, 0, 1, 0, 1, 0, 0], board[5]);
    assert_eq!(vec![0, 0, 1, 0, 1, 0, 0], board[6]);
    assert_eq!(vec![0, 0, 1, 0, 0, 0, 0], board[7]);
    assert_eq!(vec![0, 0, 1, 0, 0, 0, 0], board[8]);
    assert_eq!(vec![0, 0, 0, 0, 0, 0, 0], board[9]);
    let jet_stream_idx = drop_rock(
        &mut board,
        &mut bottom_level,
        &mut highest_level,
        4,
        &stream,
        jet_stream_idx,
    );
    assert_eq!(22, jet_stream_idx);
    assert_eq!(vec![0, 0, 1, 1, 1, 1, 0], board[0]);
    assert_eq!(vec![0, 0, 0, 1, 0, 0, 0], board[1]);
    assert_eq!(vec![0, 0, 1, 1, 1, 0, 0], board[2]);
    assert_eq!(vec![0, 0, 0, 1, 0, 0, 0], board[3]);
    assert_eq!(vec![0, 0, 1, 1, 1, 0, 0], board[4]);
    assert_eq!(vec![0, 0, 1, 0, 1, 0, 0], board[5]);
    assert_eq!(vec![0, 0, 1, 0, 1, 0, 0], board[6]);
    assert_eq!(vec![0, 0, 1, 0, 0, 0, 0], board[7]);
    assert_eq!(vec![0, 0, 1, 0, 0, 0, 0], board[8]);
    assert_eq!(vec![0, 0, 1, 1, 0, 0, 0], board[9]);
    assert_eq!(vec![0, 0, 1, 1, 0, 0, 0], board[10]);
    assert_eq!(vec![0, 0, 0, 0, 0, 0, 0], board[11]);
}

#[test]
fn test_drop_rocks_left_stream() {
    let mut board = VecDeque::from(vec![vec![0u8; 7]; 15]);
    let stream = vec![-1; 3000];
    let mut bottom_level = 0;
    let mut highest_level: i64 = 0;
    let jet_stream_idx = drop_rock(
        &mut board,
        &mut bottom_level,
        &mut highest_level,
        0,
        &stream,
        0,
    );
    assert_eq!(vec![1, 1, 1, 1, 0, 0, 0], board[0]);
    assert_eq!(vec![0, 0, 0, 0, 0, 0, 0], board[1]);
    assert_eq!(4, jet_stream_idx);
    let jet_stream_idx = drop_rock(
        &mut board,
        &mut bottom_level,
        &mut highest_level,
        1,
        &stream,
        jet_stream_idx,
    );
    assert_eq!(8, jet_stream_idx);
    assert_eq!(vec![1, 1, 1, 1, 0, 0, 0], board[0]);
    assert_eq!(vec![0, 1, 0, 0, 0, 0, 0], board[1]);
    assert_eq!(vec![1, 1, 1, 0, 0, 0, 0], board[2]);
    assert_eq!(vec![0, 1, 0, 0, 0, 0, 0], board[3]);
    assert_eq!(vec![0, 0, 0, 0, 0, 0, 0], board[4]);
    let jet_stream_idx = drop_rock(
        &mut board,
        &mut bottom_level,
        &mut highest_level,
        2,
        &stream,
        jet_stream_idx,
    );
    assert_eq!(12, jet_stream_idx);
    assert_eq!(vec![1, 1, 1, 1, 0, 0, 0], board[0]);
    assert_eq!(vec![0, 1, 0, 0, 0, 0, 0], board[1]);
    assert_eq!(vec![1, 1, 1, 0, 0, 0, 0], board[2]);
    assert_eq!(vec![0, 1, 0, 0, 0, 0, 0], board[3]);
    assert_eq!(vec![1, 1, 1, 0, 0, 0, 0], board[4]);
    assert_eq!(vec![0, 0, 1, 0, 0, 0, 0], board[5]);
    assert_eq!(vec![0, 0, 1, 0, 0, 0, 0], board[6]);
    assert_eq!(vec![0, 0, 0, 0, 0, 0, 0], board[7]);
    let jet_stream_idx = drop_rock(
        &mut board,
        &mut bottom_level,
        &mut highest_level,
        3,
        &stream,
        jet_stream_idx,
    );
    assert_eq!(18, jet_stream_idx); // this one travelled extra two spaces, conumsed 4+2 streams
    assert_eq!(vec![1, 1, 1, 1, 0, 0, 0], board[0]);
    assert_eq!(vec![0, 1, 0, 0, 0, 0, 0], board[1]);
    assert_eq!(vec![1, 1, 1, 0, 0, 0, 0], board[2]);
    assert_eq!(vec![0, 1, 0, 0, 0, 0, 0], board[3]);
    assert_eq!(vec![1, 1, 1, 0, 0, 0, 0], board[4]);
    assert_eq!(vec![1, 0, 1, 0, 0, 0, 0], board[5]);
    assert_eq!(vec![1, 0, 1, 0, 0, 0, 0], board[6]);
    assert_eq!(vec![1, 0, 0, 0, 0, 0, 0], board[7]);
    assert_eq!(vec![1, 0, 0, 0, 0, 0, 0], board[8]);
    assert_eq!(vec![0, 0, 0, 0, 0, 0, 0], board[9]);
    let jet_stream_idx = drop_rock(
        &mut board,
        &mut bottom_level,
        &mut highest_level,
        4,
        &stream,
        jet_stream_idx,
    );
    assert_eq!(22, jet_stream_idx);
    assert_eq!(vec![1, 1, 1, 1, 0, 0, 0], board[0]);
    assert_eq!(vec![0, 1, 0, 0, 0, 0, 0], board[1]);
    assert_eq!(vec![1, 1, 1, 0, 0, 0, 0], board[2]);
    assert_eq!(vec![0, 1, 0, 0, 0, 0, 0], board[3]);
    assert_eq!(vec![1, 1, 1, 0, 0, 0, 0], board[4]);
    assert_eq!(vec![1, 0, 1, 0, 0, 0, 0], board[5]);
    assert_eq!(vec![1, 0, 1, 0, 0, 0, 0], board[6]);
    assert_eq!(vec![1, 0, 0, 0, 0, 0, 0], board[7]);
    assert_eq!(vec![1, 0, 0, 0, 0, 0, 0], board[8]);
    assert_eq!(vec![1, 1, 0, 0, 0, 0, 0], board[9]);
    assert_eq!(vec![1, 1, 0, 0, 0, 0, 0], board[10]);
}

// old input stream doesn't work anymore

// #[test]
// fn test_first_few_sample_rocks() {
//     let puzzle_ascii_art = "|.......|
// |....#..|
// |....#..|
// |....##.|
// |##..##.|
// |######.|
// |.###...|
// |..#....|
// |.####..|
// |....##.|
// |....##.|
// |....#..|
// |..#.#..|
// |..#.#..|
// |#####..|
// |..###..|
// |...#...|
// |..####.|";
//     let expected_board: Board = puzzle_ascii_art
//         .lines()
//         .rev()
//         .map(|line| {
//             line.split_at(8)
//                 .0
//                 .split_at(1)
//                 .1
//                 .chars()
//                 .map(|char| if char == '#' { 1 } else { 0 })
//                 .collect()
//         })
//         .collect();
//     print!("{:?}", expected_board);
//     let jet_streams = parse_input(get_sample_input());
//     let (actual_board, _) = simulate(&jet_streams, 10);
//     let mut idx = 0;
//     for row in expected_board {
//         println!("{idx}");
//         assert_eq!(row, actual_board[idx]);
//         idx += 1;
//     }
// }

fn parse_input(input: &str) -> Vec<i64> {
    input
        .chars()
        .map(|char| {
            if char == '>' {
                1
            } else if char == '<' {
                0
            } else {
                panic!()
            }
        })
        .collect()
}

#[test]
fn test_parse_sample_input() {
    let results = parse_input(get_sample_input());
    assert_eq!(get_sample_input().chars().count(), results.len());
    assert_eq!(1, results[0]);
    assert_eq!(1, results[1]);
    assert_eq!(1, results[2]);
    assert_eq!(0, results[3]);
    assert_eq!(0, results[4]);
    assert_eq!(1, results[5]);
}

fn get_sample_input() -> &'static str {
    ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"
}

fn get_puzzle_input() -> &'static str {
    "><<<<><<>>><<<>>><<><<>>>><<><<>><><<<<>>>><<>>><<<>>>><<<<><<<<>>>><>>>><>>><<><>><<>><<<<>><<><<><>><<><>>><<<>><>>><<>><<>>><<<><>>><<>>><<<<>><<<<><<<><<>>><<<>>><>>><<<>>><<<<>>>><<<>>><<<>><<<<>><<<>>><<<>>><<>><<>><<<>><>><<<><<>><<<>>>><>>>><>><<>>>><<<<><<<<><<<<><<>>>><<<>>>><<<><<>>><<<><><<>><<>>><<>><>>><<>><<>>>><<<<><<<>>><<>>>><<>>>><<>><>>><<<<>>><<<>><<<<><>>><>><<<<>>><<>>><><<<>>><<<><<<>><<<<>>>><<<><<<><<>>>><<<>>><<><<>>>><<<<>><<>><<<<>>>><<<><<>>><<<<>>>><<<<>>>><<<<>><><>><<>>>><>>>><<<<>><>>><<<<>>><><<><<<>>>><><<<>><<>>><<<>>><<<>><<<><<>><<><<>>>><<>>><<<<><<<<><>>>><<>><<>>><<<>>>><<<<>>>><<<>>>><<>><>>>><><<<>>>><<<<><>>><>>><<>>>><>>><<<<><<>>><<<<><<>>><<>><<<>><<<<>>><<>>><<<>>><>>><<<<><>>><><<<<>>><>>><<<<><><<<<>>>><<>>><<<<>>><<<<>>>><>>>><<><<<<>>><<<<>><<<<>><<>><<<<>>>><<<>>>><<<<>>><>>>><<<<>><<<<>><<>>>><<<><<<>>>><<><<<<>>>><><<<<>><>><<>>>><<<<>>><<>>><<>>><<<>>>><>>><<<<>>>><>>><<>><<><<>><<><<><<<>>><>><<<>><<<><<<<><<<<>><<>><<<<>>>><<<<><<<<>>>><<><<><<<<>><<<>>><>>><<<<><><>><<>>><<><>>><<<>><<<<><<<<>>><<<>>>><<>><<><<<<>><<<<>>><<>><<>><<<<><>>>><><<>>>><>>><<<><>><<>>>><>>><<<<><<<<>>><<<>>><<<>>>><<<<><<<<>>>><<><<<<>>><<<>><<>>>><<>>><<>><<><<<<>>><<>><<><<<<>>><<<><<<<>>><<>>>><<<<>><>>>><<<<>>><<>>><<><<>><<<>>><<<<>>>><<<<>><<><<>><<<<>>>><><<<><><<<><<<><<<<>><>><<<>>><><<<><<<<><<<>><>>><<>><>>><<>><<>><><>>>><<<><>>><<>>>><<<>><<<>><<<<>>><<<<>>><<<<>>>><>><<<>><<>><<<><<>>>><>><<<><<<<>>><<><<<<>><<>>><<<<>>>><<<>>>><>>><<<>>><<<>>><<>><<<<>>>><<<<><><<<<>>>><<<>><<<<><<<<>>><<<<>>>><<>>><<<<>>>><<<<><<<><>>>><<>>>><<<<>><<<>>>><<<<>><<>>>><>><>>>><<<>><<>>>><<<>>>><<>><<<>>>><<<>><>><><<><>><>><<<>><<<>>><<>>><<<<>>><<<><<>>><<<<><>>>><<><>>>><<><<<>>>><><<<<>>><<<<>><<>>>><<<><<<>><<><>>><<>>>><<><>>>><<<<><>><<>><<<>>><>><<<>><<<<>>><<<<>><>><<><<<<><<<>>><><>>><<>>>><<><<<<>>><<>>><<<><<>>><><<<><>>><<><<<>>><<>>><<><<><<<<>>><<><<<<>><<<<>><>>>><<<<><>>>><<<<>>>><<>><<<<>>>><<<<><<<>>><<<<>>><<<<><<<><<>><>>>><>>><<<>>>><<<<>>><<<>><<<<>>>><<<>>><<<<>>><<<<>>><<<>>><<>><<<>><<<<>><<<<>>>><<>><<>><>>><<<<>>><<<<>>><<<<><<>>><><<<<>><<>>>><<><<>>><>><<<><>>><<<<><<>><<>>><><<>>><<<<><<<<><>>>><<>>><<>>>><<<>>><<>>><<<<>>><<>>>><>><><<<<>>>><>>><<<><<<<>>><<><<>>>><<<<>>>><>><>><<<>>>><>><><<>>><<<<><<<>>><<>>><><<>>>><<>>>><<<><<<>>><<<><<<>>><<<><<>>><<<<>><<<>><>>><>><<<<>>>><><<<<>>>><><<<<>>>><<<>>>><<>><<<>>><<<<>>>><<<>><<><>>>><<<>>><<<<><<<>>><<<>>><<>><<>>>><<>>><>><<<>>>><<>>>><<<><<>>><<<>><><<>>><<<<>>>><<<><<>>><>><>>>><><>>><><<<<>><<<><>>>><>><<<>><>>><<<><<><><<>><<<>>>><<<<>>>><>>>><<<><<<>><>>>><<>>>><<<>>><<><<<<><<<<>><<<>><<>>><<><><<<<>>><>>>><<<><>><<><>><>>>><><>>><<><<>><<<>>>><<>><<><<<><><<<>>>><<<<>><<<<>>><<<<>>>><>>><<<<><<>>>><><<>>><<<<>><<<>><<<>>>><><<<>><<<<><<<><<>>>><<<>>>><<<<>>><<<>>>><<<<>><<<>>><<<>>><>>>><<<<><<<<>><>>>><<>><<<<>>><<<<><<<<><<<<><<<<>>><<<>><<<>><<>>>><>><<<<><<><<>>><<>>>><<<><>><<<<><<<>>>><<<<><>><<<>>>><<<<>>>><<<>><<<><<<>>>><<>>>><<<>><><<<<>>>><<>>>><<<>>><>>>><<<<>><<<<>><>>><<<<>>><>>>><<<<>>><<>>><<<>>><<<>>><<<<>>><<<<>>>><<<<>><<<<>><>>><<<<>>><<<>>><<>><<<<><<>>><>>>><<<>>><<>>><<><<<>>>><<<>>><<>>>><<>>><><<<<>><>>>><>>><>>>><<<>><<>><<<<><<<><>><<<<>>>><<<><<>><<<<>><<>><<>><<>>><<<<>>><<<<><<<>>><>><>>>><<<>>><>>>><<<<>><<<<>><>>>><<>>><<<<>>><<>>><<<<><>>>><>><><<<<>><>>><<<<>><<<<>>>><<<>>>><<<<>><<<<><<<<>>><>>><<<<>><<<><<<><><<<>>><<>>>><<<<><<<<>>><<<<><<>>><<<>>><<<<>>>><<<>>><<<<><<><<>>>><>>>><<<<>>><<<<>>>><<>>>><<<>><<<<>><<>>>><<<>>>><<<<>>><<<<>>><<<><<><<<<>>>><<><<><>>>><<<>>><<<<>>><>><>>>><<<<>><<<>>>><<<>><>>><<>><>><<<<>>>><><<<>>>><<>>>><<<<><<<>>><<<<>><<<<>>>><<<><<<>><<>><<<>>>><<<>><>>>><<>>>><<<>><>><>>>><<<>>><<<<>>>><>><<>><><>>><<<<><<>><>><>>><<>><>><<<><<>>>><<>>>><<<>>>><<>>>><><<>>>><<<>><<<<>><<<<><<<<>>>><>>>><><>>><<<<>>>><<<>><<<>>><<<<>>><<><<<>>>><>>><>>><<<>>><<<><>>><<<><<<>>>><<>>><<<<>><<<<>><<<>><<<<>>><<<<>>>><<<<>>>><>>>><><<<<>><<<<>><<<<>>><<<<>>>><>>><<>><>>><>>>><>>><<<><>><>>><><>>><>><>>>><<<<>><<<>>>><<<>>><<<<>><<<<>>><>><><<<<>><<>><<<><<>><<<<>>><<<>><>><>><<><<<>>>><<<<>><<<<><<<<><<<>>>><<><<<<>><<<>>><<><<><>>><<>><<>>>><<<>><<>><>><<>><<<>>>><>>>><><>>>><<<<>>>><<>>>><<>>>><<<<><<>><><<<<>>><<>>>><<<<>>><>>>><<<>>>><<<>><<>>><<>>><<<>>><<<<>>><<><<<<><<<<>>><<>>><<<<>>><<<>>>><<<<>><<<<>>>><<<<>>>><<><>><<>>><<>><<<>>>><>>><>><<<<>>><>><<<<>>>><<<<>><><<<><>>>><><<>>><<<<>><<<<>>><>><>>><<<>>><><<<<>>><>>><<<<>>><>>><<<>><>>>><><<<<>>>><<<<>><<<><<<>><<<>>><<<>>><<>>><>>><<<<>><>>><>>><<<>><<>>><<><<<<>><>>>><>>>><<<>><><>>><<<><>><<<>>>><<>>><<<<>>>><<>>><<<<>><<<<>>>><<<<>>>><<>>><>>>><<<<>>>><<<>>>><<<<>>><<>><><>>><<<>>>><<>>>><>>><<<<>>><<>>><<>>><<<>><>>><>>><<><<<><<<><>><><>>>><<<>>>><<<<>>>><>>><<<>>><<<<>><<<<><<>>><<>>>><<<<>>><<<<>>>><<>><<<<>>><<<<>><<<>>><<>>>><<>>>><<<<>>><<>><>><>><>><<<>>>><<<<>>>><<<>>>><<<<>>>><<<<><<<>><<<>><<<><<>>><<<>><>><<>>><<<><<><<<>>><<<<>><><<>>><<>>>><<><<<>>>><><<<>>>><<<<>><<<>>>><<>>><<<<>>><<<<>>><<<<>>>><<>>>><<><<<<>><<><<>>>><<><<>>><<<>>>><<>>><<<<>>><<<>>><<>>>><<<>>>><<<<>>><<<<>>><<<<>>><<><<>>><<<<>><<<<>>><<<><<<>>>><<<<>>>><<><><<>>><<<<><<<>>>><<<>>>><<<<>>><>>><<>>>><>><<<<>>><<<<>>>><<>><<<<>>><<<>>>><<<>>>><<<<>><>>>><<<<>>>><<<>><>>><<<<><<<>>><<<<>>>><<<<>>><<>>>><<>>>><<<>>><<<<>><>>><<<<>><<<<>>>><>><<<>>>><<><<>>>><<<>>><<><>>>><<<>>><<<>>>><<<<><<<>><<<<><<<>>>><<<<>>>><<<>>>><<<>>><>>>><<<><>>><<<<><<<><>><>><>><<<<>>><<><<<>>><<>><<<<>>><<<<>>><<<<>>><<><<<<>>>><<>>><<<>><<<>><>><<<>>>><>>>><<<<>><<><<<>><<<<>><<>>><<<>>>><<<><><<<>>><><<<<>>><<>><<>><>><<<<>><<<<><>>>><><>>>><<<><<><<<>>><>>><<<>>><<<<><<<<><><<><<>><<>><<<<>>><>>>><<<<>><<><>><>>><>>>><<<>>><<><>><<<<><>>>><>>>><>>>><<>>>><<<>>><<>>><<<>>><>>>><<>><<>><<<<>>>><<<>>><<>><>><<<>>>><>>><<<>>><<<>>><<<<>>>><<><>><<<>>><<<>>>><>>><<><>><<<<>><<<<>>>><<<<>>>><<<>><<<<><>>>><<><><<>><>><><>>>><<<>><<<<><<<<><><><>>><>><<<<>>>><<>>>><>>>><<>><>>>><<<<>><<<<>>><><<<>><>>>><>>>><<>>>><<<<>>>><><<<<><><<<<><<<<><<<>><<<>>>><<<<>>>><>>>><<<<>><<>>>><<<<>>>><>>><<<>>>><<><>>><<<<>>>><<><<><<<>><>><<><<>><<<<>>>><>>>><<>>><<<<>>><>>>><<<>>><<<>><<<><<<><>>><>><>><<<>>>><<<<><>>>><<<<><<<<><<>>><<<>>><<<>><>>><<<>><<<>><<><<<<>><<<>>>><<<>>>><>>><<<>>>><>>>><<<><<>><>><<<<><>>>><<<<>><>>>><<<><<<<><>>><<<<>>>><<<<>>>><<<>><>><<<>>><<<><<<<>><<><>>><<>>>><>><><<<<>>>><<<<>>><<>><<><<<<><<<<>>><<<>><<><<><>>>><>><<<<><<<><<<<>><<<<>><<<>><<><<<<><<<>>>><<><<<<>><<<<>>>><>>><<<<><<<>>><>>>><>>>><<<>>>><<>>>><<>>><<>>>><<>><<<<>>><<<>>><<<<>><<><<>>>><>><<>>>><<<<><<<>>>><<<>>>><<<>>>><<<><<<>><>>><<<<>>><<>><<<><<>>>><<<><<>>>><<<<>><<<<>><>><><<<>><<>>>><<<>><<<<>>><<<><<<<><<<>>><<<><<<<>>>><<>>>><>><<<<>>><><<<>>><<>>>><<<<><<>><<>>><>><<<>>>><<<<>>>><<>>>><<<<><><<<<>><><>><>><<>>><<>>><<><<<><><<<><><<>>><<<<>>>><<<>>><<<>><<<<>><<<>>><<<<><>>>><<>>>><<<<><<<>><<<>>>><<>><<>>>><<<>><<><<<>>>><>>><<<>>>><<>><<<<>>><<>><<<<><<>>><<<<>>>><>>>><<<>>><>><>>>><>>>><<>>><>>>><>>><<>>><<>>><<<>>><<<<>><>>>><<><<>>><<<>>>><<<<>>><<<><<<>>><<<>>><<>><>>><><<<><<>>><<>><<<>><<>>>><<>>>><>>>><<<>>><<<>>>><<<<>>>><><<<<>><<<>><<><<<<>><<<<><>>><>><>>><>>>><<>>><>>><<<<><<<<><<<>><>>>><>><<>>><><<<>>>><<<>>>><>>>><<<<>>><<<<>>>><<<>>><<<><>>>><><<<>><>>>><<><<<><<<>>><<>>><>>><<<<>>>><<><<<<>>>><<>>><<<<>>>><<<><<<<><>>><<<><>>><<<<>>><<<<><><<<<>><<<>><<<<><<<>><>>><>><<>>>><<>>><<<<>>><>>><<><<<<>>><>><<<<>>><><><<>>>><<<>>>><<>>>><<<<><>>>><<<<>><<>>>><>><<<>>><<>><>>><<<><<<>><<<><<>>><>>><<><<>><<<<><<>>>><<>>>><<<>>><<<<>><<<>><<<>>><<<>><<<<>>><<<<><><<<><><<<>><><>>>><>>>><<<><<<><<<<><<<>>>><<>><>>>><<<<>>>><>><<<<>>><>>>><<<><<<>>>><<<>>><><<<>>>><<<>>><<<<>>><>>><<<<><<<>><<<><><<<>><<><>>>><<>><<>>><<>><<<><>>>><<>><<<>><<>><>>><<<><<<<>>><<>>>><<>>><<<<>>><>>>><<<<>>><<<<>>><<>>><<>>>><<<<><>><<<>>><<<>>><>><<>>><><>><<>>><<<<>><<<<><<<<>>><<<<>>><<>>>><<<<>>>><>>><<<<><<>>>><<>>><>><<<<><<<<>>><<>>><<<<>>><<<><<>><<<<>>><>>>><<<>>>><<>><>><<<><<<>>>><<>>><<<<>>>><<<<>><<<<>>>><<<>>>><<>>><<<<>>><<<>>>><<>>><><>>>><<<<>>><<<<>>><<<>>>><<<<>>>><>>>><<>>>><><<>><>>>><<<<>>><<<<><<<>><>><<>>>><<>><>>><<>><<>>><<>>><<>>>><<<<>><<<<>><<<<>>>><<<<>>>><<>><<<><<>>><<>>><<<<>>><>>>><<><>><><>>><<<>><<<<>>>><<>><<<><<<>>><<<<>>><<>>>><<>><<<>>><<>>>><<>><<>>><<<<>><<<<>><<<><<<>>>><<<>>><<<<>><>><>>><>>>><>>>><<>>>><>>><<<>>><<><>>>><<<<>>><<<>><<<<>><<><>>>><<<<><<><>>><<<<>><<<<>>><<<><<><<<>><>>>><<<<>><<<<>>>><<<>>><><<>>>><<><><<>>><<<<>><<<<>>><>>>><<<>><<<<>>>><<<>>><<<<>>>><<><<<>>><<>>><<<><><<<<><><<>><<<<>>>><<<<>><<<<><<>>>><>><>><<>>>><>>>><<<<>><><<<>><<<>>>><<><<>><<<<>><<><>>>><<<<>><<>><<<>>><>>>><<<<><<<<>><<<><<>>>><<<><<<>>>><<>>>><<<<><>>><<<>>>><<<<>>>><>><>>>><<<>><<>>>><<<>>>><>>>><<<<>>>><<<<>>><<<><<><<<<>><>>>><<<>>><<>><<<<>>><<<<>><>><>>>><>>>><<>><>>>><<<<>>><>>><<<<>><<><<<>>><<>>>><<<<>>>><><><<<>>><<<<><<<<>><<><<<<>><<<>>><<><<<<>>>><<<<>><>><>><><><<><>><<<<>>><<><<<>>><<><<>>>><<<><<<<><<<<><<>>><>>><<>>><<>>>><<><<<<>>>><><<><<>>><><<<<>><>>><><<<<>>><<<<>><<<>>>><<<<><<<>><<<>><<><><<<>><<<>>>><<<><<<><>>><<<<>>><<>><<<>>><<<>>><<<><<><<<<>>><<<><>>>><<><<<>>><>>><<<>>><<<>>>><<<>>><<<>>>><<<><<>>><<>><<<>>>><<<<>><<<<><><<<<>><<>><<<>>><<<>><<<>>><<<<>><<>><>><<<><<>>><<<>>><>>><<<>>><<<>><<<>>>><<<<>><<>><<>><><<<>><<<<>>><<<>>><<<<><>>><<<<>><<<<>>>><<<>>><<<<>><<>>><<>>><<<>>><<<>><<>>>><><<>>>><><<>>><><<<<>>>><<<<>><>><<<<>>><<<>><<<><<>>><<<<><>>><>>><<>>>><<><<<>><<<>>><<<>>><<<>><<>><<><<<<><<<<>>><<<><<<<><<<<><<<><<<><>>><<>>>><>>>><>>><<><<<<><<<>>><<<>>><<><<<<>>>><<<<>>>><<>>>><<>>>><<<<><<>><>><<<><><><>><<<>><<<<>>>><<<<>><<<<>>><<><<<<>><<><>>><<<>><<<><<<<>><><<<<><<<>>><>>><<>>><<>>>><><<>><>>>><<<<><<<<><<><<<>><<<>>>><<>><<>>>><<<<>><<<>><<<><<>><<>>><<>>><<>>><<>>>><<>>><<<<>>><<><<<<>><>><<<><<<>>><<>>>><<<>><>>><<><<<<>>>><>>>><<><>>><<<<><<<><>>>><<<<>>>><<<><>><<<<><<<>>>><<<><<<<>>><<<>><><>><>>>><<<<>>>><>><<<<>>><<>><<<<><<>>>><<<<>><>>><>><<<>>>><>><<<><><<>>>><<<>><<>><>><<><<>><>>><>>><<><><<>>>><<>>>><>>><<>>>><>><"
}
