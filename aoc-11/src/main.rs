#[derive(Debug, Clone, PartialEq)]
struct Monkey {
    items: Vec<i32>,
    operation: fn(i32) -> i32,
    divisible_by: i32,
    true_monkey_idx: usize,
    false_monkey_idx: usize,
    inspected: usize,
}

fn main() {
    println!("Hello, world!");
    let mut monkeys = get_puzzle_input();
    for i in 0..20 {
        monkeys = do_all_the_monkeys(&monkeys);
    }
    let inspected: Vec<usize> = monkeys.iter().map(|monkey| monkey.inspected).collect();
    println!("inspected: {:?}", inspected);
}

fn do_all_the_monkeys(monkey_states: &Vec<Monkey>) -> Vec<Monkey> {
    let mut new_monkey_states = monkey_states.clone();
    for i in 0..monkey_states.len() {
        new_monkey_states = do_monkey(&new_monkey_states, i);
    }

    new_monkey_states
}

#[test]
fn test_do_all_the_monkeys() {
    let new_states = do_all_the_monkeys(&get_sample_input());
    assert_eq!(vec![20, 23, 27, 26], new_states[0].items);
    assert_eq!(vec![2080, 25, 167, 207, 401, 1046], new_states[1].items);
    assert_eq!(Vec::<i32>::new(), new_states[2].items);
    assert_eq!(Vec::<i32>::new(), new_states[3].items);

    let new_states = do_all_the_monkeys(&new_states);
    assert_eq!(vec![695, 10, 71, 135, 350], new_states[0].items);
    assert_eq!(vec![43, 49, 58, 55, 362], new_states[1].items);
    assert_eq!(Vec::<i32>::new(), new_states[2].items);
    assert_eq!(Vec::<i32>::new(), new_states[3].items);

    let new_states = do_all_the_monkeys(&new_states); // 3
    let new_states = do_all_the_monkeys(&new_states);
    let new_states = do_all_the_monkeys(&new_states); // 5
    let new_states = do_all_the_monkeys(&new_states);
    let new_states = do_all_the_monkeys(&new_states); // 7
    let new_states = do_all_the_monkeys(&new_states);
    let new_states = do_all_the_monkeys(&new_states); // 9
    let new_states = do_all_the_monkeys(&new_states); // 10
    assert_eq!(vec![91, 16, 20, 98], new_states[0].items);
    assert_eq!(vec![481, 245, 22, 26, 1092, 30], new_states[1].items);
    assert_eq!(Vec::<i32>::new(), new_states[2].items);
    assert_eq!(Vec::<i32>::new(), new_states[3].items);

    let new_states = do_all_the_monkeys(&new_states);
    let new_states = do_all_the_monkeys(&new_states); // 12
    let new_states = do_all_the_monkeys(&new_states);
    let new_states = do_all_the_monkeys(&new_states); // 14
    let new_states = do_all_the_monkeys(&new_states);
    let new_states = do_all_the_monkeys(&new_states); // 16
    let new_states = do_all_the_monkeys(&new_states);
    let new_states = do_all_the_monkeys(&new_states); // 18
    let new_states = do_all_the_monkeys(&new_states);
    let new_states = do_all_the_monkeys(&new_states);
    assert_eq!(vec![10, 12, 14, 26, 34], new_states[0].items);
    assert_eq!(vec![245, 93, 53, 199, 115], new_states[1].items);
    assert_eq!(Vec::<i32>::new(), new_states[2].items);
    assert_eq!(Vec::<i32>::new(), new_states[3].items);

    assert_eq!(101, new_states[0].inspected);
    assert_eq!(95, new_states[1].inspected);
    assert_eq!(7, new_states[2].inspected);
    assert_eq!(105, new_states[3].inspected);
}

// we can copy vecs, it'll be fast enough for this
fn do_monkey(monkey_states: &Vec<Monkey>, monkey_idx: usize) -> Vec<Monkey> {
    let mut mut_monkey_states: Vec<Monkey> = monkey_states.clone();
    let my_monkey = &monkey_states[monkey_idx];
    for item in my_monkey.items.iter() {
        let worry_level = (my_monkey.operation)(*item);
        let worry_level = worry_level / 3;
        if worry_level % my_monkey.divisible_by == 0 {
            mut_monkey_states[my_monkey.true_monkey_idx]
                .items
                .push(worry_level);
        } else {
            mut_monkey_states[my_monkey.false_monkey_idx]
                .items
                .push(worry_level);
        }
    }

    mut_monkey_states[monkey_idx].inspected += mut_monkey_states[monkey_idx].items.len();
    mut_monkey_states[monkey_idx].items.clear();

    mut_monkey_states
}

#[test]
fn test_do_monkey() {
    let old_monkey_states = get_sample_input();
    let new_monkey_states = do_monkey(&old_monkey_states, 0);
    assert_eq!(vec![74, 500, 620], new_monkey_states[3].items);
    assert_eq!(Vec::<i32>::new(), new_monkey_states[0].items);
    assert_eq!(2, new_monkey_states[0].inspected);
}

fn get_sample_input() -> Vec<Monkey> {
    vec![
        Monkey {
            // 0:
            items: vec![79, 98],
            operation: |x| x * 19,
            divisible_by: 23,
            true_monkey_idx: 2,
            false_monkey_idx: 3,
            inspected: 0,
        },
        Monkey {
            items: vec![54, 65, 75, 74],
            operation: |x| x + 6,
            divisible_by: 19,
            true_monkey_idx: 2,
            false_monkey_idx: 0,
            inspected: 0,
        },
        Monkey {
            items: vec![79, 60, 97],
            operation: |x| x * x,
            divisible_by: 13,
            true_monkey_idx: 1,
            false_monkey_idx: 3,
            inspected: 0,
        },
        Monkey {
            items: vec![74],
            operation: |x| x + 3,
            divisible_by: 17,
            true_monkey_idx: 0,
            false_monkey_idx: 1,
            inspected: 0,
        },
    ]
}

fn get_puzzle_input() -> Vec<Monkey> {
    vec![
        Monkey {
            items: vec![54, 61, 97, 63, 74],
            operation: |x| x * 7,
            divisible_by: 17,
            true_monkey_idx: 5,
            false_monkey_idx: 3,
            inspected: 0,
        },
        Monkey {
            items: vec![61, 70, 97, 64, 99, 83, 52, 87],
            operation: |x| x + 8,
            divisible_by: 2,
            true_monkey_idx: 7,
            false_monkey_idx: 6,
            inspected: 0,
        },
        Monkey {
            items: vec![60, 67, 80, 65],
            operation: |x| x * 13,
            divisible_by: 5,
            true_monkey_idx: 1,
            false_monkey_idx: 6,
            inspected: 0,
        },
        Monkey {
            items: vec![61, 70, 76, 69, 82, 56],
            operation: |x| x + 7,
            divisible_by: 3,
            true_monkey_idx: 5,
            false_monkey_idx: 2,
            inspected: 0,
        },
        Monkey {
            items: vec![79, 98],
            operation: |x| x + 2,
            divisible_by: 7,
            true_monkey_idx: 0,
            false_monkey_idx: 3,
            inspected: 0,
        },
        Monkey {
            items: vec![72, 79, 55],
            operation: |x| x + 1,
            divisible_by: 13,
            true_monkey_idx: 2,
            false_monkey_idx: 1,
            inspected: 0,
        },
        Monkey {
            items: vec![63],
            operation: |x| x + 4,
            divisible_by: 19,
            true_monkey_idx: 7,
            false_monkey_idx: 4,
            inspected: 0,
        },
        Monkey {
            items: vec![72, 51, 93, 63, 80, 86, 81],
            operation: |x| x * x,
            divisible_by: 11,
            true_monkey_idx: 0,
            false_monkey_idx: 4,
            inspected: 0,
        },
    ]
}
