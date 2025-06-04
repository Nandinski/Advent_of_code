use serde_json::{self, json};
use std::{cmp::Ordering, fs};

fn main() {
    println!("--- Day 13: Distress Signal ---");

    let input = fs::read_to_string("src/input.txt").unwrap();

    let mut packets: Vec<serde_json::Value> = input
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|l| serde_json::from_str(l).unwrap())
        .collect();
    let divider_packets = [json!([[2]]), json!([[6]])];
    packets.extend(divider_packets.clone());

    packets.sort_by(cmp_left_and_right_val);
    let divider_packet_start = packets
        .binary_search_by(|probe| cmp_left_and_right_val(probe, &divider_packets[0]))
        .unwrap();
    let divider_packet_end = packets
        .binary_search_by(|probe| cmp_left_and_right_val(probe, &divider_packets[1]))
        .unwrap();

    let decoder_key = (divider_packet_start + 1) * (divider_packet_end + 1);
    // println!("Hello {:#?}", packets);
    dbg!(decoder_key);
}

fn find_packet_pairs_in_correct_order(input: String) {
    let packet_pairs = input.split("\n\n");
    let packets_in_correct_order: usize = packet_pairs
        .into_iter()
        .enumerate()
        .map(|(i, packet_pair)| {
            let (left, right) = packet_pair.split_once("\n").unwrap();
            let (left_packet, right_packet) = (
                serde_json::from_str(left).unwrap(),
                serde_json::from_str(right).unwrap(),
            );
            if cmp_left_and_right_val(&left_packet, &right_packet).is_le() {
                i + 1
            } else {
                0
            }
        })
        // .inspect(|i| {dbg!(i);})
        .sum();

    println!(
        "Packet pairs in correct order = {}",
        packets_in_correct_order
    );
}

fn cmp_left_and_right_val(left_val: &serde_json::Value, right_val: &serde_json::Value) -> Ordering {
    match left_val {
        serde_json::Value::Number(left_n) => {
            if let serde_json::Value::Number(right_n) = right_val {
                cmp_numbers(left_n, right_n)
            } else if let serde_json::Value::Array(_) = right_val {
                // Make array to wrap number
                let left_packet = wrap_number_in_array(left_n);
                cmp_left_and_right_val(&left_packet, right_val)
            } else {
                unreachable!()
            }
        }
        serde_json::Value::Array(left_arr) => {
            if let serde_json::Value::Array(right_arr) = right_val {
                compare_vectors(left_arr, right_arr)
            } else if let serde_json::Value::Number(right_n) = right_val {
                // Make array to wrap number
                let right_packet = wrap_number_in_array(right_n);
                cmp_left_and_right_val(left_val, &right_packet)
            } else {
                unreachable!()
            }
        }
        _ => unreachable!(),
    }
}

fn cmp_numbers(left_n: &serde_json::Number, right_n: &serde_json::Number) -> Ordering {
    left_n.as_i64().cmp(&right_n.as_i64())
}

fn wrap_number_in_array(n: &serde_json::Number) -> serde_json::Value {
    serde_json::json!([n])
}

fn compare_vectors(
    left_arr: &Vec<serde_json::Value>,
    right_arr: &Vec<serde_json::Value>,
) -> Ordering {
    let left_arr_iter = left_arr.iter();
    let right_arr_iter = right_arr.iter();

    let mut ord = Ordering::Equal;
    for (left_val, right_val) in left_arr_iter.zip(right_arr_iter) {
        // println!("Comparing {} with {}", left_val, right_val);
        ord = cmp_left_and_right_val(left_val, right_val);
        if !ord.is_eq() {
            break;
        }
    }

    if ord.is_eq() {
        ord = left_arr.len().cmp(&right_arr.len());
    }

    ord
}
