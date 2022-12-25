use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
    let (width, height, mut blizzards) = parse_input(&get_puzzle_input());
    let result = process_blizzards_3(&mut blizzards, width, height);
    println!("boom? {result}");
}

#[derive(PartialEq,Debug)]
struct Blizzard {
    x: usize,
    y: usize,
    facing: u8
}

// if we pre-process blizzards we can get a list of gaps at what times. gaps that neighbor in time and space become the graph 
// we traverse. easy

#[derive(PartialEq,Debug,Eq,Hash)]
struct Gap {
    id: usize,  // weaked out on this one and didn't have the energy to do it the RUSTy way
    x: i32,
    y: i32,
    t: usize
}

fn tick_blizzards(blizzards: &mut Vec<Blizzard>, width: usize, height: usize) {
    for blizzard in blizzards {
        match blizzard.facing {
            0 => { blizzard.x = (blizzard.x+1)%width; }
            1 => { blizzard.y = (blizzard.y+1)%height; }
            2 => { blizzard.x = (blizzard.x+width-1)%width; }
            3 => { blizzard.y = (blizzard.y+height-1)%height; }
            _ => { panic!(); }
        }
    }

}

fn search_gaps(visited: &mut HashSet<usize>, gaps: &Vec<Gap>, start: &Gap, end: &Gap) -> bool {
    if visited.contains(&start.id) {
        return false;
    }
    if start.x == end.x && start.y == end.y {
        return true;
    }
    let theoretical_min_time = manhattan_distance(&(start.x, start.y), &(end.x, end.y));
    if theoretical_min_time > start.t as i32 {
        visited.insert(start.id);
        return false;
    }
    let possible_gaps = gaps.iter().filter(|gap| gap.x>=start.x-1 
        && gap.x<=start.x+1 
        && gap.y>=start.y-1 
        && gap.y<=start.y+1 
        && (gap.x==start.x || gap.y==start.y) // only horiz and vert and same moves allowed
        && gap.t==start.t-1);
            // once we've searched a branch we never have to search it again...
    for mut new_gap in possible_gaps {
        // if the gaps aren't sparse a bfs would be quicker
        if search_gaps(visited, gaps, &mut new_gap, end) {
            return true;  // found a way, can exit
        }
    }

    visited.insert(start.id);
    // nada
    false
}

#[test]
fn test_search_gaps_no_path() {
    let gaps = vec![Gap{id:0, x:0, y:-1, t:0}, Gap{id:1, x:0, y:0, t:0}, Gap{id:2, x:0, y:1, t:1}, Gap{id: 3, x:1, y:1, t:2}];
    let mut visited = HashSet::<usize>::new();
    assert_eq!(false, search_gaps(&mut visited, &gaps, &gaps[3], &gaps[0]));
} 

#[test]
fn test_search_gaps_path() {
    let gaps = vec![Gap{id:0, x:0, y:-1, t:0}, Gap{id:1, x:0, y:0, t:1}, Gap{id: 2, x:0, y:1, t:2}, Gap{id:3, x:1, y:1, t:3}];
    let mut visited = HashSet::<usize>::new();
    assert_eq!(true, search_gaps(&mut visited, &gaps, &gaps[3], &gaps[0]));
} 


fn process_blizzards(blizzards: &mut Vec<Blizzard>, width: usize, height: usize) -> usize {
    let mut gaps: Vec<Gap> = vec![Gap{id: 0, x:0,y:-1,t:0}];
    let mut visited = HashSet::<usize>::new();

    let mut tick = 1;

    if let Some(value) = _process_blizzards_guts(blizzards, width, height, &mut tick, &(0,-1), &(width as i32-1,height as i32), &mut gaps, &mut visited) {
        return gaps[value].t;
    }
 
    usize::MAX
}


fn process_blizzards_3(blizzards: &mut Vec<Blizzard>, width: usize, height: usize) -> usize {
    let mut gaps: Vec<Gap> = vec![Gap{id: 0, x:0,y:-1,t:0}];
    let mut visited = HashSet::<usize>::new();

    let mut tick = 1;
    let gap_id = _process_blizzards_guts(blizzards, width, height, &mut tick, &(0,-1), &(width as i32-1,height as i32), &mut gaps, &mut visited).unwrap();
    println!("Stage 1: tick: {tick}, gap: {:?}", gaps[gap_id]);
    gaps = vec![Gap{id:0, x:width as i32-1, y:height as i32, t:tick}];
    visited.clear();
    let gap_id: usize = _process_blizzards_guts(blizzards, width, height, &mut tick, &(width as i32-1,height as i32), &(0,-1), &mut gaps, &mut visited).unwrap();
    println!("Stage 1: tick: {tick}, gap: {:?}", gaps[gap_id]);
    gaps = vec![Gap{id:0, x:0, y:-1, t:tick}];
    visited.clear();
    if let Some(value) = _process_blizzards_guts(blizzards, width, height, &mut tick, &(0,-1), &(width as i32-1,height as i32),  &mut gaps, &mut visited) {
        return gaps[value].t;
    }
 
    usize::MAX
}

fn _process_blizzards_guts(blizzards: &mut Vec<Blizzard>, width: usize, height: usize, tick: &mut usize, start: &(i32,i32), end: &(i32,i32), gaps: &mut Vec<Gap>, visited: &mut HashSet<usize>) -> Option<usize> {
    loop {
        println!("Blizzard tick {tick}");
        // the question is how long to process. and I guess it's until there's a chain of gaps that reaches the exit?
        // the first one that connects is the one?
        tick_blizzards(blizzards, width, height);
            // quickest way to find gaps would be to plot the blizzards in 2d then scan for gaps. complexity n
        // but I'm going to be slow at first
        // always a gap at start and end

        for y in 0..height {
            for x in 0..width {
                if blizzards.iter().find(|bliz| bliz.x==x && bliz.y == y)==None {
                    gaps.push(Gap{id: gaps.len(), x: x as i32, y:y as i32, t:*tick});
                }
            }
        }   
        gaps.push(Gap{id:gaps.len(), x:start.0,y:start.1,t:*tick});
        gaps.push(Gap{id:gaps.len(), x:end.0, y:end.1, t:*tick});
        let destination_gap = gaps.last().unwrap();
        // probe to entrance
        // only bother if we can theoretically make it
        if search_gaps(visited, &gaps, &destination_gap, &gaps[0]) {
            return Some(destination_gap.id);
        }

        *tick += 1;
    }
}

#[test]
fn test_sample_input() {
    let (width, height, mut blizzards) = parse_input(&get_sample_input());
    let result = process_blizzards(&mut blizzards, width, height);
    assert_eq!(18, result);
}

fn manhattan_distance(vec2d0: &(i32,i32), vec2d1: &(i32,i32)) -> i32 {
    (vec2d1.0-vec2d0.0).abs()+(vec2d1.1-vec2d0.1).abs()
}

fn parse_input(input: &str) -> (usize,usize,Vec<Blizzard>) {
    let mut blizzards: Vec<Blizzard> = vec![];
    let height = input.lines().count()-2;
    let width = input.lines().nth(0).unwrap().len()-2;
    let lines = input.lines();
    for (yc, line) in lines.enumerate() {
        if yc == 0 { continue; }
        if yc > height { break; }

        for (xc, c) in line.chars().enumerate() {
            if xc == 0 { continue; }
            if xc >= line.len()-1 { break; }
            match c {
                '>' => {
                    blizzards.push(Blizzard{x: xc-1, y: yc-1, facing: 0});
                },
                'v' => {
                    blizzards.push(Blizzard{x: xc-1, y: yc-1, facing: 1}); 
                },
                '<' => {
                    blizzards.push(Blizzard{x: xc-1, y: yc-1, facing: 2});
                },
                '^' => {
                    blizzards.push(Blizzard{x: xc-1, y: yc-1, facing: 3});
                },
                '.' => {
                }
                _ => {
                    panic!();
                }
            }
        }
    }

    (width, height, blizzards)
}

#[test]
fn test_parse_sample() {
    let (width, height, blizzards) = parse_input(get_sample_input());
    assert_eq!(6, width);
    assert_eq!(4, height);
    assert_eq!(19, blizzards.len());
    assert_eq!(Blizzard{x: 0, y:0, facing:0}, blizzards[0]);
    assert_eq!(Blizzard{x: 5, y:3, facing:0}, blizzards[blizzards.len()-1]);
}

fn get_sample_input() -> &'static str {
"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#"
}

fn get_puzzle_input() -> &'static str {
"#.########################################################################################################################
#>^v<.<<^v^<><>>>^<vv>^>><<<^^.^v^<<v^v<v>^^>^^<^><^>.^>v<v<v.v<>vv<^^^vvv<.^v^<v^.^.>.<^<^>v^.v^v<>^vv^<<>>v^<<.<vvv^.<<#
#><^>><<vvv.v>^<..<v><v^^>>>v>>vv^^^<^^<><<>v>v<><><<^<<v>v.>.v>vv<>^>.^vv^^.<v<vv.<v<>>^>>><v<.>v>v^<v<>^<<<^v^<.v..v<^>#
#<v^.^>vv.^<^<>v^<^^^<^^>v<<>v><><^v^<v.v<^><<<^^^.>vv<.^<v<v<v^<v>^<v^<.><^^<>^v>..<v..>v^><>>^<.>.vv<<<.v.^^^^^>.<<.<v<#
#>.<<^>^v>>v<v<>>^v^>><>>v.^>.<>>^v<>.^<>^^^<vv<vv^^^v^^<<><v^^>>.v<v^v<.>^^.^>^v><vv<^^<^>>^v>^>^^v^v.<<^.vv<v..^^v.>>v>#
#>v^^>^^.>.^^<>><>v^^^<v>>>v^v^v^^<^<<^<<>><<^<<vv<<^>>>.<^v^><v<v>>v^>><vv..<v><vv^.>.v^vv>^><v.<vv>^vv^^>><v>v>v<v>>^><#
#>^<vvv^><^vvv>><^^vv^^>v<.v^<^<vv<><v<>^^^^<>^>v<^vv><v.<>>..>^^v<><>v.>>vv<><^^><>v<v<>vv^^>v><>>.v<v<>>>^^^v^v<<<^><^<#
#>^><v>>^.^><.^^^^<v>v<.v^>^^^<<<^<<v><><v>>>>v<v>vv<v><>.^><<^v<>^^v<.^^v<^>v<>v^.>.<<.>vv.^^vvv.<.v^>^><v^<^^^^>.^vv^^<#
#><v^vvv>^<v<>^>^<.><v<<<><v<<<^v^>^<v>>v<^<vv<v>v^..^>^^^^<.><<<>vvv>^vv^>v^<<>vv<<v^>^.v>.^<^^..>v<^<v<^>^^<><vv<.^v^.>#
#<^<><v.>.<<^>>v<^..<<.>>v<<^^>v.<v.<.v<.vv<^>^vvv>^>>v<.<>v^<<<<^<v>^<vv>v<<<^^>^^<<<>^>vv>>^.><>^^v^^><v.>v^^<>^^^^<v>>#
#<v.>^><^>>vv^vvvv^<>vv<<v<<v^v^v<><<^.^<><v^^<><.v<^>^v>^<^v^><^>v>^<<<v><v<<^^>v><v<<<<^.v^.<^<v<<>^>>><>vv<.^^..v>v<<>#
#<v<^>v<<^^>>v<^>^^>v>^>v.v<<<>v>><v<vv.v^<<.v<v>v<<^v<<.>^<v>^^>^..<>^^v.^><^vv><v<<>>vvv^^vv<<>^>^v^<^><<^v<.>^>><^^<^<#
#<<>>v^^>>^.<^^<^^^>^^>vv>>>>>v^vv><.v>v>v<^>><^<<v><^>^>^vv^vv^<v<<<v>><v.v>^^<<.^vvv<>vv<><^^^vv>v^.><^<^vv><<.vv^^^.v<#
#<.<v><vv>^<>>^>.^<<v>..^vvv^^v>^<>>v<<<..^vv>^<...^<<>.<^>^<.v><v.>^^.vv<<^<v.>v^v^^>.^^vv.^v^><<>^^^^.v<..>v^<>^<>>vvv>#
#<^<<v^v<^<.^v^>v^<^<.>^^v^<^>.<v>v>^^^.v><v><vv<<v<>v.<^><>.><vv><<^^.vv<.<^.vvvvvv^^^^<vvv.^<.>v.^v>^v>><v^v.>v^^v.<.^<#
#.<<^.<v<<^^<>v.<.>v.^^^<<^v>.v<>v<...>^<v.<v<<><>.v<^^^>.^v>vv^<^.>.^>>^>v.>v>.^^<v><v^<><^^v<v^^<<<>^.>v><<v<<v>.<^v<v>#
#>>><<^v<^>v^>><^^.<v<^<<<^.<v^>v<<^^^>>^^.^v^>v^<>^v.>^><^>.^^v>><<>v<v><^vv<><^v>.v><^><<v>.^>v^^<<^<>.>v<v^^v<v.>v..>>#
#.v>^^.<v.^^>><^v<vv>v<^<>><v>^^v<<v>>.<<v^v<^.v.<^>^vvv<vv^v>><>^v<^><v..<^<vv<<v<.<^^^<<>v^^v>v>>^^^>vvvv<^^<>.<^^^><^.#
#<^^<v<^<>>vv>v<<vv^v<^.<>><<v<<v>^^^^>^<^>>^vv^^<^vv^>vvv>v>^v.><^>^^>.^^v.<<v.v<^<<^v><vv^^^v^>><.^<>v><v<vv^^>>v^>>..>#
#<^>v^^vv>^><^^^^^^>>.v<^.<^<>^><^.v<v<>>v.^.v^<vv^^<<<><..>>><.v<vv>^v^.<>>v^v.<vv><v^<vv<<v<><.<<^^v><.>v<<>v<<^^>>vvv>#
#.><<>>><^vv^v.>.>.^.<>>^^^v^^><>v<><>>>><v<<v^<>^v^v>>^^v^.<>v>vvv>v>>.<>>^>>>>^>^>v<.><vv>>.>>v<>><<^><v.^<<..>.vvv^v<<#
#>^>^<^^v>^^^>v>.v><>^>^v^^.v.>>v>>v^^^>>>v><>.<vv.<<<.<^^<^>^>v<v>^<^v<><>><>.^vv>^>.^>>>^><<^>^v<<^<vv<vv^.^^<vv>>v.<>>#
#><<><v<<vv>^>.^.<<.>.<.v>^>><^>v<>v^.<><^v^>>v<^^<.<^<>.<vv<<>vvvv>vv>v.<<^v<.>v>^^<v><>.>v>^<^<^^..>v<v>v.^<^<^<vv<v><>#
#>>^^<^<v^<v<vv^>..^v.^<^>^.v><^vv^..v.<>v<<.<v..v>v>vvv^>^<v>vv.^.>^.^^>v<^>v>v><>>^^>.v^<<.<^>v^vv><.v..<.><<v<.<<.<vv>#
#><<>^<>><^.^^><v<<^<^^>>v><v.vv><v<<<<.<^<v>v><>v<<^<<>>^>^vv>v<v.vv>v>v>>>><..v^^vv>><<>>vv><>>.>v<<>v<v^vv<v<<v^>>v<^.#
#>.<<><<vv<^.v<v<v<^>>v><v.^>^v.<^v.^^^<<<<.<<v^v>^<.>^vv^<<^^>.v>>>.v^v<v<v><^>v.>>^^>^v>vv<>vv^^<.<.v>^^v>.^^v<^v^>^^v>#
########################################################################################################################.#"
}