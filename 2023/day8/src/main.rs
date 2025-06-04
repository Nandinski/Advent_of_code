use std::{fs, collections::HashMap};

use regex::Regex;


fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    
    let (instructions_input, map_input) = input.split_once("\n\n").unwrap();
    let instructions: Vec<_> = instructions_input.chars().map(|c| {
        match c {
            'L' => 0,
            'R' => 1,
            _ => panic!("Intruction {c} not recognized.")
        }
    }).collect();
    
    let re: Regex = Regex::new(r"(?P<map_key>\w{3}) = \((?P<left>\w{3}), (?P<right>\w{3})\)").unwrap();
    let map: HashMap<String, [String; 2]> = map_input.lines().map(|line| {
        let captures = re.captures(line).unwrap();
        (captures["map_key"].to_string(), [captures["left"].to_string(), captures["right"].to_string()])
    }).collect();

    // let mut instructions_iter = instructions.iter().cycle();
    // let mut next_node = &"AAA".to_string();
    // let mut step_count = 0;
    // while next_node != "ZZZ" {
    //     let next_instruction_index = *instructions_iter.next().unwrap();
    //     next_node = &map.get(next_node).unwrap()[next_instruction_index];
    //     step_count += 1;
    // }

    // let starting_nodes: Vec<_> = map.keys().filter(|node| node.ends_with('A')).collect();
    // let ending_nodes: Vec<_> = map.keys().filter(|node| node.ends_with('Z')).collect();
    // let mut past_record = vec![HashMap::<(&String, usize), (i32, i32)>::new(); starting_nodes.len()]; 
    
    // dbg!(starting_nodes.len());
    // dbg!(ending_nodes.len());

    // let mut instructions_iter = instructions.iter().enumerate().cycle();
    // let mut next_nodes = starting_nodes;
    // let mut step_count = 0;
    // let mut repeating = vec![false; next_nodes.len()];
    
    // while !(next_nodes.iter().all(|node| {node.ends_with('Z')})) {
    //     let (instruction_i, next_instruction_index) = instructions_iter.next().unwrap();

    //     next_nodes.iter_mut().for_each(|node| {
    //         *node = &map.get(*node).unwrap()[*next_instruction_index];
    //     }); 

    //     step_count += 1;

    //     // Record state
    //     next_nodes.iter().enumerate().for_each(|(ghost_i, node)| {
    //         if repeating[ghost_i] {
    //             println!("Ghost_i = {ghost_i} repeating!");
    //             return
    //         }

    //         let ghost_i_hashmap = past_record.get_mut(ghost_i).unwrap();
    //         if let Some((_, second_occurance)) = ghost_i_hashmap.get_mut(&(*node, instruction_i)) {
    //             if *second_occurance == 0 { 
    //                 *second_occurance = step_count;
    //             } else {
    //                 // Loop closed!
    //                 repeating[ghost_i] = true;
    //                 if repeating.iter().all(|b| {*b}) {
    //                     println!("This will loop forever!");
    //                     exit(0);
    //                 }
    //             }
    //         } else {
    //             ghost_i_hashmap.insert((*node, instruction_i), (step_count, 0));
    //         }
    //     }); 
    // }

    let mut instructions_iter = instructions.iter().cycle();
    let starting_nodes: Vec<_> = map.keys().filter(|node| node.ends_with('A')).collect();

    let min_steps = starting_nodes.iter().map(|start_node| {
        let mut step_count: i64 = 0;
        let mut next_node = *start_node;
        while !next_node.ends_with('Z') {
            let next_instruction_index = *instructions_iter.next().unwrap();
            next_node = &map.get(next_node).unwrap()[next_instruction_index];
            step_count += 1;
        }
        step_count
    }).fold(1, num_integer::lcm);


    dbg!(&min_steps);
}
