fn main() {
    let result = do_the_thing(puzzle_input());
    println!("answer: {result}");
}

// too lazy to look up how to create an OO vec2 with +/- 
// or to grab one from a crate, even
fn addp(point0: &(i64, i64), point1: &(i64,i64)) -> (i64, i64) {
    (point0.0+point1.0, point0.1+point1.1)
}

fn subp(point0: &(i64, i64), point1: &(i64,i64)) -> (i64, i64) {
    (point0.0-point1.0, point0.1-point1.1)
}

fn do_the_thing(input: &str) -> usize {
    follow_the_path(&input).len()
}

fn follow_the_path(input: &str) -> Vec<(i64, i64)> {
    let mut head_pos = (0,0);
    let mut tail_pos = (0,0);
    let mut visited : Vec<(i64, i64)> = vec![];
    visited.push(tail_pos);
    for line in input.lines() {
        let mut split = line.split(' ');
        let dir = match split.next() {
            Some("R") => { (1,0) }
            Some("U") => { (0,-1) }
            Some("L") => { (-1,0) }
            Some("D") => { (0,1) }
            Some(_) => { assert!(false); (0,0) }
            None => { assert!(false); (0,0) }
        };
        let count = split.next().unwrap().parse::<i64>().unwrap();
        for _ in 0..count {
            head_pos = addp(&head_pos, &dir);
            let mut distance = subp(&head_pos, &tail_pos);
            let mut delta = determine_delta(&distance);
            tail_pos = addp(&tail_pos, &delta);
            assert!(1 >= chess_distance(&subp(&head_pos, &tail_pos)));
            assert!(0 <= chess_distance(&subp(&head_pos, &tail_pos)));
            visited.push(tail_pos);
        }
    }
   visited.sort();
   visited.dedup();

    visited
}

fn chess_distance(distance: &(i64, i64)) -> i64 {
    std::cmp::max(distance.0.abs(), distance.1.abs())
}

fn determine_delta(distance: &(i64, i64)) -> (i64,i64) {
    if chess_distance(distance) > 1 {
        let dx = if distance.0.abs() >= 1 { distance.0 / distance.0.abs() } else { 0 };
        let dy = if distance.1.abs() >= 1 { distance.1 / distance.1.abs() } else { 0 };

        (dx, dy)
    }
    else {
        (0,0)
    }
}

#[test]
fn test_deltas() {
    assert_eq!((0,0), determine_delta(&(-1,-1)));
    assert_eq!((0,0), determine_delta(&(1,0)));
    assert_eq!((1,0), determine_delta(&(2,0)));
    assert_eq!((0,1), determine_delta(&(0,2)));
    assert_eq!((1,-1), determine_delta(&(1,-2)));
    assert_eq!((1,-1), determine_delta(&(2,-1)));
    assert_eq!((-1,-1), determine_delta(&(-1,-2)));
    assert_eq!((-1,-1), determine_delta(&(-2,-1)));
    assert_eq!((-1,1), determine_delta(&(-1,2)));
    assert_eq!((-1,1), determine_delta(&(-2,1)));
    assert_eq!((1,1), determine_delta(&(1,2)));
    assert_eq!((1,1), determine_delta(&(2,1)));}

#[test]
fn i_dont_know_where_im_going() {
    let sample_input =
"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    assert_eq!(13, do_the_thing(sample_input));
}

#[test]
fn nobody_said_it_was_a_single_character() {
    let sample_input: &str = "R 100";
    assert_eq!(100, do_the_thing(sample_input));
}

fn puzzle_input() -> &'static str {
"U 1
D 1
L 1
U 2
L 1
D 2
L 2
U 2
L 1
R 2
U 2
D 1
R 2
U 1
R 1
U 1
R 1
U 2
L 1
U 2
D 1
R 1
U 2
D 1
U 2
D 2
U 2
D 1
R 1
D 2
L 2
R 2
L 1
R 2
D 2
L 2
U 1
D 2
R 2
L 2
R 2
D 2
R 1
D 1
L 1
D 1
L 2
D 1
R 1
L 1
U 2
L 2
U 1
D 1
U 1
D 1
L 2
D 2
L 2
U 1
R 2
D 1
U 1
R 2
L 1
R 1
L 1
R 2
L 1
U 2
R 1
L 1
U 1
D 2
R 1
D 2
L 1
U 2
D 2
L 2
U 1
L 1
D 1
L 2
U 2
R 2
U 1
D 2
L 1
D 2
R 1
L 1
R 1
D 2
U 1
R 1
L 1
U 2
R 2
L 1
R 2
L 1
R 1
L 2
U 1
R 2
D 2
U 1
D 1
L 2
D 1
R 3
D 1
L 3
D 1
R 2
D 1
R 2
U 3
L 2
U 2
D 2
L 3
R 3
U 3
R 3
L 1
R 2
L 2
R 3
U 2
D 3
L 3
D 1
R 2
U 1
D 2
U 2
L 1
U 2
R 2
D 2
L 3
D 1
L 1
R 1
D 3
L 2
R 2
L 2
R 2
D 1
R 2
L 3
U 1
R 2
D 2
R 1
D 2
R 2
L 3
U 1
L 2
U 1
D 3
L 1
U 1
D 3
U 2
L 1
U 3
D 2
R 2
U 3
L 3
D 1
U 3
R 1
U 2
L 2
D 1
U 1
D 3
R 1
D 3
U 3
L 3
R 2
D 3
L 2
U 3
L 1
D 3
L 2
D 1
R 3
L 1
R 3
U 2
L 3
D 1
R 2
L 1
U 1
L 3
R 3
U 2
D 2
U 2
R 1
L 3
U 3
L 1
R 3
D 3
L 2
D 3
U 1
L 3
D 3
R 2
U 3
D 1
R 1
L 1
U 4
L 2
U 3
R 3
U 1
D 2
R 4
U 1
L 1
U 3
R 2
D 4
U 3
D 1
U 1
L 1
U 3
R 4
L 1
R 4
L 1
R 2
L 4
D 1
L 4
R 2
D 2
R 2
U 1
D 2
L 4
D 1
R 3
D 2
U 3
D 4
L 3
U 2
R 4
U 4
D 2
L 2
D 1
U 3
L 2
R 4
L 1
U 3
L 2
R 3
U 1
R 4
D 3
L 3
D 1
U 4
L 4
D 3
R 1
L 1
D 2
L 1
R 4
D 2
L 3
D 4
L 4
U 2
D 1
U 4
D 4
U 2
D 2
R 4
L 2
D 4
U 2
L 1
D 4
U 3
D 3
R 2
D 1
U 4
R 2
U 3
D 1
R 1
L 1
D 1
L 3
R 4
U 2
L 4
D 3
L 2
R 4
L 3
U 3
L 1
D 4
R 1
L 2
D 1
R 2
L 1
R 4
L 1
R 3
U 2
D 1
R 2
L 1
D 5
U 1
D 2
R 5
U 4
L 1
D 2
R 3
U 2
D 1
L 4
D 5
R 4
D 2
R 3
D 5
L 2
R 5
U 2
L 4
R 4
U 3
D 3
R 1
U 4
D 2
U 2
L 3
D 4
L 4
D 2
U 5
R 1
U 4
L 2
U 3
R 5
D 4
U 5
R 2
D 5
R 3
U 1
R 4
L 3
D 1
L 1
R 5
L 5
D 2
U 5
D 5
R 5
U 3
D 4
R 4
U 2
L 1
D 3
U 3
R 4
U 4
D 1
L 1
R 2
L 5
R 1
L 3
D 1
U 5
R 1
U 4
D 5
L 5
R 2
L 4
U 1
L 5
D 2
R 2
D 1
R 4
D 5
L 3
D 3
L 4
U 5
L 3
R 4
D 4
R 2
U 4
R 4
D 3
U 3
L 3
R 1
D 4
U 1
R 1
D 1
R 4
D 3
L 5
U 1
D 1
U 5
L 5
D 3
U 6
D 5
U 5
D 1
L 5
D 4
L 2
R 6
U 3
R 6
D 6
U 5
D 3
L 1
R 2
U 2
D 4
U 6
D 1
U 1
D 2
U 3
L 1
U 6
D 4
L 6
U 5
D 4
U 3
R 6
U 5
R 2
U 3
D 6
L 5
U 2
D 4
L 4
D 4
R 6
U 4
L 3
U 4
R 6
D 1
U 4
R 1
L 1
U 3
D 6
L 2
D 2
U 3
R 5
D 1
R 3
U 6
L 2
R 2
U 4
L 5
D 4
U 4
R 4
D 5
L 1
R 2
U 4
L 5
D 6
R 2
L 2
D 6
L 1
R 1
U 3
L 4
D 4
U 5
L 3
D 2
U 2
L 1
R 3
L 4
U 6
L 2
U 2
D 3
L 5
R 1
D 1
U 1
D 6
L 6
D 6
U 2
D 6
L 3
U 2
L 6
D 2
L 4
R 5
D 2
U 3
R 1
D 4
U 2
R 5
D 6
R 7
U 5
L 6
U 5
L 2
R 1
L 3
R 6
U 2
L 3
U 7
R 1
D 2
U 4
R 2
L 3
D 2
U 3
D 7
U 2
L 4
R 6
D 5
R 7
U 1
D 3
U 7
R 2
U 4
D 2
L 7
D 2
U 4
D 3
R 4
U 6
L 7
R 7
L 3
D 3
U 1
L 5
U 5
R 2
U 7
L 2
U 2
D 1
R 3
U 1
L 4
U 4
D 4
U 1
L 2
R 5
D 4
L 4
U 3
R 1
D 2
R 7
D 2
U 5
D 1
U 4
R 7
L 4
R 6
L 7
R 2
D 6
U 7
R 1
D 7
L 3
D 2
L 7
R 4
L 2
D 3
R 2
U 6
L 6
U 2
R 2
U 7
R 7
D 5
U 6
D 7
R 6
D 7
R 2
U 7
L 6
R 6
U 1
D 2
L 1
R 3
D 4
U 3
R 6
D 3
U 3
D 5
L 5
U 4
R 6
D 4
U 7
L 3
U 6
R 7
D 8
R 5
L 4
D 2
L 4
U 8
R 2
D 6
U 4
D 6
U 1
D 7
U 7
L 4
R 2
L 7
D 8
L 2
D 7
R 4
L 3
D 8
R 7
D 8
U 7
D 4
L 1
U 1
D 1
L 3
D 4
L 7
U 8
D 2
R 8
L 1
D 8
L 1
D 4
U 1
R 8
D 2
L 6
D 4
U 4
R 1
U 8
D 1
U 8
R 5
D 8
L 7
D 8
U 6
D 2
R 6
U 8
L 5
U 2
R 4
L 2
D 4
L 5
D 4
R 7
D 5
L 4
R 5
L 4
D 6
L 2
R 1
U 8
R 6
D 8
U 2
R 3
U 5
R 2
D 2
R 2
D 4
U 5
D 2
L 2
U 6
D 3
R 8
U 7
L 7
D 2
U 2
L 8
D 5
U 1
D 7
L 6
D 3
U 3
D 5
R 2
U 7
L 3
U 8
R 9
D 9
U 2
D 7
L 1
R 7
L 8
U 3
D 4
L 2
R 2
D 6
U 6
R 5
U 4
D 6
R 7
D 4
U 1
D 6
R 8
L 6
R 7
D 4
U 5
D 2
R 9
U 5
L 9
D 8
U 1
L 7
R 4
L 8
R 6
U 8
D 9
U 8
D 1
U 6
L 2
R 2
L 9
D 2
R 8
L 1
U 1
L 1
D 8
U 7
L 9
U 4
R 2
U 6
D 5
R 1
L 2
R 1
L 7
R 4
D 2
L 1
U 1
D 3
U 4
R 2
D 7
L 4
U 1
L 1
D 8
U 7
D 6
U 8
L 3
D 2
R 8
U 3
D 5
U 5
D 3
U 2
L 1
R 8
D 2
L 4
U 6
D 2
L 7
U 9
L 3
D 1
U 7
L 4
U 3
L 8
R 9
D 2
L 8
U 9
R 3
D 3
U 7
D 9
R 1
U 8
R 4
L 1
D 9
L 6
U 6
R 3
D 4
U 5
R 5
L 3
R 3
L 7
U 7
R 3
D 4
R 3
L 9
D 2
U 1
L 2
D 10
R 4
D 5
U 1
R 9
L 4
R 7
D 10
U 8
L 8
R 7
L 3
D 6
L 8
D 3
U 7
D 2
L 6
U 1
R 5
D 2
L 7
D 3
L 5
U 10
R 2
U 7
R 7
L 3
U 4
L 5
D 9
L 10
U 5
L 8
R 4
D 6
U 9
R 3
L 4
R 10
L 8
U 5
D 10
L 10
U 7
L 7
R 6
U 2
D 10
R 1
L 9
U 2
R 9
U 10
D 5
U 3
L 3
D 9
R 6
U 9
R 3
L 4
D 2
R 6
D 3
U 7
D 6
U 6
L 5
D 5
R 3
D 5
R 5
D 6
R 9
L 9
D 3
L 3
U 4
L 9
D 8
L 1
D 7
L 6
U 9
R 5
L 8
D 4
R 4
D 9
R 7
D 2
L 4
U 10
R 6
D 2
R 9
U 6
D 2
R 7
L 6
U 8
D 6
U 5
L 1
R 3
D 9
U 2
D 9
L 2
R 6
L 4
U 8
L 10
D 11
L 2
R 6
U 9
L 7
D 1
U 10
R 3
L 10
U 1
R 7
L 2
U 3
D 4
U 10
D 8
R 6
D 2
U 7
L 1
U 5
D 3
U 11
D 1
R 9
U 7
R 5
L 6
U 10
D 7
U 2
D 8
R 10
D 10
U 5
D 2
U 4
R 2
L 10
U 8
L 5
R 3
U 6
R 6
U 2
R 8
L 2
D 6
R 11
D 3
U 6
D 6
R 1
U 4
D 8
L 5
R 11
D 3
U 6
R 1
L 7
D 8
R 7
L 6
R 4
L 1
U 4
R 2
L 3
R 4
U 2
R 8
L 7
U 1
D 11
L 5
U 10
D 3
L 4
D 9
U 8
R 10
L 10
R 4
D 1
U 1
L 6
D 11
L 9
U 1
D 1
L 10
U 11
R 4
L 9
D 5
U 6
L 3
R 5
U 4
R 9
D 11
L 5
U 6
L 2
D 8
R 2
L 10
U 9
D 8
R 10
D 5
U 6
R 2
L 11
D 1
R 4
D 6
U 3
D 12
L 3
D 9
R 2
L 9
U 2
D 3
L 2
D 3
L 8
D 9
U 10
D 8
R 11
U 9
L 4
U 8
L 6
D 12
L 5
U 2
L 5
R 5
U 7
L 4
U 1
L 10
D 6
R 3
L 4
U 1
L 3
D 8
U 1
D 6
L 6
R 9
D 7
U 12
R 12
U 12
L 3
R 4
L 3
D 7
U 3
D 12
U 8
L 10
D 11
R 3
U 3
L 11
U 3
R 11
U 6
R 10
U 11
D 4
R 8
L 2
U 3
D 9
L 3
R 7
D 10
L 11
R 9
L 1
D 3
R 1
D 2
L 4
R 4
L 11
D 7
L 7
U 6
L 12
R 2
L 8
D 9
U 5
L 11
R 1
U 12
R 5
L 5
D 11
R 12
L 10
R 8
D 3
U 9
R 6
L 3
U 13
D 13
L 9
U 6
D 6
U 9
L 9
R 4
L 13
R 2
U 10
L 4
U 7
D 8
U 8
L 3
U 10
L 6
D 1
R 10
U 2
R 12
L 5
R 7
L 11
D 1
R 2
D 4
R 5
L 5
U 2
D 10
U 5
R 6
U 10
R 3
U 13
R 9
D 2
L 4
R 11
U 3
D 6
R 10
D 4
L 1
U 4
D 1
U 12
D 5
L 8
U 3
R 11
U 12
L 11
R 8
L 4
R 12
D 3
L 11
R 1
L 12
R 2
D 10
R 10
L 4
U 4
D 4
U 7
D 6
L 9
R 12
L 7
D 2
R 11
L 5
D 8
L 3
D 1
L 5
R 7
L 10
U 9
R 11
D 13
L 12
R 9
D 1
L 2
D 13
L 5
R 10
D 12
R 10
L 13
R 8
L 5
R 9
U 1
R 3
D 4
U 6
D 2
L 6
U 1
D 8
U 9
D 2
R 9
D 10
U 12
L 14
U 4
L 6
D 3
U 7
R 5
D 13
R 1
D 11
U 10
L 1
U 3
L 9
U 2
L 14
D 14
L 11
R 9
L 12
R 10
D 14
R 3
D 10
R 3
U 3
D 13
L 6
D 5
L 7
U 14
D 7
U 13
L 12
U 13
R 7
L 4
D 8
R 3
U 7
L 1
D 14
U 7
D 12
U 12
D 5
L 3
U 5
D 13
U 12
L 9
R 8
D 8
R 4
U 11
R 10
D 8
L 6
R 7
D 12
U 10
L 10
D 8
L 9
R 11
D 8
L 10
D 14
L 6
R 6
L 10
R 10
U 14
R 4
U 1
R 3
U 11
L 3
R 4
D 11
U 12
D 6
L 5
R 3
D 4
U 4
D 10
R 11
D 10
L 2
D 7
L 6
D 2
R 9
L 1
R 3
D 1
U 10
D 14
L 9
D 8
U 7
D 12
R 2
U 3
D 9
L 5
D 8
R 15
U 6
L 10
R 1
D 3
U 5
R 10
U 5
L 1
U 9
D 9
R 8
D 7
U 1
R 12
U 15
D 2
U 9
L 7
D 11
R 10
U 8
L 8
D 2
L 1
R 2
L 11
D 1
R 8
D 3
L 3
D 15
R 9
U 3
L 13
U 3
D 6
L 7
R 8
D 10
U 6
R 15
D 8
U 1
D 2
R 6
U 2
R 5
L 8
D 12
U 1
L 4
D 1
R 5
L 14
R 9
L 9
U 9
L 11
U 8
R 2
U 4
D 2
R 12
U 13
D 10
L 10
D 5
L 9
R 10
L 10
R 14
L 6
D 8
R 4
D 2
L 11
R 1
U 11
R 9
L 13
D 9
L 14
R 15
U 15
L 6
U 3
D 15
L 5
D 4
U 2
L 4
D 7
L 14
R 2
L 5
U 8
L 3
U 10
R 7
L 12
U 13
L 14
R 3
U 13
L 13
U 16
D 10
R 10
U 8
L 4
R 9
D 4
L 4
U 4
D 10
L 11
R 1
L 11
U 11
R 2
U 11
R 7
L 6
D 6
U 11
L 5
U 8
R 8
L 7
R 11
D 2
R 4
L 16
U 10
D 2
U 2
R 13
U 7
L 2
R 7
U 13
R 3
L 7
R 10
D 10
L 3
R 13
L 10
U 11
L 8
R 14
U 7
D 15
L 9
R 6
L 6
R 16
D 10
L 13
R 5
L 7
D 1
R 3
D 16
U 11
R 16
L 15
U 10
R 6
D 7
R 3
D 4
L 1
D 11
R 14
U 16
L 14
U 11
L 16
R 10
L 10
U 16
R 12
L 6
U 3
R 5
L 10
U 3
D 16
L 15
U 8
L 7
R 12
L 2
U 15
L 1
D 9
U 7
D 8
L 2
R 14
L 9
D 2
U 7
R 2
D 8
U 9
L 4
D 9
U 6
L 13
D 15
L 2
D 15
L 10
R 7
D 10
L 4
U 16
D 7
R 8
D 13
R 4
L 5
U 8
R 15
U 10
R 17
U 8
L 5
D 11
L 2
U 13
D 12
L 3
U 4
R 16
D 4
L 5
D 3
L 16
R 16
L 16
U 10
L 13
R 14
L 7
U 3
D 15
U 11
D 8
L 3
D 3
U 6
D 14
U 1
L 3
D 7
U 14
R 17
U 1
R 16
U 11
R 17
D 10
R 8
D 5
L 14
R 1
L 3
D 17
R 4
U 9
D 1
U 13
R 4
L 3
U 9
R 11
U 5
R 5
D 14
U 12
L 7
R 2
L 15
D 17
R 16
D 3
L 8
D 11
L 1
U 2
D 7
L 11
R 14
L 5
R 17
U 11
L 12
D 14
L 11
D 13
L 12
R 1
D 11
R 2
L 2
D 17
U 5
R 4
D 14
L 14
D 9
L 13
D 12
R 2
L 10
R 7
U 11
L 16
U 12
R 9
U 1
R 17
U 4
L 7
U 1
L 7
R 1
D 5
L 8
U 11
D 4
U 5
L 15
D 3
L 6
R 4
D 1
R 9
L 18
R 6
L 6
U 16
R 16
D 16
L 1
U 13
R 16
D 6
R 15
U 8
D 14
U 5
D 16
U 14
L 13
R 18
L 6
U 5
R 14
U 18
L 10
D 3
U 5
D 15
R 1
L 12
R 1
L 9
U 8
L 14
U 9
L 9
D 5
R 3
L 8
D 18
L 13
D 15
R 15
U 4
D 17
L 17
U 9
R 1
L 13
R 8
D 9
U 15
L 11
D 6
L 14
D 6
L 6
U 8
D 18
U 3
R 1
L 18
U 16
L 11
D 6
L 16
U 13
R 15
U 10
D 4
L 3
D 1
U 1
L 9
U 16
L 8
R 8
D 2
U 13
R 15
U 9
L 7
D 11
R 17
D 8
U 9
R 7
D 7
R 6
L 10
U 5
L 3
D 4
L 1
U 15
R 8
U 17
L 2
R 9
U 15
D 7
R 3
L 10
U 18
L 16
R 15
L 1
R 18
L 8
D 15
U 6
D 11
R 11
U 7
R 9
L 10
D 7
L 7
U 13
R 14
U 9
D 13
R 11
U 17
D 13
R 17
D 19
L 9
U 12
L 16
D 9
U 15
R 5
D 13
L 17
D 3
R 18
D 12
U 9
D 13
R 7
U 18
R 1
U 10
R 11
U 15
L 5
D 15
R 10
D 18
L 8
U 4
D 15
R 11
U 11
D 17
U 2
R 6
U 13
R 8
D 9
U 19
D 8
U 5
D 10
U 18
L 7
D 2
R 13
D 11
R 14
L 14
R 11
U 11
D 13
L 4
R 2
U 18
L 4
D 3
L 15
U 6
R 16
L 13
D 3
U 14
R 5
U 15
L 7
R 5
U 11
L 16
R 5
L 1
U 9
D 7
R 8
L 11
R 6
L 16
U 19
R 2
D 10
L 15
D 7
U 19
D 1
U 5
L 3"
}