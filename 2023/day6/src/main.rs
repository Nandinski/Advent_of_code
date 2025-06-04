use std::fs;
use std::iter::zip;

fn main() {
    let input = fs::read_to_string("input_2.txt").unwrap();

    let mut lines = input.lines();
    let times: Vec<f64> = lines.next().unwrap().split_whitespace().skip(1).map(|t| t.parse().unwrap()).collect();
    let records: Vec<f64> = lines.next().unwrap().split_whitespace().skip(1).map(|r| r.parse().unwrap()).collect();
    let races: Vec<(f64, f64)> = zip(times, records).collect();

    let possibilities: Vec<_> = races.iter().map(|(time_limit, record_time)| {
        let (mut min_hold, mut max_hold) = compute_roots(-1.0, time_limit, -record_time);
        if min_hold.fract() == 0.0 {
            min_hold += 1.0;
        }
        if max_hold.fract() == 0.0 {
            max_hold -= 1.0;
        }

        min_hold = min_hold.ceil();
        max_hold = max_hold.floor();

        if min_hold >= max_hold {
            panic!("min was bigger or equal to max");
        }

        max_hold - min_hold + 1.0
    }).collect();

    dbg!(&possibilities);
    let possibilities_multipled: f64 = possibilities.iter().product();
    dbg!(&possibilities_multipled);

}

fn compute_roots(a:f64, b: &f64, c: f64) -> (f64, f64) {
    let sqrt_val = (b*b - 4.0*a*c).sqrt();
    if sqrt_val.is_nan() {
        panic!("sqrt value was negative! sqrt = {sqrt_val}")
    }

    let left_root = (-b + sqrt_val) / 2.0*a;
    let right_root = (-b - sqrt_val) / 2.0*a;

    (left_root, right_root)
}
