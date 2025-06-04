use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let sequences: Vec<Vec<i32>> = input.lines().map(|line| {
        line.split_whitespace().map(|element| element.parse().unwrap()).collect()
    }).collect();

    // dbg!(&sequences);

    let seq_sum: i32 = sequences.iter().map(|sequence| {

        let mut sequence_clone = sequence.clone();
        let mut sequence_size = sequence.len()- 1;
        
        while sequence_clone[0..sequence_size].iter().any(|n| *n != 0) {
            for i in 0..sequence_size  {
                sequence_clone[i] = sequence_clone[i+1] - sequence_clone[i];
            }
            
            sequence_size -= 1;
        }
        
        sequence_clone.iter().sum::<i32>()
    }).sum();

    dbg!(&seq_sum);


    let seq_pred_sub: i32 = sequences.iter().map(|sequence| {

        let mut sequence_clone = sequence.clone();
        let sequence_size = sequence.len();
        let mut seq_start = 1;
        
        while sequence_clone[seq_start..sequence_size].iter().any(|n| *n != 0) {
            for i in (seq_start..sequence_size).rev()  {
                sequence_clone[i] = sequence_clone[i] - sequence_clone[i - 1];
            }
            
            seq_start += 1;
        }
        
        sequence_clone.iter().rev().fold(0, |acc, x| {x - acc})
    }).sum();
    
    dbg!(&seq_pred_sub);
}
