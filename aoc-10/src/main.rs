fn main() {
    let puzzle_input = get_puzzle_input();
    let answer
        = signal_strength(&puzzle_input, 20) 
        + signal_strength(&puzzle_input, 60)
        + signal_strength(&puzzle_input, 100)
        + signal_strength(&puzzle_input, 140)
        + signal_strength(&puzzle_input, 180)
        + signal_strength(&puzzle_input, 220);
    println!("answer {answer}");
}

fn signal_strength(input: &str, cycle: i64) -> i64 {
    let mut tick = 0;
    let mut x: i64 = 1;
    let mut lines = input.lines();
    while tick < cycle {
        let line = lines.next().unwrap();
        let mut split = line.split(' ');
        match split.next() {
            Some("addx") => {
                let v = split.next().unwrap().parse::<i64>().unwrap();
                tick += 2;
                if tick < cycle {
                    x += v;
                }
            }
            Some("noop") => {
                tick += 1;
            }
            Some(_) => {
                assert!(false);
            }
            None => {
                assert!(false);
            }
        }
    }
    x * cycle
}

#[test]
fn test_signal_strength() {
    let input = 
"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
    assert_eq!(420, signal_strength(&input, 20));
    assert_eq!(1140, signal_strength(&input, 60));
    assert_eq!(1800, signal_strength(&input, 100));
    assert_eq!(2940, signal_strength(&input, 140));
    assert_eq!(2880, signal_strength(&input, 180));
    assert_eq!(3960, signal_strength(&input, 220));
}

fn get_puzzle_input() -> &'static str {
"addx 1
noop
addx 2
addx 5
addx 2
noop
noop
noop
addx 5
noop
noop
addx 1
addx 2
addx -5
addx 12
addx 1
addx 4
addx 2
noop
addx -1
addx 4
noop
noop
addx -37
addx 21
addx -13
addx -3
noop
addx 3
addx 2
addx 5
addx -2
addx 7
addx -2
addx 2
addx 11
addx -4
addx 3
noop
addx -18
addx 7
addx 14
addx 2
addx 5
addx -39
addx 1
addx 5
noop
noop
noop
addx 1
addx 4
noop
addx 12
addx 10
addx -17
addx 5
addx -17
addx 14
addx 6
noop
addx 3
addx 7
noop
noop
addx 2
addx 3
noop
addx -40
addx 40
addx -33
addx -2
noop
addx 3
addx 2
addx 5
addx -7
addx 8
noop
addx 6
addx 8
addx -11
addx 8
noop
addx -19
addx 27
noop
addx -2
addx 4
noop
addx -10
addx -27
noop
noop
addx 7
addx 4
addx -3
addx 2
addx 5
addx 2
addx -4
addx -3
addx 10
addx 15
addx -8
addx 2
addx 3
addx -2
addx 5
addx 2
addx 2
addx -39
addx 1
addx 3
addx 3
addx 3
noop
addx 2
addx 1
addx 4
addx -1
addx 2
addx 4
addx 1
noop
noop
addx 2
addx 5
addx 3
noop
noop
addx -27
addx 29
noop
addx 3
noop
noop"
}