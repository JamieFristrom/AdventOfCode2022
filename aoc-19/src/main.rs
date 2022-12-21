use std::collections::{HashMap,HashSet,BinaryHeap, VecDeque};
use std::cmp::{Ordering, max};
use scanf::sscanf;
use std::thread;

static RESOURCE_NAMES: [&str;4] = ["ore","clay","obsidian","geode"];

type Datum = u8;
type Vec4d = [Datum;4];

fn main() {
    println!("Hello, world!");
    // part 1
    //let answer = do_the_thing(get_puzzle_input(), 300, 24);
    // part 2
    let answer = do_the_thing(get_puzzle_input(), 3, 32);
    println!("Boom? {answer}")
}

fn do_the_thing(input: &str, max_blueprints: usize, turns_left: i32) -> usize {
    let blueprints = parse_input(&input);
    let mut total_quality: usize = 0;
    //let mut threads = vec![];
    for (i, blueprint) in blueprints.iter().enumerate() {
        if i>=max_blueprints {break;}
        println!("blueprint {i}/{}", max_blueprints);
        //let mut turn_tracker = 0i32;
        //let mut answer_space = HashMap::<(Vec4d,Vec4d,i32),i32>::new();
        let geodes = bfs(blueprint, turns_left);
        println!("blueprint {i} geodes {geodes}");
        let quality = geodes * (i+1);
        total_quality += quality;
    }   

    total_quality
}

// fn do_the_thing_in_parallel(input: &str, max_blueprints: usize, turns_left: i32) -> usize {
//     let blueprints = parse_input(&input);
//     let mut total_quality: usize = 0;
//     let mut threads = vec![];
//     let blueprints_len = blueprints.len();
//     for (i, blueprint) in blueprints.iter().enumerate() {
//         if i>=max_blueprints {break;}
//         let cloned_blueprint = blueprint.clone();
//         threads.push( thread::spawn(|| {
//             println!("blueprint {i}/{}", blueprints_len);
//             let mut turn_tracker = 0i32;
//             let mut answer_space = HashMap::<(Vec4d,Vec4d,i32),i32>::new();
//             let geodes = bfs(&cloned_blueprint, turns_left);
//             println!("blueprint {i} geodes {geodes}");
//             geodes
//         }));
//     }
//     for (i, thread) in threads.into_iter().enumerate() {
//         let quality = thread.join().unwrap() * (i+1);
//         total_quality += quality;
//     }

//     total_quality
// }

// #[test]
// fn test_do_the_thing() {
//     let answer = do_the_thing(get_sample_input(),1000,24);
//     assert_eq!(33, answer);
// }

fn affordable(cost: &Vec4d, resources: &Vec4d) -> bool {
    for i in 0..cost.len() {
        if cost[i] > resources[i] { return false;}
    }

    true
}

fn v4less_or_equal(v1: &Vec4d, v2: &Vec4d) -> bool {
    for i in 0..v1.len() {
        if v1[i] > v2[i] { return false;}
    }

    true
}

fn v4equal(v1: &Vec4d, v2: &Vec4d) -> bool {
    for i in 0..v1.len() {
        if v1[i] != v2[i] { return false;}
    }

    true 
}

fn v4add(r1: &Vec4d, r2: &Vec4d) -> Vec4d {
    let mut remainder = [0,0,0,0];
    for i in 0..r1.len() {
        remainder[i] = r1[i]+r2[i];
    }

    remainder
}

fn v4sub(v1: &Vec4d, v2: &Vec4d) -> Vec4d {
    let mut remainder = [0,0,0,0];
    for i in 0..v1.len() {
        remainder[i] = v1[i]-v2[i];
    }

    remainder
}

fn v4mul(v1: &Vec4d, v2: &Vec4d) -> Vec4d {
    [v1[0]*v2[0],v1[1]*v2[1],v1[2]*v2[2],v1[3]*v2[3]]
}

fn v4mul_scalar(v: &Vec4d, scalar: u8) -> Vec4d {
    [v[0]*scalar, v[1]*scalar, v[2]*scalar, v[3]*scalar]
}

fn get_elements_for_robot_idx() -> &'static [Vec4d;4] {
    &[[1,0,0,0],[0,1,0,0],[0,0,1,0],[0,0,0,1]]
}

fn theoretical_resource_production(robots: &Vec4d, turns: u8)->Vec4d {
    let turns = std::cmp::max(turns,0);
    [robots[0]*turns,robots[1]*turns,robots[2]*turns,robots[3]*turns]
}


#[derive(Clone, Eq, PartialEq)]
struct State {
    robots: Vec4d,
    resources: Vec4d,
    turns_left: Datum,
    value: u32
}

fn evaluate_state(robots: &Vec4d, resources: &Vec4d, turns_left: Datum) -> u32 {
    let total_output = v4add( resources, &v4mul_scalar(robots, turns_left));
   
    (total_output[3] as u32) * 16777216 + (total_output[2] as u32) * 65536 + (total_output[1] as u32)*256 + (total_output[0] as u32)
}

fn v4cmp(v1: &Vec4d, v2: &Vec4d) -> Ordering {
    let v1value = (v1[3] as u64) * 16777216 + (v1[2] as u64) * 65536 + (v1[1] as u64)*256 + (v1[0] as u64);    
    let v2value = (v2[3] as u64) * 16777216 + (v2[2] as u64) * 65536 + (v2[1] as u64)*256 + (v2[0] as u64);
    v1value.cmp(&v2value)
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // since we're running out of memory, computing this every time instead of saving it. same complexity...
        let total_resources = v4add(&self.resources, &v4mul_scalar(&self.robots, self.turns_left));
        let other_total_resources = v4add(&other.resources, &v4mul_scalar(&other.robots, other.turns_left));
        v4cmp(&other_total_resources, &total_resources)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn bfs(blueprints: &Vec<Vec4d>, turns_left: i32) -> usize {
    bfs_from_state(blueprints, &[0,0,0,0], &[1,0,0,0], turns_left)
}

type PriorityQueue = BinaryHeap<State>;
//type PriorityQueue = VecDeque<State>;

// memory usage is a problem. don't put things we don't care about on the queue
fn queue_push(blueprints: &Vec<Vec4d>, visited_states: &mut HashMap::<Vec4d,Vec4d>, queue: &mut PriorityQueue, state: State) {
    
    
   queue.push(state);
   //queue.push_back(state);
}

fn bfs_from_state(blueprints: &Vec<Vec4d>, resources: &Vec4d, robots: &Vec4d, turns_left: i32) -> usize {  
    
   let mut priority_queue = BinaryHeap::new();
   priority_queue.push( State { robots: *robots, resources: *resources, turns_left: turns_left as Datum, value: evaluate_state(robots, resources, turns_left as Datum) } );
  //  let mut priority_queue = PriorityQueue::new();//(State { robots: [1,0,0,0], resources: [0,0,0,0], turns_left});
    //priority_queue.push_back( State { robots: *robots, resources: *resources, turns_left: turns_left as Datum});
    let mut visited_states = HashMap::<Vec4d,Vec4d>::new();
    let mut best: usize = 0;
    let mut turns_watermark = i32::MAX;

    while let Some(State { robots, resources, turns_left, value }) = priority_queue.pop() { //{}.pop_front() {
        if (turns_left as i32) < turns_watermark {
            println!("Turns_left {turns_left} hit");
            turns_watermark = turns_left as i32;
        }
        
        let theoretical_output = v4add(&resources, &v4mul_scalar(&robots, turns_left as u8));
        if let Some(visited_theoretical_output) = visited_states.get(&(robots)) {
            // not pruning by total value - there are cases where it's wrong -- for example, you might 
            // have a deck of all geode crackers which looks good on paper, but if that's early in the game
            // you might not be able to build any more geode crackers, so a fleet with no geode crackers but
            // some geode cracker builders would be better
            if v4less_or_equal(&theoretical_output, &visited_theoretical_output) {
                if !v4equal(&theoretical_output, &visited_theoretical_output) {
                    // strictly less
                    //println!("pruning succesful");
                    continue;
                }
            }
        }
        visited_states.insert(robots.clone(),theoretical_output);
        // not quite redudant - currently a state could get pushed on the queue later that's better, but 
        // 
        // let theoretical_output = v4mul_scalar(&resources, turns_left);
        // // the robots state is pretty important, and we can compare resources to decide whether to bail
        // if let Some(visited_theoretical_output) = visited_states.get(&(robots)) {
        //     if affordable(&theoretical_output, visited_theoretical_output) {
        //         // this situation is strictly worse or the same, screw it
        //         continue;
        //     }
        // }
        // visited_states.insert(robots,theoretical_output);

        let new_resources = v4add( &resources, &robots ); 
        
        best = std::cmp::max(best, new_resources[3] as usize);

        // what robot to build now?
        if turns_left > 1 {
                for robot_choice in (0..4).rev() {  // do best robot first
                if affordable(&blueprints[robot_choice], &resources) {
                    let remaining_resources = v4sub(&new_resources, &blueprints[robot_choice]);
                    let mut new_robots = robots.clone();
                    new_robots[robot_choice] += 1;
                    assert!(remaining_resources[0]>=0);
                    assert!(remaining_resources[1]>=0);
                    assert!(remaining_resources[2]>=0);
                    assert!(remaining_resources[3]>=0);
                    //let total_resources = v4add(&remaining_resources, &v4mul_scalar(&new_robots, turns_left-1));
                    queue_push( blueprints, &mut visited_states, &mut priority_queue, State { robots: new_robots, resources: remaining_resources, turns_left: turns_left-1, value: evaluate_state(&new_robots, &remaining_resources, turns_left-1) });
                }
            }
            // or do nothing and save your monies
            let total_resources = v4add(&resources, &v4mul_scalar(&robots, turns_left-1));
            queue_push( blueprints, &mut visited_states, &mut priority_queue, State { robots: robots, resources: new_resources, turns_left: turns_left-1, value: evaluate_state(&robots, &new_resources, turns_left-1) });
        }
    }

    best
}

fn best_plan(answer_space: &mut HashMap<(Vec4d,Vec4d,i32),i32>, factory: &Vec<Vec4d>, resources: &Vec4d, robots: &Vec4d, turns_left: i32, turn_tracker: &mut i32) -> i32 {
    match answer_space.get(&(*resources,*robots,turns_left)) {
        Some(cached_answer) => { return *cached_answer; }
        _ => {
            let mut computed_answer = 0xff;
            if turns_left==*turn_tracker {
                println!("Executed turns left {turns_left}");
                *turn_tracker += 1;
            }
            // try to prune a bit
            if turns_left == 0 {
                computed_answer = resources[3];
            }
            // if turns_left==0 {} !might_be_possible_to_eventually_build_geode_bot(factory, robots, resources, turns_left) { 
            //     computed_answer = theoretical_resource_production(robots, turns_left)[3]; 
            // }
            else {
                let new_resources = v4add( resources, &robots ); 
                for (i, robot_choice) in factory.iter().enumerate() {
                    if affordable(&robot_choice, resources) {
                        let remainder = v4sub( &new_resources, robot_choice);
                        let new_robots = v4add(robots, &get_elements_for_robot_idx()[i]);
                        let this_choice = best_plan( answer_space, &factory, &remainder, &new_robots, turns_left-1, turn_tracker);
                        computed_answer = std::cmp::max(computed_answer, this_choice as u8);
                    }
                }
                // or build nothing this round
                let last_choice = best_plan( answer_space, &factory, &new_resources, &robots, turns_left-1, turn_tracker);
                computed_answer = std::cmp::max(computed_answer, last_choice as u8);
            }
            answer_space.insert((*resources,*robots,turns_left), computed_answer as i32);

            return computed_answer as i32;
        }
    };
}

// #[test]
// fn test_best_plan() {
//     println!("Hello?");
//     let factory = vec![[4,0,0,0],[2,0,0,0],[3,14,0,0],[2,0,7,0]];
//     let mut answer_space = HashMap::<(Vec4d,Vec4d,i32),i32>::new();
//     let mut turn_tracker = 0i32;
//     let answer = best_plan(&mut answer_space, &factory, &[5,37,6,7], &[1,4,2,2], 1, &mut turn_tracker);
//     assert_eq!(9, answer);    
//     let answer = best_plan(&mut answer_space, &factory, &[4,33,4,5], &[1,4,2,2], 2, &mut turn_tracker);
//     assert_eq!(9, answer);        
//     let answer = best_plan(&mut answer_space, &factory, &[3,29,2,3], &[1,4,2,2], 3, &mut turn_tracker);
//     assert_eq!(9, answer); 
//     let answer = best_plan(&mut answer_space, &factory, &[4,25,7,2], &[1,4,2,1], 4, &mut turn_tracker);
//     assert_eq!(9, answer); 
//     let answer = best_plan(&mut answer_space, &factory, &[0,0,0,0], &[1,0,0,0], 24, &mut turn_tracker);
//     assert_eq!(9, answer); 
// }

#[test]
fn test_bfs_from_state() {
    println!("Hello?");
    let factory = vec![[4,0,0,0],[2,0,0,0],[3,14,0,0],[2,0,7,0]];
    let mut turn_tracker = 0i32;
    let answer = bfs_from_state(&factory, &[5,37,6,7], &[1,4,2,2], 1);
    assert_eq!(9, answer);    
    let answer = bfs_from_state( &factory, &[4,33,4,5], &[1,4,2,2], 2);
    assert_eq!(9, answer);        
    let answer = bfs_from_state(&factory, &[3,29,2,3], &[1,4,2,2], 3);
    assert_eq!(9, answer); 
    let answer = bfs_from_state(&factory, &[4,25,7,2], &[1,4,2,1], 4);
    assert_eq!(9, answer); 
    let answer = bfs_from_state(&factory, &[0,0,0,0], &[1,0,0,0], 24);
    assert_eq!(9, answer); 
}

#[test]
fn test_bfs() {
    println!("Hello?");
    let factory = vec![[4,0,0,0],[2,0,0,0],[3,14,0,0],[2,0,7,0]];
    let answer = bfs(&factory, 24);
    assert_eq!(9, answer);
}

// #[test]
// fn test_best_plan_2() {
//     println!("Hello?");
//     let factory = vec![[2,0,0,0],[3,0,0,0],[3,8,0,0],[3,0,12,0]];
//     let mut answer_space = HashMap::<(Vec4d,Vec4d,i32),i32>::new();
//     let mut turn_tracker = 0i32;
//     let answer = best_plan(&mut answer_space, &factory, &[0,0,0,0], &[1,0,0,0], 24, &mut turn_tracker);
//     assert_eq!(12, answer); 
// }

// #[test]
// fn test_sample_blueprint_2() {
//     let blueprints = parse_input(get_sample_input());
//     let mut answer_space = HashMap::<(Vec4d,Vec4d,i32),i32>::new();
//     let mut turn_tracker = 0i32;
//     let answer = best_plan(&mut answer_space, &blueprints[1], &[0,0,0,0], &[1,0,0,0], 24, &mut turn_tracker);
//     assert_eq!(12, answer);
// }

#[test]
fn test_sample_blueprint_2() {
    let blueprints = parse_input(get_sample_input());
    let mut answer_space = HashMap::<(Vec4d,Vec4d,i32),i32>::new();
    let mut turn_tracker = 0i32;
    let answer = bfs( &blueprints[1],  24);
    assert_eq!(12, answer);
 }

fn parse_input(input: &str) -> Vec<Vec<Vec4d>> {
    let mut blueprints = vec![];

    for line in input.lines() {
        let mut blueprint_idx = usize::MAX;
        let mut robot_data_str = String::new();
        let mut blueprint: Vec<Vec4d> = vec![];
        match sscanf!(&line, "Blueprint {}: {}", blueprint_idx, robot_data_str) {
            Ok(_) => {
                for entry in robot_data_str.split(". ") {
                    let mut flavor = String::new();
                    let mut costs = String::new();
                    let mut cost_vector = [0,0,0,0];
                    match sscanf!(entry, "Each {} robot costs {}", flavor, costs) {
                        Ok(_) => {
                            for cost in costs.split(" and ") {
                                let mut resource_cost = 0xff;
                                let mut resource = String::new();
                                match sscanf!(cost, "{} {}", resource_cost, resource) {
                                    Ok(_) => { 
                                        for (i, name) in RESOURCE_NAMES.iter().enumerate() {
                                            if resource.contains(*name) {  // doing contains rather than == because there's a trailing period on last thingy
                                                cost_vector[i] = resource_cost;
                                            }
                                        }

                                    }
                                    _ => { panic!(); }
                                }
                            }
                        },
                        _ => { panic!(); }
                    }
                    blueprint.push(cost_vector);
                }
            }
            _ => { panic!(); }
        }
        
        blueprints.push(blueprint);
        
        assert_eq!(blueprint_idx, blueprints.len());
    }

    blueprints
}

#[test]
fn test_parse_sample() {
    let result = parse_input(get_sample_input());
    assert_eq!(2, result.len());
    assert_eq!(result[0][0] , [4,0,0,0]);
    assert_eq!(result[0][2] , [3,14,0,0]);
    assert_eq!(result[0][3] , [2,0,7,0]);
    assert_eq!(result[1][0] , [2,0,0,0]);
    assert_eq!(result[1][3] , [3,0,12,0]);

}

fn get_sample_input() -> &'static str {
"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian."
}

fn get_puzzle_input() -> &'static str {
"Blueprint 1: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 16 clay. Each geode robot costs 2 ore and 9 obsidian.
Blueprint 2: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 14 clay. Each geode robot costs 2 ore and 16 obsidian.
Blueprint 3: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 12 clay. Each geode robot costs 3 ore and 8 obsidian.
Blueprint 4: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 14 clay. Each geode robot costs 3 ore and 8 obsidian.
Blueprint 5: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 15 clay. Each geode robot costs 3 ore and 7 obsidian.
Blueprint 6: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 8 clay. Each geode robot costs 2 ore and 10 obsidian.
Blueprint 7: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 16 clay. Each geode robot costs 3 ore and 14 obsidian.
Blueprint 8: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 7 clay. Each geode robot costs 3 ore and 8 obsidian.
Blueprint 9: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 17 clay. Each geode robot costs 3 ore and 11 obsidian.
Blueprint 10: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 6 clay. Each geode robot costs 4 ore and 11 obsidian.
Blueprint 11: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 9 clay. Each geode robot costs 3 ore and 15 obsidian.
Blueprint 12: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 14 clay. Each geode robot costs 3 ore and 16 obsidian.
Blueprint 13: Each ore robot costs 4 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 8 clay. Each geode robot costs 4 ore and 14 obsidian.
Blueprint 14: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 19 clay. Each geode robot costs 2 ore and 12 obsidian.
Blueprint 15: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 9 clay. Each geode robot costs 3 ore and 9 obsidian.
Blueprint 16: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 4 ore and 19 clay. Each geode robot costs 4 ore and 12 obsidian.
Blueprint 17: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 17 clay. Each geode robot costs 4 ore and 16 obsidian.
Blueprint 18: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 7 clay. Each geode robot costs 2 ore and 9 obsidian.
Blueprint 19: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 18 clay. Each geode robot costs 4 ore and 12 obsidian.
Blueprint 20: Each ore robot costs 2 ore. Each clay robot costs 4 ore. Each obsidian robot costs 3 ore and 19 clay. Each geode robot costs 4 ore and 13 obsidian.
Blueprint 21: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 19 clay. Each geode robot costs 2 ore and 12 obsidian.
Blueprint 22: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 16 clay. Each geode robot costs 2 ore and 18 obsidian.
Blueprint 23: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 13 clay. Each geode robot costs 2 ore and 9 obsidian.
Blueprint 24: Each ore robot costs 2 ore. Each clay robot costs 2 ore. Each obsidian robot costs 2 ore and 10 clay. Each geode robot costs 2 ore and 11 obsidian.
Blueprint 25: Each ore robot costs 3 ore. Each clay robot costs 3 ore. Each obsidian robot costs 2 ore and 16 clay. Each geode robot costs 3 ore and 14 obsidian.
Blueprint 26: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 18 clay. Each geode robot costs 2 ore and 19 obsidian.
Blueprint 27: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 20 clay. Each geode robot costs 4 ore and 7 obsidian.
Blueprint 28: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 4 ore and 18 clay. Each geode robot costs 4 ore and 11 obsidian.
Blueprint 29: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 4 ore and 8 clay. Each geode robot costs 3 ore and 7 obsidian.
Blueprint 30: Each ore robot costs 4 ore. Each clay robot costs 3 ore. Each obsidian robot costs 4 ore and 20 clay. Each geode robot costs 4 ore and 8 obsidian."
}