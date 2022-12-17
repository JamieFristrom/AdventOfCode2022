use scanf::sscanf;

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

fn switch_enabled(bitset: usize, switch: usize) -> bool {
    (bitset & (1<<switch)) != 0
}

fn do_the_thing(input: &str) -> u16{
    let (mut distances, mut flows) = parse_input(input);
    trim_distances_and_flows(&mut distances, &mut flows);
    let answer_space = evaluate(&distances, &flows, 30);

    answer_space[30][0][0]
}

fn fill_out_time_n(distances: &Vec<Vec<usize>>, flows: &Vec<u16>, answer_space: &mut AnswerSpace, time_left: u16) {
    
    // at time n, do we travel to another node and turn it on (getting the value for its time-distance entry
    // in the table) or do we turn on our current switch?
    let toggles_size = 1 << flows.len();
    for bitset in 0..toggles_size {
        for (start_node_idx, start_node_flow) in flows.iter().enumerate() {
            for (dest_node_idx, dest_node_flow) in flows.iter().enumerate() {
                if !(switch_enabled(bitset, dest_node_idx)) {
                    // we get the value of traveling there + turning on, plus the value for starting there with it off
                    let distance = distances[start_node_idx][dest_node_idx] as u16; 
                    if distance<time_left-1 {  // example, distance 1, time_left 2: enough time to travel there, turn it on, but it doesn't fuel until next tick, and get no further value
                        let turn_on_value = dest_node_flow * (time_left-1 as u16-distance);  // distance 1, time_left 2, 1 unit of flow
                        let changed_bitset = bitset | (1<<dest_node_idx);
                        // -2 below: -1 for starting array indexing at 0, -1 for the time it took to throw the switch
                        let node_value = answer_space[(time_left-distance-1) as usize][changed_bitset][dest_node_idx]; // time_left 2, distance 1, 0th index (which is all 0s)
                        let our_value = turn_on_value + node_value;
                        if our_value > answer_space[time_left as usize][bitset][start_node_idx] {  
                            // hard to wrap my head around which bitset to use. this is the one where we haven't flipped it yet, because flipping it is one of the options we could take at this location at this time
                            answer_space[time_left as usize][bitset][start_node_idx] = our_value;
                        }
                    }
                }
            }
        }
    }
}

fn evaluate(distances: &Vec<Vec<usize>>, flows: &Vec<u16>, time: usize)->AnswerSpace {
    // allocate our mondo array
    let toggles_size = 1 << flows.len();
    // array of time * starting-point * time 
    let mut answer_space = vec![vec![vec![0u16;flows.len()];toggles_size];time+1];
    // time left 0 is already filled with 0's - it is too late to accomplish anything, but I want a column in the array
    // to skip an if check
    for i in 1..31 {
        println!("What to do when {i} time left");
        fill_out_time_n(distances, flows, &mut answer_space, i);
    }
    // for tick in 1..time {
    //     fill_out_time_n(distances, flows, &mut answer_space, tick);
    // }
    answer_space
}

#[test]
fn test_simple_sitch() {
    let distances = vec![vec![0,1,2],vec![1,0,1],vec![2,1,0]];
    let flows = vec![1u16,3u16,7u16];
    let toggles_size = 1 << flows.len();
    // array of time * starting-point * time 
    let mut answer_space = vec![vec![vec![0u16;flows.len()];toggles_size];10];
    fill_out_time_n(&distances, &flows, &mut answer_space, 1);
    assert_eq!(0, answer_space[0][0][0]);  // time, toggles, nodes
    assert_eq!(0, answer_space[0][7][0]);
    assert_eq!(0, answer_space[0][0][1]);
    assert_eq!(0, answer_space[0][7][1]);
    assert_eq!(0, answer_space[0][0][2]);
    assert_eq!(0, answer_space[0][7][2]);
    fill_out_time_n(&distances, &flows, &mut answer_space, 2);
    assert_eq!(0, answer_space[1][0][0]);  // time, toggles, nodes
    assert_eq!(0, answer_space[1][7][0]);
    assert_eq!(0, answer_space[1][0][1]);
    assert_eq!(0, answer_space[1][7][1]);
    assert_eq!(0, answer_space[1][0][2]);
    assert_eq!(0, answer_space[1][7][2]);
    fill_out_time_n(&distances, &flows, &mut answer_space, 3);
    assert_eq!(1, answer_space[2][0][0]);  // time, toggles, nodes
    assert_eq!(0, answer_space[2][7][0]);
    assert_eq!(3, answer_space[2][0][1]);
    assert_eq!(0, answer_space[2][7][1]);
    assert_eq!(7, answer_space[2][0][2]);
    assert_eq!(0, answer_space[2][7][2]);
    fill_out_time_n(&distances, &flows, &mut answer_space, 4);
    assert_eq!(3, answer_space[3][0][0]);  // move to 1 and turn on
    assert_eq!(0, answer_space[3][7][0]);
    assert_eq!(7, answer_space[3][0][1]);  // you have time to move to #2 and turn it on for 7 instead of 2x3
    assert_eq!(0, answer_space[3][7][1]);
    assert_eq!(14, answer_space[3][0][2]);  // stay where you are and turn on
    assert_eq!(0, answer_space[3][7][2]);
    fill_out_time_n(&distances, &flows, &mut answer_space, 5);
    assert_eq!(7, answer_space[4][0][0]);  // move to 2 and turn on
    assert_eq!(0, answer_space[4][7][0]);
    assert_eq!(16, answer_space[4][0][1]);  // stay here, turn it on (for 9) move to 3 for 7. Better than move to 3 and turn on for 2*7. The code figured it out!
    assert_eq!(0, answer_space[4][7][1]);
    assert_eq!(24, answer_space[4][0][2]);  // stay where you are and turn on (21), then move one and turn on 2 for 3
    assert_eq!(0, answer_space[4][7][2]);    
    assert_eq!(6, answer_space[4][4][2]); // if you start on 2 when it's already toggled, what can you do? MOve left and turn on for 2 units
    fill_out_time_n(&distances, &flows, &mut answer_space, 6);
    assert_eq!(16, answer_space[5][0][0]);  // move to 1, turn on for 9, move to 2 and turn on for 7
    assert_eq!(0, answer_space[5][7][0]);
    assert_eq!(26, answer_space[5][0][1]);  // move to 2 and turn on for 3*7, then back for 3 for 21...or turn on for 3*4, move to 2 for 2*7 = 14+12 = 26
    assert_eq!(0, answer_space[5][7][1]);
    assert_eq!(34, answer_space[5][0][2]);  // stay where you are and turn on for 28, then to #2 for 6
    assert_eq!(0, answer_space[5][7][2]);    
    //assert_eq!()
    
}

#[test]
fn test_sample_input() {
    let answer = do_the_thing(get_sample_input());
    assert_eq!(1651, answer);
}

// returns distance matrix and flow matrix
fn parse_input(input: &str) -> (Vec<Vec<usize>>, Vec<u16>) {
    let mut flows: Vec<u16> = vec![]; 
    let mut valve_ids: Vec<String> = vec![];
    let mut tunnels_for_valves: Vec<String> = vec![];
    for line in input.lines() {
        let mut valve_id = String::new();
        let mut flow: u16 = 0;
        let mut tunnels = String::new();
        println!("{line}");
        match sscanf!(line, "Valve {} has flow rate={}; tunnels lead to valves {}", valve_id, flow, tunnels) {
            Ok(_)=>{
                valve_ids.push(valve_id);
                flows.push(flow);
                tunnels_for_valves.push(tunnels);
            },
            Err(err) => {
                println!("{err}");
                panic!(); 
            }
        }
    }

    // create adjacency
    let mut distances: Vec<Vec<usize>> = vec![vec![1000;valve_ids.len()];valve_ids.len()];
    for (valve_idx,tunnels) in tunnels_for_valves.iter().enumerate() {
        let tunnel_ids = tunnels.split(", ");
        for tunnel_id in tunnel_ids {
            let target_valve_idx = valve_ids.iter().position(|x| x==tunnel_id).unwrap();
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

    (distances, flows)
}

#[test]
fn test_parse_input() {
    let (distances, flows) = parse_input(get_sample_input());
    assert_eq!(0,flows[0]);
    assert_eq!(13,flows[1]);
    assert_eq!(21,flows[9]);
    assert_eq!(1,distances[0][1]);
    assert_eq!(1,distances[1][0]);
    assert_eq!(3,distances[0][5]);
    assert_eq!(3,distances[5][0]);
    assert_eq!(2,distances[0][9]);
    assert_eq!(2,distances[9][0]);
}

fn trim_distances_and_flows(distances: &mut Vec<Vec<usize>>, flows: &mut Vec<u16>) {
    // we only care about the ones that flow
    // using the example from the retain() docs
    let mut flows_iter = flows.iter();
    distances.retain(|_| *flows_iter.next().unwrap()>0);

    for i in 0..distances.len() {
        let mut flows_iter = flows.iter();
        distances[i].retain(|_| *flows_iter.next().unwrap()>0);
    }

    flows.retain(|flow| flow>&0 );
}

#[test]
fn test_trim_distances_and_flows() {
    let (mut distances,mut flows) = parse_input(get_sample_input());
    trim_distances_and_flows(&mut distances, &mut flows);
    assert_eq!(6,distances.len());
    assert_eq!(6,flows.len());
    assert_eq!(6,distances[0].len());
    assert_eq!(13,flows[0]);
    assert_eq!(21,flows[5]);
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