use std::collections::HashMap;
use scanf::sscanf;


static RESOURCE_NAMES: [&str;4] = ["ore","clay","obsidian","geode"];

type Vec4d = [i32;4];

fn main() {
    println!("Hello, world!");
    let answer = do_the_thing(get_puzzle_input());
    println!("Boom? {answer}")
}

fn do_the_thing(input: &str) -> usize {
    let blueprints = parse_input(&input);
    let mut total_quality: usize = 0;
    for (i, blueprint) in blueprints.iter().enumerate() {
        println!("blueprint {i}/{}", blueprints.len());
        let mut turn_tracker = 0i32;
        let mut answer_space = HashMap::<(Vec4d,Vec4d,i32),i32>::new();
        let geodes = best_plan(&mut answer_space, &blueprint, &[0,0,0,0], &[1,0,0,0], 24, &mut turn_tracker);
        let quality = (geodes as usize) * (i+1);
        total_quality += quality;
    }

    total_quality
}

#[test]
fn test_do_the_thing() {
    let answer = do_the_thing(get_sample_input());
    assert_eq!(33, answer);
}

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

fn v4mul(v1: &Vec4d, v2: &Vec4d) -> Vec4d {
    [v1[0]*v2[0],v1[1]*v2[1],v1[2]*v2[2],v1[3]*v2[3]]
}

fn get_elements_for_robot_idx() -> &'static [Vec4d;4] {
    &[[1,0,0,0],[0,1,0,0],[0,0,1,0],[0,0,0,1]]
}

fn theoretical_resource_production(robots: &Vec4d, turns: i32)->Vec4d {
    let turns = std::cmp::max(turns,0);
    [robots[0]*turns,robots[1]*turns,robots[2]*turns,robots[3]*turns]
}

// pruning
fn might_be_possible_to_eventually_build_geode_bot(blueprints: &Vec<Vec4d>, robots: &Vec4d, resources: &Vec4d, turns_left: i32) -> bool {
    if affordable(&blueprints[3], &v4add(resources, &theoretical_resource_production(robots, turns_left-1))) {
        return true;
    }
    return false; 
}

#[test]
fn test_might_be_possible_to_eventually_build_geode_bot() {
    let blueprints = vec![[1,0,0,0],[1,0,0,0],[1,1,0,0],[1,0,1,0]];
    let answer = might_be_possible_to_eventually_build_geode_bot(&blueprints, &[1,1,0,0], &[10,10,0,0], 10);
    assert!(!answer);
    let blueprints = vec![[1,0,0,0],[1,0,0,0],[1,1,0,0],[10,0,10,0]];
    let answer = might_be_possible_to_eventually_build_geode_bot(&blueprints, &[1,0,1,0], &[10,10,0,0], 10);
    assert!(answer);
    let answer = might_be_possible_to_eventually_build_geode_bot(&blueprints, &[1,0,1,0], &[10,10,0,0], 9);
    assert!(!answer);
    
}

fn best_plan(answer_space: &mut HashMap<(Vec4d,Vec4d,i32),i32>, factory: &Vec<Vec4d>, resources: &Vec4d, robots: &Vec4d, turns_left: i32, turn_tracker: &mut i32) -> i32 {
    match answer_space.get(&(*resources,*robots,turns_left)) {
        Some(cached_answer) => { return *cached_answer; }
        _ => {
            let mut computed_answer = -1;
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
                        let remainder = v4sub(robot_choice, &new_resources);
                        let new_robots = v4add(robots, &get_elements_for_robot_idx()[i]);
                        let this_choice = best_plan( answer_space, &factory, &remainder, &new_robots, turns_left-1, turn_tracker);
                        computed_answer = std::cmp::max(computed_answer, this_choice);
                    }
                }
                // or build nothing this round
                let last_choice = best_plan( answer_space, &factory, &new_resources, &robots, turns_left-1, turn_tracker);
                computed_answer = std::cmp::max(computed_answer, last_choice);
            }
            answer_space.insert((*resources,*robots,turns_left), computed_answer);

            return computed_answer;
        }
    };
}

#[test]
fn test_best_plan() {
    println!("Hello?");
    let factory = vec![[4,0,0,0],[2,0,0,0],[3,14,0,0],[2,0,7,0]];
    let mut answer_space = HashMap::<(Vec4d,Vec4d,i32),i32>::new();
    let mut turn_tracker = 0i32;
    let answer = best_plan(&mut answer_space, &factory, &[5,37,6,7], &[1,4,2,2], 1, &mut turn_tracker);
    assert_eq!(9, answer);    
    let answer = best_plan(&mut answer_space, &factory, &[4,33,4,5], &[1,4,2,2], 2, &mut turn_tracker);
    assert_eq!(9, answer);        
    let answer = best_plan(&mut answer_space, &factory, &[3,29,2,3], &[1,4,2,2], 3, &mut turn_tracker);
    assert_eq!(9, answer); 
    let answer = best_plan(&mut answer_space, &factory, &[4,25,7,2], &[1,4,2,1], 4, &mut turn_tracker);
    assert_eq!(9, answer); 
    let answer = best_plan(&mut answer_space, &factory, &[0,0,0,0], &[1,0,0,0], 24, &mut turn_tracker);
    assert_eq!(9, answer); 
}

#[test]
fn test_best_plan_2() {
    println!("Hello?");
    let factory = vec![[2,0,0,0],[3,0,0,0],[3,8,0,0],[3,0,12,0]];
    let mut answer_space = HashMap::<(Vec4d,Vec4d,i32),i32>::new();
    let mut turn_tracker = 0i32;
    let answer = best_plan(&mut answer_space, &factory, &[0,0,0,0], &[1,0,0,0], 24, &mut turn_tracker);
    assert_eq!(12, answer); 
}

#[test]
fn test_sample_blueprint_2() {
    let blueprints = parse_input(get_sample_input());
    let mut answer_space = HashMap::<(Vec4d,Vec4d,i32),i32>::new();
    let mut turn_tracker = 0i32;
    let answer = best_plan(&mut answer_space, &blueprints[1], &[0,0,0,0], &[1,0,0,0], 24, &mut turn_tracker);
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
                                let mut resource_cost = -1;
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