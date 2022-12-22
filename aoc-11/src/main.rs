use num_bigint::{BigUint, ToBigInt, ToBigUint};

type MonkeyItem = u128;

#[derive(Debug, Clone, PartialEq)]
struct Monkey {
    items: Vec<MonkeyItem>,
    operation: fn(MonkeyItem) -> MonkeyItem,
    divisible_by: MonkeyItem,
    true_monkey_idx: usize,
    false_monkey_idx: usize,
    inspected: usize,
}

fn bi(uint: u128) -> MonkeyItem { 
    uint //uint.to_biguint().unwrap() 
}

fn main() {
    println!("Hello, world!");
    let monkeys = get_puzzle_input();
    let (_, inspected) = do_the_thing(&monkeys, 10000);
    println!("inspected: {:?}", inspected);
}

// returns inspected array
fn do_the_thing(monkey_states: &Vec<Monkey>, num_times: u64) -> (Vec<Monkey>, Vec<usize>) {
    let mut monkeys = monkey_states.clone();
    for i in 0..num_times {
        if i%100==0 {
            println!("did the thing {i}");
        }
        monkeys = do_all_the_monkeys(&monkeys);
    }

    let inspected: Vec<usize> = monkeys.iter().map(|monkey| monkey.inspected).collect();
    (monkeys, inspected)
}

fn do_all_the_monkeys(monkey_states: &Vec<Monkey>) -> Vec<Monkey> {
    let mut new_monkey_states = monkey_states.clone();
    for i in 0..monkey_states.len() {
        new_monkey_states = do_monkey(&new_monkey_states, i);
    }

    new_monkey_states
}

//#[test]
// fn test_do_all_the_monkeys() {
//     let new_states = do_all_the_monkeys(&get_sample_input());
//     assert_eq!(vec![20, 23, 27, 26], new_states[0].items);
//     assert_eq!(vec![2080, 25, 167, 207, 401, 1046], new_states[1].items);
//     assert_eq!(Vec::<MonkeyItem>::new(), new_states[2].items);
//     assert_eq!(Vec::<MonkeyItem>::new(), new_states[3].items);

//     let new_states = do_all_the_monkeys(&new_states);
//     assert_eq!(vec![695, 10, 71, 135, 350], new_states[0].items);
//     assert_eq!(vec![43, 49, 58, 55, 362], new_states[1].items);
//     assert_eq!(Vec::<MonkeyItem>::new(), new_states[2].items);
//     assert_eq!(Vec::<MonkeyItem>::new(), new_states[3].items);

//     let new_states = do_all_the_monkeys(&new_states); // 3
//     let new_states = do_all_the_monkeys(&new_states);
//     let new_states = do_all_the_monkeys(&new_states); // 5
//     let new_states = do_all_the_monkeys(&new_states);
//     let new_states = do_all_the_monkeys(&new_states); // 7
//     let new_states = do_all_the_monkeys(&new_states);
//     let new_states = do_all_the_monkeys(&new_states); // 9
//     let new_states = do_all_the_monkeys(&new_states); // 10
//     assert_eq!(vec![91, 16, 20, 98], new_states[0].items);
//     assert_eq!(vec![481, 245, 22, 26, 1092, 30], new_states[1].items);
//     assert_eq!(Vec::<MonkeyItem>::new(), new_states[2].items);
//     assert_eq!(Vec::<MonkeyItem>::new(), new_states[3].items);

//     let new_states = do_all_the_monkeys(&new_states);
//     let new_states = do_all_the_monkeys(&new_states); // 12
//     let new_states = do_all_the_monkeys(&new_states);
//     let new_states = do_all_the_monkeys(&new_states); // 14
//     let new_states = do_all_the_monkeys(&new_states);
//     let new_states = do_all_the_monkeys(&new_states); // 16
//     let new_states = do_all_the_monkeys(&new_states);
//     let new_states = do_all_the_monkeys(&new_states); // 18
//     let new_states = do_all_the_monkeys(&new_states);
//     let new_states = do_all_the_monkeys(&new_states);
//     assert_eq!(vec![10, 12, 14, 26, 34], new_states[0].items);
//     assert_eq!(vec![245, 93, 53, 199, 115], new_states[1].items);
//     assert_eq!(Vec::<MonkeyItem>::new(), new_states[2].items);
//     assert_eq!(Vec::<MonkeyItem>::new(), new_states[3].items);

//     assert_eq!(101, new_states[0].inspected);
//     assert_eq!(95, new_states[1].inspected);
//     assert_eq!(7, new_states[2].inspected);
//     assert_eq!(105, new_states[3].inspected);
// }

fn prime_base(monkeys: &Vec<Monkey>) -> MonkeyItem {
    monkeys.iter().map(|monkey| monkey.divisible_by ).reduce(|x,y| x*y).unwrap()
}

#[test]
fn test_do_the_thing_2() {
    let new_states = get_sample_input();
    let (new_states, inspected) = do_the_thing(&new_states, 1);
    assert_eq!(2, new_states[0].inspected);
    assert_eq!(4, new_states[1].inspected);
    assert_eq!(3, new_states[2].inspected);
    assert_eq!(6, new_states[3].inspected);
    
    let (new_states, inspected) = do_the_thing(&new_states, 9999);
    
    assert_eq!(52166, new_states[0].inspected);
    assert_eq!(47830, new_states[1].inspected);
    assert_eq!(1938, new_states[2].inspected);
    assert_eq!(52013, new_states[3].inspected);
}

// we can copy vecs, it'll be fast enough for this
fn do_monkey(monkey_states: &Vec<Monkey>, monkey_idx: usize) -> Vec<Monkey> {
    let mut mut_monkey_states: Vec<Monkey> = monkey_states.clone();
    let my_monkey = &monkey_states[monkey_idx];
    for item in my_monkey.items.iter() {
        let mut worry_level = (my_monkey.operation)(item.clone());
        worry_level %= prime_base(&monkey_states);
        //let worry_level = worry_level / 3;
        if &worry_level % &my_monkey.divisible_by == bi(0) {
            mut_monkey_states[my_monkey.true_monkey_idx]
                .items
                .push(worry_level.clone());
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
    assert_eq!(vec![bi(74), bi(500), bi(620)], new_monkey_states[3].items);
    assert_eq!(Vec::<MonkeyItem>::new(), new_monkey_states[0].items);
    assert_eq!(2, new_monkey_states[0].inspected);
}

fn get_sample_input() -> Vec<Monkey> {
    vec![
        Monkey {
            // 0:
            items: vec![bi(79), bi(98)],
            operation: |x| x * bi(19),
            divisible_by: bi(23),
            true_monkey_idx: 2,
            false_monkey_idx: 3,
            inspected: 0,
        },
        Monkey {
            items: vec![bi(54), bi(65), bi(75), bi(74)],
            operation: |x| x + bi(6),
            divisible_by: bi(19),
            true_monkey_idx: 2,
            false_monkey_idx: 0,
            inspected: 0,
        },
        Monkey {
            items: vec![bi(79), bi(60), bi(97)],
            operation: |x| &x * &x,
            divisible_by: bi(13),
            true_monkey_idx: 1,
            false_monkey_idx: 3,
            inspected: 0,
        },
        Monkey {
            items: vec![bi(74)],
            operation: |x| x + bi(3),
            divisible_by: bi(17),
            true_monkey_idx: 0,
            false_monkey_idx: 1,
            inspected: 0,
        },
    ]
}

fn get_puzzle_input() -> Vec<Monkey> {
    vec![
        Monkey {
            items: vec![bi(54), bi(61), bi(97), bi(63), bi(74)],
            operation: |x| x * bi(7),
            divisible_by: bi(17),
            true_monkey_idx: 5,
            false_monkey_idx: 3,
            inspected: 0,
        },
        Monkey {
            items: vec![bi(61), bi(70), bi(97), bi(64), bi(99), bi(83), bi(52), bi(87)],
            operation: |x| x + bi(8),
            divisible_by: bi(2),
            true_monkey_idx: 7,
            false_monkey_idx: 6,
            inspected: 0,
        },
        Monkey {
            items: vec![bi(60), bi(67), bi(80), bi(65)],
            operation: |x| x * bi(13),
            divisible_by: bi(5),
            true_monkey_idx: 1,
            false_monkey_idx: 6,
            inspected: 0,
        },
        Monkey {
            items: vec![bi(61), bi(70), bi(76), bi(69), bi(82), bi(56)],
            operation: |x| x + bi(7),
            divisible_by: bi(3),
            true_monkey_idx: 5,
            false_monkey_idx: 2,
            inspected: 0,
        },
        Monkey {
            items: vec![bi(79), bi(98)],
            operation: |x| x + bi(2),
            divisible_by: bi(7),
            true_monkey_idx: 0,
            false_monkey_idx: 3,
            inspected: 0,
        },
        Monkey {
            items: vec![bi(72), bi(79), bi(55)],
            operation: |x| x + bi(1),
            divisible_by: bi(13),
            true_monkey_idx: 2,
            false_monkey_idx: 1,
            inspected: 0,
        },
        Monkey {
            items: vec![bi(63)],
            operation: |x| x + bi(4),
            divisible_by: bi(19),
            true_monkey_idx: 7,
            false_monkey_idx: 4,
            inspected: 0,
        },
        Monkey {
            items: vec![bi(72), bi(51), bi(93), bi(63), bi(80), bi(86), bi(81)],
            operation: |x| &x * &x,
            divisible_by: bi(11),
            true_monkey_idx: 0,
            false_monkey_idx: 4,
            inspected: 0,
        },
    ]
}
