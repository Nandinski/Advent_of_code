use std::fs;

use derive_more::{Add, Into, Sub};
use sscanf::sscanf;

fn main() {
    println!("--- Day 15: Beacon Exclusion Zone ---");

    let sensors_info = fs::read_to_string("src/input.txt").unwrap();
    // let target_line_height = 10; // TODO don't forget to change the line height for the test

    let sensors = read_sensors(&sensors_info);
    let mut beacons: Vec<Beacon> = sensors.iter().map(|s| s.scanned_beacon).collect();
    beacons.sort();
    beacons.dedup();

    // part 1
    // let target_line_height = 2000000;
    // let excluded_pos = get_excluded_pos_count(&sensors, &beacons, target_line_height);

    // part 2
    let scan_min: u64 = 0;
    let scan_max = 4_000_000;
    // let scan_max = 20;
    for line_height in scan_min..scan_max {
        let x_pos = find_missing_x_in_line(&sensors, scan_min, scan_max, line_height);
        if let Some(x_pos) = x_pos {
            println!(
                "We found the distress position! x_pos = {}, line_height = {}",
                x_pos, line_height
            );
            println!("Tuning frequency = {}", x_pos * 4_000_000 + line_height);
        }
    }
}

fn find_missing_x_in_line(
    sensors: &[Sensor],
    scan_min: i32,
    scan_max: i32,
    line_height: i32,
) -> Option<i32> {
    let mut ranges = get_scanned_ranges_in_line(sensors, line_height);
    ranges.sort();

    let mut x_pos = None;
    let mut current_min = i32::MIN;
    let mut current_max = i32::MIN;
    for range in ranges {
        // range already considered
        if range.min >= current_min && range.max <= current_max {
            continue;
        } else if range.min <= current_max && range.max > current_max {
            // extends prev minimum
            current_max = range.max;
        } else {
            // new disconnected ranged
            if range.min - 2 == current_max {
                let candidate_x = range.min - 1;
                if candidate_x >= scan_min && candidate_x <= scan_max {
                    // we already found another isolated point on this line
                    if x_pos.is_some() {
                        return None;
                    } else {
                        x_pos = Some(candidate_x)
                    }
                }
            }

            current_min = range.min;
            current_max = range.max;
        }
    }

    x_pos
}

fn get_excluded_pos_count(
    sensors: &[Sensor],
    beacons: &[Beacon],
    target_line_height: i32,
) -> usize {
    let mut ranges = get_scanned_ranges_in_line(sensors, target_line_height);
    ranges.sort();

    let excluded_pos = get_excluded_beacon_pos_from_sorted_ranges(&ranges);
    let beacons_in_line =
        get_beacons_in_line_from_sorted_ranges(&beacons, &ranges, target_line_height);
    let excluded_pos_wo_beacons = excluded_pos - beacons_in_line;
    excluded_pos_wo_beacons
}

fn get_beacons_in_line_from_sorted_ranges(
    beacons: &[Beacon],
    ranges: &[Range],
    target_line_height: i32,
) -> usize {
    beacons
        .iter()
        .filter(|&beacon| beacon.pos.y == target_line_height && is_beacon_in_ranges(beacon, ranges))
        .count()
}

fn is_beacon_in_ranges(beacon: &Beacon, ranges: &[Range]) -> bool {
    ranges
        .iter()
        .any(|range| beacon.pos.x >= range.min && beacon.pos.x <= range.max)
}

fn get_excluded_beacon_pos_from_sorted_ranges(ranges: &Vec<Range>) -> usize {
    let mut current_min = i32::MIN;
    let mut current_max = i32::MIN;
    let mut excluded_pos_count = 0;
    for range in ranges {
        // range alsready considered
        let range_excluded_pos_count;
        if range.min >= current_min && range.max <= current_max {
            continue;
        } else if range.min <= current_max && range.max > current_max {
            // extends prev minimum
            range_excluded_pos_count = range.max - current_max;
            current_max = range.max;
        } else {
            // new disconnected ranged
            range_excluded_pos_count = range.max - range.min + 1;
            current_max = range.max;
            current_min = range.min;
        }

        excluded_pos_count += (range_excluded_pos_count) as usize;
    }

    excluded_pos_count
}

fn read_sensors(sensors_info: &str) -> Vec<Sensor> {
    let sensors: Vec<Sensor> = sensors_info
        .lines()
        .map(|sensor_info| {
            let (sensor_x, sensor_y, beacon_x, beacon_y) = sscanf!(
                sensor_info,
                "Sensor at x={i32}, y={i32}: closest beacon is at x={i32}, y={i32}"
            )
            .unwrap();
            let beacon = Beacon {
                pos: Coordinate {
                    x: beacon_x,
                    y: beacon_y,
                },
            };
            let sensor = Sensor {
                pos: Coordinate {
                    x: sensor_x,
                    y: sensor_y,
                },
                scanned_beacon: beacon,
            };
            sensor
        })
        .collect();
    sensors
}

fn get_scanned_ranges_in_line(sensors: &[Sensor], target_line_height: i32) -> Vec<Range> {
    sensors
        .iter()
        .filter_map(|sensor| {
            let max_distance = sensor.pos.manhattan_distance_to(&sensor.scanned_beacon.pos);
            let distance_to_target_line = (target_line_height - sensor.pos.y).abs() as usize;
            let dist_remaining_on_line = max_distance.checked_sub(distance_to_target_line);
            if let Some(dist_remaining_on_line) = dist_remaining_on_line {
                Some(Range {
                    min: sensor.pos.x - dist_remaining_on_line as i32,
                    max: sensor.pos.x + dist_remaining_on_line as i32,
                })
            } else {
                None
            }
        })
        .collect()
}

#[derive(Add, Sub, Into, Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn manhattan_distance_to(&self, other: &Coordinate) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }
}

struct Sensor {
    pos: Coordinate,
    scanned_beacon: Beacon,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Beacon {
    pos: Coordinate,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Range {
    min: i32,
    max: i32,
}
