fn main() {
    println!("Hello, world!");
    let mut elves = parse_input(&get_puzzle_input());
    let answer = simulate(&mut elves);
    println!("boom? {answer}");

}

fn parse_input(input: &str) -> Vec<(i32,i32)> {
    let mut elves: Vec<(i32,i32)> = vec![];

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c=='#' {
                elves.push((x as i32,y as i32));
            }
        }
    }

    elves
}


fn simulate(elves: &mut Vec<(i32,i32)>) -> i32 {
    let mut next_dir = 0;
    for _ in 0..10 {
        (_, next_dir) = simulate_round(elves, next_dir);
    }
    let left = elves.iter().min_by_key(|elf| elf.0).unwrap().0;
    let right = elves.iter().max_by_key(|elf| elf.0).unwrap().0;
    let top = elves.iter().min_by_key(|elf| elf.1).unwrap().1;
    let bottom = elves.iter().max_by_key(|elf| elf.1).unwrap().1;
    let square_area = (right-left+1) * (bottom-top+1);

    square_area - elves.len() as i32
}

#[test]
fn test_simulate() {
    let mut elves = parse_input(&get_sample_input());
    assert_eq!(110, simulate(&mut elves));
}

fn simulate_round(elves: &mut Vec<(i32,i32)>, first_dir: i32 ) -> (bool, i32) {
    // During the first half of each round, each Elf considers the eight positions adjacent to themself.
    let mut proposals: Vec<(i32,i32)> = vec![];
    for (elf_idx, elf) in elves.iter().enumerate() { 
        let nearby_elves: Vec<&(i32,i32)> = elves.iter().filter(|&e| e!=elf && e.0 >= elf.0-1 && e.0 <= elf.0+1 && e.1 >= elf.1-1 && e.1 <= elf.1+1).collect();
        // If no other Elves are in one of those eight positions, the Elf does not do anything during this round. 
        if nearby_elves.is_empty() {
            proposals.push(*elf);
            continue;    
        }
        // Otherwise, the Elf looks in each of four directions in the following order and proposes moving one step in the first valid direction:
        let mut examine_dir = first_dir;
        for _ in 0..4 {
            match examine_dir {
                0 => {
                    // If there is no Elf in the N, NE, or NW adjacent positions, the Elf proposes moving north one step.
                    match nearby_elves.iter().find(|e| e.1==elf.1-1) {
                        Some(_) => {}
                        None => { proposals.push((elf.0,elf.1-1)); break;}
                    }
                },
                1 => {
                    // If there is no Elf in the S, SE, or SW adjacent positions, the Elf proposes moving south one step.
                    match nearby_elves.iter().find(|e| e.1==elf.1+1) {
                        Some(_) => {}
                        None => { proposals.push((elf.0,elf.1+1)); break;}
                    }
                },
                // If there is no Elf in the W, NW, or SW adjacent positions, the Elf proposes moving west one step.
                2 => {
                    // If there is no Elf in the S, SE, or SW adjacent positions, the Elf proposes moving south one step.
                    match nearby_elves.iter().find(|e| e.0==elf.0-1) {
                        Some(_) => {}
                        None => { proposals.push((elf.0-1,elf.1)); break;}
                    }
                },
                // If there is no Elf in the E, NE, or SE adjacent positions, the Elf proposes moving east one step.
                3 => {
                    // If there is no Elf in the S, SE, or SW adjacent positions, the Elf proposes moving south one step.
                    match nearby_elves.iter().find(|e| e.0==elf.0+1) {
                        Some(_) => {}
                        None => { proposals.push((elf.0+1,elf.1)); break;}
                    }
                },
                _ => { panic!(); }
            }
            examine_dir = (examine_dir+1)%4;
        }
        // if we didn't find a direction to propose, propose we stay where we are
        assert!(proposals.len()>=elf_idx);
        if proposals.len() <= elf_idx {
            proposals.push((elf.0,elf.1));
        }
    }
    
    // After each Elf has had a chance to propose a move, the second half of the round can begin. 
    for (i, proposal) in proposals.iter().enumerate() {
        // Simultaneously, each Elf moves to their proposed destination tile if they were the only Elf to propose moving to that position. 
        // If two or more Elves propose moving to the same position, none of those Elves move.
        if proposals.iter().filter(|p| *p==proposal).count()==1 {
            elves[i] = *proposal;
        }
    }
    
    // Finally, at the end of the round, the first direction the Elves considered is moved to the end of the list of directions.
    (false, (first_dir+1)%4)
}

// #[test]
// fn test_simulate_round_done() {
//     let mut elves = vec![(0,0), (2,0),(0,2),(2,2)];
//     let (done, new_dir) = simulate_round(&mut elves, 3);
//     assert!(done);
//     assert_eq!(0, new_dir);
//     assert_eq!(vec![(0,0), (2,0),(0,2),(2,2)], elves);
// }

#[test]
fn test_simulate_round() {
    let mut elves = parse_input(&get_sample_input());
    let (done, new_dir) = simulate_round(&mut elves, 0);
    assert!(!done);
    assert_eq!(1,new_dir);
    assert_eq!(Some(&(4,-1)),elves.iter().find(|elf| **elf==(4,-1)));
    assert_eq!(Some(&(2,0)),elves.iter().find(|elf| **elf==(2,0)));
    assert_eq!(Some(&(6,0)),elves.iter().find(|elf| **elf==(6,0)));
    assert_eq!(Some(&(0,1)),elves.iter().find(|elf| **elf==(0,1)));
    assert_eq!(Some(&(4,7)),elves.iter().find(|elf| **elf==(4,7)));
    

}

#[test]
fn test_parse_input() {
    let elves = parse_input(&get_sample_input());
    
    assert_eq!(22,elves.len());
    assert_eq!((4,0),elves[0]);
    assert_eq!((2,1),elves[1]);
    assert_eq!((4,6),elves[elves.len()-1]);
}

// fn put_sample_input_in_map(map: &Vec<Vec<u8>>, input: &str, topleft: (usize,usize)) {
//     for (y, line) in input.lines().enumerate() {
//         for (x, c) in line.chars().enumerate() {
//             if c == '#' {
//                 map[topleft.1+y][topleft.0+x] = 1;
//             }
//         }
//     }

// }

fn get_sample_input() -> &'static str {
"....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#.."
}

fn get_puzzle_input() -> &'static str {
"...####..#.#.#....###..#....#..#..#...##..###..##.#...######.##....####
..#.##..#.#...###..###.##....#..##......#.###..###....#.#.#....#...#.#.
.##...#...####..####....#....#####.##.#.....#.#.#.#..#..#...####....###
.####..#.##...#.##..#.#...#..#..##.######.#######.###.##.....####.#.##.
##.##.#..######.####..##...##.#.##...###.###..#.#..####..##...###.##.#.
...###.#.###..####..###.##..#..###..#..##.###.##.##....##...##.#.#..##.
###.##...#...#.#..####..##.#..###.##..#..##.#...#...##..##..##.##.###.#
..##....###.##.#...#####.#.#.###.#.#.#.##.#...#####.#####.....###.....#
#.####.#...#.#...##..#..#.#...###.#.####.#..##..##.#.###...#.####....##
#.##...#.#####...##....##..###..#...##.###..#.#.##.##.....#####....#...
#...#..#..#.#...##..#####.#.###......##..##.##.##...###..###...#..###..
..#.##.#..##.##.####.....#.###..#..#..#...###.###..#.#.##.##..#####..##
.##.#...#.....####......#.#..#..#.#....#.#...####...##.#.#.....#.#.....
#..#.##..###..#.....#.###..#.#..........##.......##.##.#..######...###.
.#...##.#.#...##.####..######.###.#.#.#.####.##.###..#...###.#..#..##.#
#.#.##.#.###..##.###.##.....###.#.#...#...###.##.#.#.##.##.#####.....#.
.#...###.##...#.##.#.###.###.##......##...##.#.###..#..#.#.##...####.#.
.#.#.##..#...#.##..#..##.....#....#....#.###....#########..#.#.########
.#.##..###..##..#.......##.##..#..###.#.##.####....#..#.#...##..##.#...
#####.##.#.....#.##...#.#...#.#.####..####....#.#...#.....###...##.....
..###....#....##...#.#...###....##.###.#.#...##..##.#..#.##....#.#.#.##
########..####...#.#..##.###...##.#...#...#.######...#..#..#..#...#..##
#.#....##...##..#...#....#.##....#.###.##.#...###.#..#.#.#...###..#.#.#
#.##........#..##...#...##.##.###.#.##...##...#####.#.#.....####.#.#..#
##.##...####.##.#...#....#...###.#...#....#.#.#.####...#.#....#.####.#.
.###....#..#.#..#..####..###.#.#..#.#..#.####..#####..#.#.#..#.....##.#
.#####.#.##.###......#..##.#.....#######..##.##.#####....#..#.#..####.#
.#....###....#.##..##.#.#.#.....#####..#.#.#.##...#.#....#.##..##....#.
####....####...#.#...#..##....#.#.#.###..#.######.##..##..######...#.#.
.#..####..#####.###..#..##.#.####.##.##..#.###..#...##.#...#.#.#.#.#.##
#.###########.####.....###.#..#...###.#######...#.#...##..###....#....#
#..#.####...###..####.#.##.#.#.#.#####.###.####...#...#....########..#.
#######.#..#.#..####.##.##..##.#...##..#######..#.#..#...#..##..#.###.#
##.#.....###.#..####.##..#.#######.#.#..#.##.#..###.##..##.##.#...#.#.#
#.#######.#.##.#.###.##.##...##..#.#....#.#.#..#.#..###..##.#.#.....##.
.##.##.#...####.#####.####.####.....#..####..#..##..###...#.#.###.#...#
.####...#..#.#.#.#........#..###...#####..####..##..######.###...#####.
###....#..##.##..#####....##.#..#...##.#####..#..###.#####..###.#..##.#
..#.#...#.##..#.##.####.####..#######....##....#.#....#.#.#..##.#.#..##
.#..#.########.........###.#.....###.######..#.######...####..#..#.#.##
.#.#...###...#.#.#.#..#...##.###.#.#.##.##.....####......##.#.#....##.#
..##..##...#.#######.#.##.####.###.#..#.....#.#..#...#..####..##..#.###
.#...####.##....###..#.###...###.##...###..######.#.#.#.#.#.#.#....#.##
.###..#.###.###..##.#..##..#.#.#...#..#.#..#...##..#..#...###...#####.#
#.#..#.#.#.....##..#.##..##.#.....#..###..#.#.#.#.#.#.##..#.####..##...
..#.#.####...#.##..##.##.####..##.#.#..##.#.###.#.######..#.######.#...
###.#.#.#.####..##.######.#.#...##......###..###.#..#...#...#.#..######
.###.#..#..######..##.####.###..##.#...#.#.#......###..##.#.#....#.#.#.
.##..#...#....#..###.##.#..#####.#.#....######..#....#..######.....#...
#.#...#..#.##...#..#..###.......####...#.####...#.#.#....#####.##...#.#
....#.####.#...#..#####.#..###..######.###.....#..#.#.#.#.#.##.#.##....
#.......#####.#.###..##.###..#####..#.###..#.....#.###.###..###..#.##.#
##.#.#....##.#.#..#..#...###.#..#.....###...###...#..#.######.#.#...##.
#....####.##.###....#....#....#...#..####...#.#.#.###..##.#.#..##.#..##
#.....#.##....###..####..##.#.......#..#.#.#.....#....####.#...#....###
###.#..####....#.....#..####....######.....#.#....#..###.#.#..#.#..###.
#..#.#.#...##....##.##....##.....#..####.###..##.#.#.##..#.##.###...###
....###.#.#.#..###.#.##.#.#.#.#####.#####..#..##...##.##.#...####.#.###
#####.....#.#.#.#.#.#.###.#####.#...#.##..#.#.##..##..###...#...#..##..
##.####.#.#.##..##.##.###....##.#..###.####..#######.#...##....#..##.##
##..#.####.##..#..#.#.#####.#.#.....#####..##..#.##.....##..#.#....##.#
.#..####...####.#####.##..#.##.....###.###.#.#.######..####.#...#..#.#.
##.####..##..#.....##.#.####..#..#....##...#....#####..##########.###.#
#...#.#.....#####.......###.#.#.#.###....#.#..#..#.##..#..#..#.####.###
##.###..#..##......##########....###..#######..#....#.#..###.##........
####.........#.#...##.#.#.#..#.#.####.#####....##...#.#.##.###..#.##.##
.####.#...#..#.##.####..##..#..#...##...........##..###.####..##..#.#..
.#####.##....##.#..####.##..##.#.###.....##.#..#..#.#.#...#.#...####...
#...####........#.##.####..##.....#####.#.#####.#####..#.##.#..##...##.
.###..###.####..###..##..##.#..##.###.#...#..#...#..#...#..#######.#.##
.#..#..##...##....##....####..#....#.##.....#.##..#.#..#...#..##.#..#.#"
}