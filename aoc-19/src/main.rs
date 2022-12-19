use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode
}

type Vec4d = [i32;4];

fn affordable(cost: &Vec4d, resources: &Vec4d) -> bool {
    for i in 0..cost.len() {
        if cost[i] > resources[i] { return false;}
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

fn v4sub(cost: &Vec4d, resources: &Vec4d) -> Vec4d {
    let mut remainder = [0,0,0,0];
    for i in 0..cost.len() {
        remainder[i] = resources[i]-cost[i];
    }

    remainder
}

fn get_elements_for_robot_idx() -> &'static [Vec4d;4] {
    &[[1,0,0,0],[0,1,0,0],[0,0,1,0],[0,0,0,1]]
}

fn best_plan(answer_space: &mut HashMap<(Vec4d,Vec4d,i32),i32>, factory: &Vec<Vec4d>, resources: &Vec4d, robots: &Vec4d, turns_left: i32) -> i32 {
    match answer_space.get(&(*resources,*robots,turns_left)) {
        Some(cached_answer) => { return *cached_answer; }
        _ => {
            let mut computed_answer = -1;
            if turns_left==0 { 
                computed_answer = resources[3]; 
            }
            else {
                let new_resources = v4add( resources, &robots ); 
                for (i, robot_choice) in factory.iter().enumerate() {
                    if affordable(&robot_choice, resources) {
                        let remainder = v4sub(robot_choice, &new_resources);
                        let new_robots = v4add(robots, &get_elements_for_robot_idx()[i]);
                        let this_choice = best_plan( answer_space, &factory, &remainder, &new_robots, turns_left-1);
                        computed_answer = std::cmp::max(computed_answer, this_choice);
                    }
                }
                // or build nothing this round
                let last_choice = best_plan( answer_space, &factory, &new_resources, &robots, turns_left-1);
                computed_answer = std::cmp::max(computed_answer, last_choice);
                if turns_left >= 19 {
                    println!("Turns left {turns_left}");
                }
            }
            answer_space.insert((resources.clone(),robots.clone(),turns_left), computed_answer);

            return computed_answer;
        }
    };
}

#[test]
fn test_best_plan() {
    println!("Hello?");
    let factory = vec![[4,0,0,0],[2,0,0,0],[3,14,0,0],[2,0,7,0]];
    let mut answer_space = HashMap::<(Vec4d,Vec4d,i32),i32>::new();
    let answer = best_plan(&mut answer_space, &factory, &[5,37,6,7], &[1,4,2,2], 1);
    assert_eq!(9, answer);    
    let answer = best_plan(&mut answer_space, &factory, &[4,33,4,5], &[1,4,2,2], 2);
    assert_eq!(9, answer);        
    let answer = best_plan(&mut answer_space, &factory, &[3,29,2,3], &[1,4,2,2], 3);
    assert_eq!(9, answer); 
    let answer = best_plan(&mut answer_space, &factory, &[4,25,7,2], &[1,4,2,1], 4);
    assert_eq!(9, answer); 
    let answer = best_plan(&mut answer_space, &factory, &[0,0,0,0], &[1,0,0,0], 24);
    assert_eq!(9, answer); 
}

// fn parse_input(input: &str) {
//     for 
// }

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