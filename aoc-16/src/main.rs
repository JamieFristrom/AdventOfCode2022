use scanf::sscanf;
use std::{collections::HashMap, hash::Hash};
use core::time;

fn main() {
    println!("Hello, world!");
    let answer = do_the_thing(get_puzzle_input());
    println!("Boom : {answer}");
}

// my insight here is that given a known set of already-toggled-switches (2^n), a starting point (n), and time,
// there is a right answer, and that's a (barely) tractable number of things (30m for 15 toggles)
// arbitrarily - time, toggles, nodes
type AnswerSpace = Vec<Vec<Vec<u16>>>;

struct Valve {
    id: String,
    flow: u16,
    distances: HashMap<String,u16>,
    bit: Option<usize>
}

fn switch_enabled(bitset: usize, important_valves: &Vec<&Valve>, valve: &Valve) -> bool {
    match get_bit_for_valve(important_valves, valve) {
        Some(bit) => { (bitset & (1<<bit)) != 0 }
        None => { false } // we could probably skip some math if we acted as if 0 flow switches were aleady toggled but less readable?
    }
}

fn do_the_thing(input: &str) -> u16{
    let (mut distances, mut valves) = parse_input(input);
    //trim_distances_and_valves(&mut distances, &mut flows);
    let answer_space = evaluate(&distances, &mut valves, 30);

    let starting_valve_idx = valves.iter().position(|v| v.id=="AA").unwrap();
    answer_space[30][0][starting_valve_idx]
}

// can turn into a hashmap later if slow
// fn get_valve_for_toggle(valves: &Vec<Valve>, toggle_idx:usize) -> &Valve {
//     valves.iter().filter(|valve| valve.flow>0).nth(toggle_idx).unwrap()
// }

// if things are slow this is an obvious optimization target
fn get_bit_for_valve(important_valves: &Vec<&Valve>, valve: &Valve) -> Option<usize> {
    valve.bit
    //important_valves.iter().position(|important_valve| important_valve.id == valve.id) // in theory could just compare pointers, not sure how to do that
}

fn fill_out_time_n(valves: &Vec<Valve>, answer_space: &mut AnswerSpace, time_left: usize) {
    // at time n, do we travel to another node and turn it on (getting the value for its time-distance entry
    // in the table) or do we turn on our current switch?
    if time_left==14 {
        println!("stop");
    }
    let important_valves = get_important_valves(valves);
    let toggle_flags_range = 1 << important_valves.len();
    for bitset in 0..toggle_flags_range {
        for (start_toggle_idx, start_valve) in valves.iter().enumerate() {
            for (dest_toggle_idx, dest_valve) in valves.iter().enumerate() {
                let mut turn_on_value = 0;
                let mut changed_bitset = bitset;
                let distance = start_valve.distances[&dest_valve.id];
                if !(switch_enabled(bitset, &important_valves, &dest_valve)) {
                    // we get the value of traveling there + turning on, plus the value for starting there with it off
                    if distance==0 {
                        turn_on_value = dest_valve.flow * (time_left as u16-1);
                        match get_bit_for_valve(&important_valves, &dest_valve) {
                            Some(bit) => { changed_bitset = changed_bitset | 1<<bit; }
                            None => {}
                        }
                    }
                }
                if distance<=1 {
                    let node_value = answer_space[(time_left-1) as usize][changed_bitset][dest_toggle_idx]; // time_left 2, distance 1, 0th index (which is all 0s)
                    let our_value = turn_on_value + node_value;
                    if our_value > answer_space[time_left as usize][bitset][start_toggle_idx] {  
                        // hard to wrap my head around which bitset to use. this is the one where we haven't flipped it yet, because flipping it is one of the options we could take at this location at this time
                        answer_space[time_left as usize][bitset][start_toggle_idx] = our_value;
                    }
                }
            }
        }
    }
}

fn get_important_valves(valves: &Vec<Valve>) -> Vec<&Valve> {
    // optimized:
    let important_valves: Vec<&Valve> = valves.iter().filter(|valve| valve.flow>0 || valve.id=="AA").collect();
    // because you start here it's on the important list
    // unoptimized:
    //let important_valves = valves;
    important_valves
}

fn map_bits_to_valves(valves: &mut Vec<Valve>) {
    let mut bit_counter = 0;
    for valve in valves {
        if valve.flow > 0 || valve.id == "AA" {
            valve.bit = Some(bit_counter);
            bit_counter = bit_counter+1;
        }
    }
}

fn evaluate(distances: &Vec<Vec<usize>>, valves: &mut Vec<Valve>, time: usize)->AnswerSpace {
    // allocate our mondo array
    // array of time * starting-point * time 
    let num_valves = valves.len();
    map_bits_to_valves(valves);
    let important_valves = get_important_valves(&valves);
    
    let toggles_size = 1 << important_valves.len();
    
    let mut answer_space = vec![vec![vec![0u16;num_valves];toggles_size];time+1];
    // time left 0 is already filled with 0's - it is too late to accomplish anything, but I want a column in the array
    // to skip an if check
    
    for i in 1..time+1 {
        fill_out_time_n(valves, &mut answer_space, i);
        
        println!("What to do when {i} time left");
        for (j, valve) in valves.iter().enumerate() {
            println!("{j} Valve {}/{}: best_flow {}", valve.id, valve.flow, answer_space[i as usize][0][j]);
        }
    }
    // for tick in 1..time {
    //     fill_out_time_n(distances, valves, &mut answer_space, tick);
    // }
    answer_space
}

#[test]
fn test_simple_sitch() {
    let distances: Vec<Vec<usize>> = vec![vec![0,1,2,3],vec![1,0,1,2],vec![2,1,0,1],vec![3,2,1,0]];
    let mut valves = vec![
        Valve{flow:1u16,id:"AA".to_string(), distances: HashMap::new(), bit: None},
        Valve{flow:3u16,id:"B".to_string(), distances: HashMap::new(), bit: None},
        Valve{flow:0u16,id:"C".to_string(), distances: HashMap::new(), bit: None},
        Valve{flow:7u16,id:"D".to_string(), distances: HashMap::new(), bit: None}];
    transcribe_distances_into_valves(&mut valves, &distances);
    let toggles_size = 1 << valves.len();
    map_bits_to_valves(&mut valves);
    // array of time * starting-point * time 
    let mut answer_space = vec![vec![vec![0u16;valves.len()];toggles_size];10];
    fill_out_time_n(&valves, &mut answer_space, 1);
    assert_eq!(0, answer_space[0][0][0]);  // time, toggles, nodes
    assert_eq!(0, answer_space[0][7][0]);
    assert_eq!(0, answer_space[0][0][1]);
    assert_eq!(0, answer_space[0][7][1]);
    assert_eq!(0, answer_space[0][0][2]);
    assert_eq!(0, answer_space[0][7][2]);
    assert_eq!(0, answer_space[0][0][3]);
    assert_eq!(0, answer_space[0][7][3]);
    fill_out_time_n( &valves, &mut answer_space, 2);
    assert_eq!(0, answer_space[1][0][0]);  // time, toggles, nodes
    assert_eq!(0, answer_space[1][7][0]);
    assert_eq!(0, answer_space[1][0][1]);
    assert_eq!(0, answer_space[1][7][1]);
    assert_eq!(0, answer_space[1][0][2]);
    assert_eq!(0, answer_space[1][7][2]);
    assert_eq!(0, answer_space[1][0][3]);
    assert_eq!(0, answer_space[1][7][3]);
    assert_eq!(0, answer_space[1][1][0]);
    fill_out_time_n( &valves, &mut answer_space, 3);
    assert_eq!(1, answer_space[2][0][0]);  // time, toggles, nodes
    assert_eq!(0, answer_space[2][7][0]);
    assert_eq!(3, answer_space[2][0][1]);
    assert_eq!(0, answer_space[2][7][1]);
    assert_eq!(0, answer_space[2][0][2]);
    assert_eq!(0, answer_space[2][7][2]);
    assert_eq!(7, answer_space[2][0][3]);
    //assert_eq!(7, answer_space[2][7][3]);   // because last bit clear
    //assert_eq!(0, answer_space[2][15][3]);
    fill_out_time_n( &valves, &mut answer_space, 4);
    assert_eq!(3, answer_space[3][0][0]);  // move to 1 and turn on
    //assert_eq!(0, answer_space[3][15][0]);
    assert_eq!(6, answer_space[3][0][1]);  // you don't have time to move to #3 and turn it on for 7 instead of 2x3
    //assert_eq!(0, answer_space[3][15][1]);
    assert_eq!(7, answer_space[3][0][2]);  
    //assert_eq!(0, answer_space[3][15][2]);
    assert_eq!(14, answer_space[3][0][3]);
    //assert_eq!(0, answer_space[3][15][3]); 
    fill_out_time_n( &valves, &mut answer_space, 5);
    assert_eq!(6, answer_space[4][0][0]);  // move to 1 and turn on
    //assert_eq!(0, answer_space[4][15][0]);
    assert_eq!(10, answer_space[4][0][1]);  
    //assert_eq!(0, answer_space[4][15][1]);
    assert_eq!(14, answer_space[4][0][2]);  
    //assert_eq!(0, answer_space[4][15][2]);    
    assert_eq!(3, answer_space[4][4][3]); // bit 2 here goes with value 3 in optimized sitch; you have to move. step, step, turn on
    assert_eq!(21, answer_space[4][0][3]);
    //assert_eq!(0, answer_space[4][15][3]); 
    fill_out_time_n( &valves, &mut answer_space, 6);
    assert_eq!(10, answer_space[5][0][0]);  // I think turn on where you're at (4), step (6) is better than step (9) and step back (1)move to 1, turn on for 9, move to 2 and turn on for 7
    //assert_eq!(0, answer_space[5][15][0]);
    assert_eq!(19, answer_space[5][0][1]);  // turn on for 12, move to 3 and turn on for 7
    //assert_eq!(0, answer_space[5][15][1]);
    assert_eq!(21, answer_space[5][0][2]);  // move right, 3*7
    //assert_eq!(0, answer_space[5][15][2]);    
    assert_eq!(31, answer_space[5][0][3]);  // 4*7+3
    //assert_eq!(0, answer_space[5][15][3]); 
    //assert_eq!()
    
}

#[test]
fn test_sample_input() {
    let answer = do_the_thing(get_sample_input());
    assert_eq!(1651, answer);
}

// returns distance matrix and flow matrix
fn parse_input(input: &str) -> (Vec<Vec<usize>>, Vec<Valve>) {
    let mut valves: Vec<Valve> = vec![];
    let mut tunnels_for_valves: Vec<String> = vec![];
    for line in input.lines() {
        let mut valve_id = String::new();
        let mut flow: u16 = 0;
        let mut tunnels = String::new();
        println!("{line}");
        match sscanf!(line, "Valve {} has flow rate={}; tunnels lead to valves {}", valve_id, flow, tunnels) {
            Ok(_)=>{
                valves.push(Valve{id:valve_id, flow:flow, distances:HashMap::new(), bit: None});
                tunnels_for_valves.push(tunnels);
            },
            Err(err) => {
                println!("{err}");
                panic!(); 
            }
        }
    }

    // create adjacency
    let mut distances: Vec<Vec<usize>> = vec![vec![1000;valves.len()];valves.len()];
    for (valve_idx,tunnels) in tunnels_for_valves.iter().enumerate() {
        let tunnel_ids = tunnels.split(", ");
        for tunnel_id in tunnel_ids {
            let target_valve_idx = valves.iter().position(|x| x.id==tunnel_id).unwrap();
            distances[valve_idx][target_valve_idx] = 1;
            distances[valve_idx][valve_idx] = 0;
        }
    }

    // memoize, or whatever its called. I forget how to do this in one pass
    loop {
        let mut made_change = false;
        for i in 0..distances.len() {
            for j in 0..distances.len() {
                for k in 0..distances.len() {
                    let new_dist = distances[k][j] + distances[j][i];
                    if new_dist < distances[k][i] {
                        distances[k][i] = new_dist;
                        made_change = true;
                    }
                }
            }
        }
        if !made_change {break;}
    }

    // validate symmetry
    for (i, distance_row) in distances.iter().enumerate() {
        for (j, distance) in distance_row.iter().enumerate() {
            assert_eq!( distance, &distances[j][i] );
        }
    }

    transcribe_distances_into_valves(&mut valves, &distances);

    (distances, valves)
}

fn transcribe_distances_into_valves(valves: &mut Vec<Valve>, distances: &Vec<Vec<usize>>) {
    // add back to valves
    // there's probably a better way to do this to deal with the borrower
    let ids: Vec<String> = valves.iter().map(|v| v.id.clone()).collect();
    for i in 0..valves.len() {
        for j in 0..valves.len() {
            valves[i].distances.insert(ids[j].clone(), distances[i][j] as u16);
        }
    }
}

// too slow to leave in test suite:
// #[test]
// fn test_part_1() {
//     let answer = do_the_thing(get_puzzle_input());
//     assert_eq!(1638, answer);
// }

#[test]
fn test_parse_input() {
    let (distances, valves) = parse_input(get_sample_input());
    assert_eq!(0,valves[0].flow);
    assert_eq!(13,valves[1].flow);
    assert_eq!(21,valves[9].flow);
    assert_eq!(1,distances[0][1]);
    assert_eq!(1,distances[1][0]);
    assert_eq!(3,distances[0][5]);
    assert_eq!(3,distances[5][0]);
    assert_eq!(2,distances[0][9]);
    assert_eq!(2,distances[9][0]);
}

fn get_sample_input() -> &'static str {
"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnels lead to valves GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnels lead to valves II"
}

fn get_puzzle_input() -> &'static str {
"Valve TN has flow rate=0; tunnels lead to valves IX, ZZ
Valve DS has flow rate=0; tunnels lead to valves IF, OU
Valve OP has flow rate=0; tunnels lead to valves UH, ZQ
Valve FS has flow rate=0; tunnels lead to valves IF, UH
Valve WO has flow rate=0; tunnels lead to valves IS, RW
Valve KQ has flow rate=0; tunnels lead to valves SI, WZ
Valve IX has flow rate=0; tunnels lead to valves IF, TN
Valve OU has flow rate=0; tunnels lead to valves EB, DS
Valve ZZ has flow rate=10; tunnels lead to valves II, GR, HA, BO, TN
Valve OW has flow rate=0; tunnels lead to valves RI, IS
Valve DV has flow rate=0; tunnels lead to valves FR, MT
Valve ZK has flow rate=0; tunnels lead to valves WG, VE
Valve XB has flow rate=0; tunnels lead to valves WG, HM
Valve XC has flow rate=0; tunnels lead to valves IS, MT
Valve KO has flow rate=0; tunnels lead to valves NH, AA
Valve RN has flow rate=0; tunnels lead to valves AA, MT
Valve ZQ has flow rate=5; tunnels lead to valves MF, LK, OP
Valve MF has flow rate=0; tunnels lead to valves ZQ, BH
Valve HA has flow rate=0; tunnels lead to valves LK, ZZ
Valve GB has flow rate=0; tunnels lead to valves KZ, RW
Valve KZ has flow rate=24; tunnels lead to valves GB, RI
Valve TC has flow rate=0; tunnels lead to valves SI, AA
Valve II has flow rate=0; tunnels lead to valves SI, ZZ
Valve EZ has flow rate=0; tunnels lead to valves DF, MT
Valve LK has flow rate=0; tunnels lead to valves HA, ZQ
Valve DU has flow rate=0; tunnels lead to valves NH, IU
Valve MT has flow rate=3; tunnels lead to valves EZ, XC, RN, DV, RU
Valve GR has flow rate=0; tunnels lead to valves SX, ZZ
Valve SX has flow rate=0; tunnels lead to valves UH, GR
Valve BO has flow rate=0; tunnels lead to valves ZZ, AO
Valve WG has flow rate=16; tunnels lead to valves FR, MX, XB, ZK
Valve IP has flow rate=8; tunnels lead to valves HM, RU, WZ, IU
Valve RI has flow rate=0; tunnels lead to valves OW, KZ
Valve NP has flow rate=0; tunnels lead to valves WN, EB
Valve IF has flow rate=19; tunnels lead to valves IX, DS, VX, FS
Valve AA has flow rate=0; tunnels lead to valves RN, KO, TC, MX
Valve IS has flow rate=15; tunnels lead to valves OW, WO, XC
Valve BH has flow rate=11; tunnels lead to valves MF
Valve SI has flow rate=4; tunnels lead to valves KQ, II, TC
Valve WN has flow rate=0; tunnels lead to valves UH, NP
Valve RW has flow rate=18; tunnels lead to valves WO, GB
Valve DF has flow rate=0; tunnels lead to valves NH, EZ
Valve WZ has flow rate=0; tunnels lead to valves KQ, IP
Valve HM has flow rate=0; tunnels lead to valves XB, IP
Valve VX has flow rate=0; tunnels lead to valves AO, IF
Valve MX has flow rate=0; tunnels lead to valves AA, WG
Valve NH has flow rate=13; tunnels lead to valves VE, KO, DU, DF
Valve RU has flow rate=0; tunnels lead to valves MT, IP
Valve IU has flow rate=0; tunnels lead to valves IP, DU
Valve VE has flow rate=0; tunnels lead to valves ZK, NH
Valve FR has flow rate=0; tunnels lead to valves WG, DV
Valve AO has flow rate=21; tunnels lead to valves BO, VX
Valve EB has flow rate=22; tunnels lead to valves OU, NP
Valve UH has flow rate=12; tunnels lead to valves WN, OP, SX, FS"
}