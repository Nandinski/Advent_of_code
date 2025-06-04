use std::{fs, vec};

fn main() {
    let input = fs::read_to_string("./example_3.txt").unwrap();

    let tiles: Vec<Vec<char>> = input.lines().map(|line| {
        line.chars().collect()
    }).collect();

    let mut starting_pos  = (0, 0);
    'outer: for (y, row) in tiles.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'S' {
                starting_pos = (y as i32, x as i32);
                break 'outer;
            }
        }
    }

    dbg!(&starting_pos);

    let mut starting_directions: Vec<Direction> = vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right].into_iter().filter(|dir| {
        let offset = dir_to_offset(dir);
        let (next_y, next_x) = (starting_pos.0 + offset.0, starting_pos.1 + offset.1);
        if next_x < 0 || next_y < 0 {
            return false
        }
        let pipe_type = tiles[next_y as usize][next_x as usize];
        next_dir(pipe_type, dir).is_some()
    }).collect();

    dbg!(&starting_directions);

    let starting_direction = starting_directions.pop().unwrap();

    let mut input_direction = starting_direction;
    let mut current_pos = starting_pos;
    let mut next_pipe = '.';
    let mut pipe_loop = vec![];

    while next_pipe != 'S' {
        let offset = dir_to_offset(&input_direction);
        let (next_y, next_x) = (current_pos.0 + offset.0, current_pos.1 + offset.1);
        next_pipe = tiles[next_y as usize][next_x as usize];
        input_direction = next_dir(next_pipe, &input_direction).unwrap();

        current_pos = (next_y, next_x);
        pipe_loop.push((current_pos.clone(), input_direction));
    }

    // dbg!(&pipe_loop);

    // let furthest_point_distance = pipe_loop.len()/2;
    // dbg!(&furthest_point_distance);

    let mut tiles_clone = vec![vec!['.'; tiles[0].len()]; tiles.len()];
    for (pos, _) in pipe_loop.iter() {
        tiles_clone[pos.0 as usize][pos.1 as usize] = tiles[pos.0 as usize][pos.1 as usize]
    }

    // dbg!(&tiles_clone);
    // let mut interior_points = vec![];
    // for (pos, direction) in pipe_loop.iter() {
    //     let rotated_dir = rotate_90(&direction);
    //     let offset = dir_to_offset(&rotated_dir);
    //     let (affected_y, affected_x) = (pos.0 + offset.0, pos.1 + offset.1);

    //     // if tiles_clone[affected_y as usize][affected_x as usize] != 

    //     if affected_x < 0 || affected_y < 0 || affected_y >= tiles_clone.len() as i32 || affected_x >= tiles_clone[0].len() as i32 {
    //         continue;
    //     }

    //     // Check if affected is part of the pipe loop
    //     if pipe_loop.iter().find(|(pos, _)| {
    //         pos.0 == affected_y && pos.1 == affected_x
    //     }).is_none() {
    //         tiles_clone[affected_y as usize][affected_x as usize] = 'I';
    //         interior_points.push((affected_y, affected_x))
    //     }
    // }
    // // dbg!(&tiles_clone.iter().flatten().filter(|&c| *c == 'I').count());


    // while let Some(pos) = interior_points.pop() {
    //     let neighbors = get_neighbors(&pos);
    //     for neighbor in neighbors {
            
    //         if let Some(row) = tiles_clone.get_mut(neighbor.0 as usize) {
    //             if let Some(c) = row.get_mut(neighbor.1 as usize) {
    //                 if *c == '.' {
    //                     *c = 'I';
    //                     interior_points.push(neighbor);
    //                 }
    //             }
    //         }
    //     }
    // }
    // let count = tiles_clone.iter().flatten().filter(|&c| *c == 'I').count();
    // dbg!(&count);
    
    // let total = tiles[0].len() * tiles.len();
    // let others = total - count - pipe_loop.len();
    // dbg!(&others);

    // for row in tiles_clone.iter() {
    //     for c in row.iter() {
    //         print!("{c}");
    //     }
    //     print!("\n")
    // }

}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn next_dir(pipe_type: char, in_movement: &Direction) -> Option<Direction> {
    if pipe_type == '-' {
        match in_movement {
            Direction::Left => Some(Direction::Left),
            Direction::Right => Some(Direction::Right),
            _ => None
        }
    } else if pipe_type == '|' {
        match in_movement {
            Direction::Up => Some(Direction::Up),
            Direction::Down => Some(Direction::Down),
            _ => None
        }
    } else if pipe_type == 'L' {
        match in_movement {
            Direction::Down => Some(Direction::Right),
            Direction::Left => Some(Direction::Up),
            _ => None
        }
    } else if pipe_type == 'J' {
        match in_movement {
            Direction::Down => Some(Direction::Left),
            Direction::Right => Some(Direction::Up),
            _ => None
        }
    } else if pipe_type == '7' {
        match in_movement {
            Direction::Up => Some(Direction::Left),
            Direction::Right => Some(Direction::Down),
            _ => None
        }
    } else if pipe_type == 'F' {
        match in_movement {
            Direction::Up => Some(Direction::Right),
            Direction::Left => Some(Direction::Down),
            _ => None
        }
    } else if pipe_type == 'S' {
        Some(Direction::Up)
    } else {
        None
    }
}

fn dir_to_offset(dir: &Direction) -> (i32, i32) {
    match dir {
        Direction::Up => (-1, 0),
        Direction::Down => (1, 0),
        Direction::Left => (0, -1),
        Direction::Right => (0, 1),
    }
}

fn rotate_90(dir: &Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}

// fn rotate_90(dir: &Direction) -> Direction {
//     match dir {
//         Direction::Up => Direction::Left,
//         Direction::Right => Direction::Up,
//         Direction::Down => Direction::Right,
//         Direction::Left => Direction::Down,
//     }
// }

fn get_neighbors(pos: &(i32, i32)) -> Vec<(i32, i32)> {
    [Direction::Up, Direction::Down, Direction::Left, Direction::Right].into_iter().map(|dir| {
        let offset = dir_to_offset(&dir);
        (pos.0 + offset.0, pos.1 + offset.1)
    }).collect()
}
