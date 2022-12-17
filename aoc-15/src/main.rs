use scanf::sscanf;
use std::cmp::{min,max};

fn main() {
    println!("Hello, world!");
    let answer = do_the_other_thing(get_puzzle_input(), 4000001);
    println!("answer: {answer}");
}

type Sensor=((i64,i64),(i64,i64));

fn do_the_thing(input: &str, row: i64) -> i64 {
    let sensors = parse_input(input);
    let spans = get_beacon_exclusion_spans(&sensors, row);

    // hmm, brute force - need to do 3,000,000 tests * 20...
    // that's easier than coalescing spans

    check_row(&spans, row, 4000000).0
}

fn do_the_other_thing(input: &str, bound: usize) -> i64 {
    let sensors = parse_input(input);
    // let mut graph = vec![vec![0;21];21];
    // graph_beacon_exclusion_areas(&mut graph, &sensors);
    // for row in &graph {
    //     println!("{:?}", row);
    // }
    // let results = search_graph(&graph);
    // println!("{:?}", results);
    // assert_eq!(1, results.len());

    // results[0].0*4000000+results[0].1
    let mut beacon : Option<(i64,i64)> = None;
    for y in 0..bound {
        if (y%4000==0) {
            println!("row {y}");
        }
        let spans = get_beacon_exclusion_spans(&sensors, y as i64);
        let spans = merge_spans(&spans);
        match spans.iter().find(|span| span.0 >0 && span.0 <=bound as i64 ) {
            Some(span) => { 
                beacon = Some((span.0-1,y as i64)); 
                break; 
            }
            _ => {}
        }
    }

    beacon.unwrap().0*4000000+beacon.unwrap().1
}

fn search_graph(graph: &Vec<Vec<u8>>) -> Vec<(i64,i64)> {
    let mut spots: Vec<(i64,i64)> = vec![];
    for x in 0..graph.len() {
        for y in 0..graph.len() {
            if graph[y as usize][x as usize] == 0 {
                spots.push((x as i64,y as i64));
            }
        }
    }

    spots
}
// also puts the last non-excluded x-coordinate in result.1
fn check_row(spans: &Vec<(i64,i64)>, row: i64, bound: usize) -> (i64,Option<i64>) {
    // let min = spans.iter().map(|span| span.0).min().unwrap();
    // let max = spans.iter().map(|span| span.1).max().unwrap();
    let mut exclusion_count = 0;
    let mut last_good_x: Option<i64> = None;
    for x in 0..bound {
        let mut excluded = false;
        for span in spans {
            if x as i64>=span.0 && x as i64 <=span.1 { 
                excluded = true;
                break;
            }
        }
        if excluded {
            exclusion_count += 1;
        }
        else {
            last_good_x = Some(x as i64);
        }
    }

    (exclusion_count, last_good_x)
}

#[test]
fn test_do_the_thing() {
    let answer = do_the_thing(get_sample_input(), 10);
    assert_eq!(26, answer);
}

#[test]
fn test_do_the_other_thing() {
    let answer = do_the_other_thing(get_sample_input(), 21);
    assert_eq!(56000011, answer);
}

fn parse_input(input: &str) -> Vec<Sensor> {
    let mut sensors : Vec<Sensor> = vec![];
    for line in input.lines() {
        let mut sensor_0 = 0i64;
        let mut sensor_1 = 0i64;
        let mut beacon_0 = 0i64;
        let mut beacon_1 = 0i64;
        sscanf!(line, "Sensor at x={}, y={}: closest beacon is at x={}, y={}",sensor_0,sensor_1,beacon_0,beacon_1);
        sensors.push(((sensor_0,sensor_1),(beacon_0,beacon_1)));
    }

    sensors
}

fn spans_overlap(span0: &(i64,i64), span1: &(i64,i64)) -> Option<(i64,i64)> {
    if span0.0 >= span1.0 && span0.0 <= span1.1+1 {
        Some((span1.0,max(span0.1,span1.1)))
    }
    else if span1.0 >= span0.0 && span1.0 <= span0.1+1 {
        Some((span0.0,max(span0.1,span1.1)))
    }
    else {
        None
    }
}

#[test]
fn test_spans_overlap() {
    
    assert_eq!(None,spans_overlap(&(0,2),&(4,5)));
    assert_eq!(None,spans_overlap(&(4,5),&(0,2)));
    assert_eq!(Some((0,4)),spans_overlap(&(0,2),&(3,4)));
    assert_eq!(Some((0,4)),spans_overlap(&(3,4),&(0,2)));
    assert_eq!(Some((0,4)),spans_overlap(&(0,2),&(2,4)));
    assert_eq!(Some((0,4)),spans_overlap(&(2,4),&(0,2)));
}

fn merge_spans(spans: &Vec<(i64,i64)>) -> Vec<(i64,i64)> {
    let mut new_spans : Vec<(i64,i64)> = vec![];

    for span in spans {
        let mut merged_span = span.clone();
        for (i, new_span) in new_spans.iter().enumerate() {
            match spans_overlap(&merged_span, &new_span) {
                Some(new_merged_span) => {
                    merged_span = new_merged_span;
                }
                _ => {}
            }
        }
        new_spans.retain(|span| spans_overlap(&merged_span, &span)==None);
        new_spans.push(merged_span);
    }

    new_spans
}

#[test]
fn test_merge_spans() {
    let spans = vec![];
    let spans = merge_spans(&spans);
    assert_eq!(0,spans.len());
    let spans = vec![(0,2)];
    let spans = merge_spans(&spans);
    assert_eq!(vec![(0,2)], spans);
    let spans = vec![(0,2),(1,3)];
    let spans = merge_spans(&spans);
    assert_eq!(vec![(0,3)], spans);
    let spans = vec![(0,2),(3,4)];
    let spans = merge_spans(&spans);
    assert_eq!(vec![(0,4)], spans);
    let spans = vec![(0,2),(5,6),(3,4)];
    let spans = merge_spans(&spans);
    assert_eq!(vec![(0,6)], spans);
    let spans = vec![(0,2),(9,8),(3,4),(5,6)];
    let spans = merge_spans(&spans);
    assert_eq!(vec![(9,8),(0,6)], spans);
    let spans = vec![(0,2),(9,8),(3,4),(5,6),(1,1)];
    let spans = merge_spans(&spans);
    assert_eq!(vec![(9,8),(0,6)], spans);
}

fn get_beacon_exclusion_spans(sensors: &Vec<Sensor>, row: i64) -> Vec<(i64,i64)> {
    let mut spans: Vec<(i64,i64)> = vec![];
    for sensor in sensors {
        match get_beacon_exclusion_span(sensor, row) {
            Some(span) => spans.push(span),
            None => {}
        }
    }

    spans
}

fn get_beacon_exclusion_span(sensor: &Sensor, row: i64) -> Option<(i64,i64)> {    
    let dist = manhattan_distance(&sensor.0, &sensor.1);
    // example: sensor is at 0,0; beacon distance is 10; exlusion distance is 9; row 10 has no span, row 9 has span of (0,0), row 8 has a span of (-1,1), row 0 has a span of (-9,9)
    // apparently I'm wrong about that? I was off by 1. My test is wrong
    let row_distance = (row - sensor.0.1).abs();
    let span_radius = dist - row_distance;
    if span_radius >= 0 {
        Some((sensor.0.0-span_radius,sensor.0.0+span_radius))
    }
    else {
        None
    }
}

#[test]
fn test_beacon_exclusion_span() {
    let sensor = ((2,2),(0,1));
    assert_eq!(None, get_beacon_exclusion_span(&sensor, -1));
    assert_eq!(Some((2,2)), get_beacon_exclusion_span(&sensor, 0));
    assert_eq!(Some((1,3)), get_beacon_exclusion_span(&sensor, 1));
    assert_eq!(Some((0,4)), get_beacon_exclusion_span(&sensor, 2));
    assert_eq!(Some((1,3)), get_beacon_exclusion_span(&sensor, 3));
    assert_eq!(Some((2,2)), get_beacon_exclusion_span(&sensor, 4));
    assert_eq!(None, get_beacon_exclusion_span(&sensor, 5));
}

fn graph_beacon_exclusion_areas(graph: &mut Vec<Vec<u8>>, sensors: &Vec<Sensor>) {
    for sensor in sensors {
        graph_beacon_exclusion_area(graph, &sensor);
    }
}

fn manhattan_distance(vec2d0: &(i64,i64), vec2d1: &(i64,i64)) -> i64 {
    (vec2d1.0-vec2d0.0).abs()+(vec2d1.1-vec2d0.1).abs()
}

fn graph_beacon_exclusion_area(graph: &mut Vec<Vec<u8>>, sensor: &Sensor) {
    let dist = manhattan_distance(&sensor.0, &sensor.1);
    for x in 0 .. graph.len() {
        for y in 0 .. graph.len() {
            if manhattan_distance(&sensor.0, &(x as i64,y as i64)) <= dist {
                graph[y as usize][x as usize] = 1;
            }
        }
    }
}

#[test]
fn test_graph_beacon_etc() {
    let mut graph = vec![vec![0;5];5];
    let sensor = ((2,2),(0,1));
    graph_beacon_exclusion_area(&mut graph, &sensor);
    assert_eq!(vec![0,0,1,0,0],graph[0]);
    assert_eq!(vec![0,1,1,1,0],graph[1]);
    assert_eq!(vec![1,1,1,1,1],graph[2]);
    assert_eq!(vec![0,1,1,1,0],graph[3]);
    assert_eq!(vec![0,0,1,0,0],graph[4]);
}


#[test]
fn test_parse_input() {
    let result = parse_input("Sensor at x=1, y=-2: closest beacon is at x=-3, y=4\nSensor at x=-5, y=6: closest beacon is at x=7, y=-8");
    assert_eq!(vec![((1,-2),(-3,4)),((-5,6),(7,-8))], result);
}

fn get_sample_input() -> &'static str {
"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3"
}

fn get_puzzle_input() -> &'static str {
"Sensor at x=2885528, y=2847539: closest beacon is at x=2966570, y=2470834
Sensor at x=2224704, y=1992385: closest beacon is at x=2018927, y=2000000
Sensor at x=3829144, y=1633329: closest beacon is at x=2966570, y=2470834
Sensor at x=43913, y=426799: closest beacon is at x=152363, y=369618
Sensor at x=2257417, y=2118161: closest beacon is at x=2386559, y=2090397
Sensor at x=8318, y=3994839: closest beacon is at x=-266803, y=2440278
Sensor at x=69961, y=586273: closest beacon is at x=152363, y=369618
Sensor at x=3931562, y=3361721: closest beacon is at x=3580400, y=3200980
Sensor at x=476279, y=3079924: closest beacon is at x=-266803, y=2440278
Sensor at x=2719185, y=2361091: closest beacon is at x=2966570, y=2470834
Sensor at x=2533382, y=3320911: closest beacon is at x=2260632, y=3415930
Sensor at x=3112735, y=3334946: closest beacon is at x=3580400, y=3200980
Sensor at x=1842258, y=3998928: closest beacon is at x=2260632, y=3415930
Sensor at x=3712771, y=3760832: closest beacon is at x=3580400, y=3200980
Sensor at x=1500246, y=2684955: closest beacon is at x=2018927, y=2000000
Sensor at x=3589321, y=142859: closest beacon is at x=4547643, y=-589891
Sensor at x=1754684, y=2330721: closest beacon is at x=2018927, y=2000000
Sensor at x=2476631, y=3679883: closest beacon is at x=2260632, y=3415930
Sensor at x=27333, y=274008: closest beacon is at x=152363, y=369618
Sensor at x=158732, y=2405833: closest beacon is at x=-266803, y=2440278
Sensor at x=2955669, y=3976939: closest beacon is at x=3035522, y=4959118
Sensor at x=1744196, y=13645: closest beacon is at x=152363, y=369618
Sensor at x=981165, y=1363480: closest beacon is at x=2018927, y=2000000
Sensor at x=2612279, y=2151377: closest beacon is at x=2386559, y=2090397
Sensor at x=3897, y=2076376: closest beacon is at x=-266803, y=2440278
Sensor at x=2108479, y=1928318: closest beacon is at x=2018927, y=2000000
Sensor at x=1913043, y=3017841: closest beacon is at x=2260632, y=3415930
Sensor at x=2446778, y=785075: closest beacon is at x=2386559, y=2090397
Sensor at x=2385258, y=2774943: closest beacon is at x=2386559, y=2090397
Sensor at x=3337656, y=2916144: closest beacon is at x=3580400, y=3200980
Sensor at x=380595, y=66906: closest beacon is at x=152363, y=369618
Sensor at x=1593628, y=3408455: closest beacon is at x=2260632, y=3415930"
}