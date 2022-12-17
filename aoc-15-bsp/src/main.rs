use scanf::sscanf;
use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}

#[derive(PartialEq)]

// I originally thought integer math would work, but am now pretty sure
// that if the points are in the center of squares the segments are on .5 dividers
struct Vector2d {
    x: f64,
    y: f64,
}

#[derive(PartialEq)]
struct Segment {
    p0: Vector2d,
    p1: Vector2d,
    normal: Vector2d  
}

struct Bounds {
    left: f64, top: f64, right: f64, bottom: f64
}

type Sensor=(Vector2d, Vector2d);

fn sub(vec0: &Vector2d, vec1: &Vector2d) -> Vector2d {
    Vector2d { x: vec0.x-vec1.x, y: vec0.y-vec1.y }
}

fn dot(vec0: &Vector2d, vec1: &Vector2d) -> f64 {
    vec0.x*vec1.x+vec0.y*vec1.y
}
#[derive(Debug, PartialEq, Clone)]
enum Zone {
    Excluded,
    Included
}

enum Bsp {
    Bsp {
        segment: Segment,
        towards: Box<Bsp>,  // normal points at this side
        away: Box<Bsp>
    },
    Leaf {
        zone: Zone,
        segments: vec![segments]
    }
}

fn do_the_crazy_thing(input: &str, bound: f64) {
    let sensors = parse_input(get_my_input());
    let mut segment_soup = sensors_to_segment_soup(sensors);

}

fn approx_equal(v1: f64, v2: f64, epsilon: f64) -> bool {
    v1 >= v2 - epsilon && v1 <= v2 + epsilon
}

fn partition_space(segments: &mut Vec<Segment>) -> Option<Bsp> {
    let mut segment_iter = segments.iter();
    match segment_iter.next() {
        None => { None }
        Some(arbitrary_segment) => {
            let mut segments_towards: Vec<Segment> = vec![];
            let mut segments_away: Vec<Segment> = vec![];
            for segment in segment_iter {
                let point0side = dot(segment.p0-arbitrary_segment.p0,&arbitrary_segment.normal);
                let point1side: f64 = dot(segment.p1-arbitrary_segment.p0,&arbitrary_segment.normal);
                if approx_equal(point0side, 0.0, 0.00001) && approx_equal(point1side, 0.0f, epsilon) {
                    panic!();  // no idea what to do here. maybe it won't come up :P
                }
                else if point0side<0.0 && point1side<=0.0 {
                    segments_away.push(segment);
                }
                else if point0side>0.0 && point1side>=0.0 {
                    segments_towards.push(segment);
                }
                else 
            }
            let mut new_bsp = Bsp::Bsp{segment: arbitrary_segment
        }
    }
        Some(Bsp::Bsp{segment: segments[]}
    }
}

fn bsp_check_point(bsp: &Bsp, test_point: &Vector2d) -> Zone {
    match bsp {
        Bsp::Leaf(zone) => {
            zone.clone()
        },
        Bsp::Bsp{ point, normal, towards, away } => {
            if dot(&sub(test_point, point), normal) > 0.0 {
                bsp_check_point(towards, test_point)
            }        
            else {
                bsp_check_point(away, test_point)
            }
        }
    }
}




// I don't know what I'm doing
fn sensors_to_segment_soup(sensors: Vec<Sensor>) -> Vec<Segment> {
    let mut segments = vec![];
    for sensor in sensors {
        let md = manhattan_distance(&sensor.0, &sensor.1);
        let md = if md > 0.0 { md-0.5 } else {0.0};
        segments.push(Segment{p0:Vector2d{x:sensor.0.x,y:sensor.0.y-md},p1:Vector2d{x:sensor.0.x+md,y:sensor.0.y},normal:Vector2d{x:1.0,y:-1.0}});
        segments.push(Segment{p0:Vector2d{x:sensor.0.x,y:sensor.0.y+md},p1:Vector2d{x:sensor.0.x+md,y:sensor.0.y},normal:Vector2d{x:1.0,y:1.0}});
        segments.push(Segment{p0:Vector2d{x:sensor.0.x,y:sensor.0.y-md},p1:Vector2d{x:sensor.0.x-md,y:sensor.0.y},normal:Vector2d{x:-1.0,y:-1.0}});
        segments.push(Segment{p0:Vector2d{x:sensor.0.x,y:sensor.0.y+md},p1:Vector2d{x:sensor.0.x-md,y:sensor.0.y},normal:Vector2d{x:-1.0,y:1.0}});
    }

    segments
}

// this is incorrect, both points could be outside. would need to clip
fn bounds_check_segment_soup(segments: &mut Vec<Segment>, bounds: &Bounds) {
    segments.retain(|segment| in_bounds(&segment.p0, bounds)||in_bounds(&segment.p1, bounds))
}



// y goes down in the example
fn in_bounds(point: &Vector2d, bounds: &Bounds) -> bool {
    point.x >= bounds.left && point.x <= bounds.right && point.y >= bounds.top && point.y <= bounds.bottom
}

#[test]
fn test_sensors_to_segment_soup() {
    let sensors = vec![(Vector2d{x:0.0,y:0.0},Vector2d{x:1.0,y:2.0})];
    let segments = sensors_to_segment_soup(sensors);
    match segments.iter().find(|seg| seg.p0==Vector2d{x:0.0,y:2.5} && seg.p1==Vector2d{x:2.5,y:0.0} && seg.normal==Vector2d{x:1.0,y:1.0}) {
        Some(_) => {}
        _ => {panic!();}
    }
    match segments.iter().find(|seg| seg.p0==Vector2d{x:0.0,y:-2.5} && seg.p1==Vector2d{x:2.5,y:0.0} && seg.normal==Vector2d{x:1.0,y:-1.0}) {
        Some(_) => {}
        _ => {panic!();}
    }
    match segments.iter().find(|seg| seg.p0==Vector2d{x:0.0,y:2.5} && seg.p1==Vector2d{x:-2.5,y:0.0} && seg.normal==Vector2d{x:-1.0,y:1.0}) {
        Some(_) => {}
        _ => {panic!();}
    }
    match segments.iter().find(|seg| seg.p0==Vector2d{x:0.0,y:-2.5} && seg.p1==Vector2d{x:-2.5,y:0.0} && seg.normal==Vector2d{x:-1.0,y:-1.0}) {
        Some(_) => {}
        _ => {panic!();}
    }
}

fn manhattan_distance(vec2d0: &Vector2d, vec2d1: &Vector2d) -> f64 {
    (vec2d1.x-vec2d0.x).abs()+(vec2d1.y-vec2d0.y).abs()
}

#[test]
fn test_experiment() {
    let mut bsp: Bsp = Bsp::Bsp{ point: Vector2d{ x: 0.0, y: 0.0}
                                , normal: Vector2d{x:1.0, y:0.0}
                                , away: Box::new(Bsp::Leaf(Zone::Excluded))
                                , towards: Box::new(
                                    Bsp::Bsp{ point: Vector2d{ x: 0.0, y: 0.0 }
                                            , normal: Vector2d{ x: 0.0, y: 1.0 }
                                            , towards: Box::new(Bsp::Leaf(Zone::Included))
                                            , away: Box::new(Bsp::Leaf(Zone::Excluded))})};
    
    assert_eq!(Zone::Excluded, bsp_check_point(&bsp, &Vector2d{ x: -1.0, y: -1.0 }));
    assert_eq!(Zone::Excluded, bsp_check_point(&bsp, &Vector2d{ x: -1.0, y: 1.0 }));
    assert_eq!(Zone::Excluded, bsp_check_point(&bsp, &Vector2d{ x: 1.0, y: -1.0 }));
    assert_eq!(Zone::Included, bsp_check_point(&bsp, &Vector2d{ x: 1.0, y: 1.0 }));
    assert_eq!(Zone::Excluded, bsp_check_point(&bsp, &Vector2d{ x: 0.0, y: -1.0 }));
    assert_eq!(Zone::Excluded, bsp_check_point(&bsp, &Vector2d{ x: -1.0, y: 0.0 }));
    assert_eq!(Zone::Excluded, bsp_check_point(&bsp, &Vector2d{ x: 1.0, y: 0.0 }));
    assert_eq!(Zone::Excluded, bsp_check_point(&bsp, &Vector2d{ x: 0.0, y: 1.0 }));
    assert_eq!(Zone::Excluded, bsp_check_point(&bsp, &Vector2d { x:0.0, y: 0.0 }));
}


fn parse_input(input: &str) -> Vec<Sensor> {
    let mut sensors : Vec<Sensor> = vec![];
    for line in input.lines() {
        let mut sensor_0 = 0i64;
        let mut sensor_1 = 0i64;
        let mut beacon_0 = 0i64;
        let mut beacon_1 = 0i64;
        sscanf!(line, "Sensor at x={}, y={}: closest beacon is at x={}, y={}",sensor_0,sensor_1,beacon_0,beacon_1);
        sensors.push((Vector2d{x:sensor_0 as f64,y:sensor_1 as f64},Vector2d{x:beacon_0 as f64,y:beacon_1 as f64}));
    }

    sensors
}

fn get_my_input() -> &'static str {
"Sensor at x=-1, y=-1: closest beacon is at x=3, y=-1
Sensor at x=3, y=2: closest beacon is at x=6, y=2
Sensor at x=3, y=-2: closest beacon is at x=8, y=-2
Sensor at x=-1, y=6: closest beacon is at x=6, y=6"
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