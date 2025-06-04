use std::{fmt::Debug, fs, str::FromStr, collections::{BinaryHeap, HashSet, VecDeque}};

fn main() {
    println!("--- Day 12: Hill Climbing Algorithm ---");
    let grid = fs::read_to_string("src/input.txt").unwrap();
    let grid =  grid.parse::<Grid>().unwrap();

    let mut visited_coords = HashSet::new();
    let mut unvisited_coords = VecDeque::new();
    // unvisited_coords.push_front(DistanceCoord{
    //     coord: grid.start_coord, 
    //     step: 0});

    unvisited_coords.push_front(DistanceCoord{
        coord: grid.end_coord, 
        step: 0});
    let steps_to_top = loop {
        let DistanceCoord{coord: visiting_coord, step} = unvisited_coords.pop_back().unwrap();
        // if visiting_coord == grid.end_coord {
        //     break step;
        // }

        if grid.get_grid_square(visiting_coord).unwrap().0 == 0 {
            break step;
        }
        if visited_coords.get(&visiting_coord).is_some() {
            continue;
        }

        let climbable_coords = grid.get_climbable_coordinates_from_coordinate(visiting_coord);
        for &neighbor_coord in climbable_coords.iter() {
            if visited_coords.get(&neighbor_coord).is_none() {
                unvisited_coords.push_front(DistanceCoord { coord: neighbor_coord, step: step + 1 })
            } 
        }

        visited_coords.insert(visiting_coord);
        // println!("{:?}", visited_coords);
    };
    println!("Movement count {steps_to_top}");
}


#[derive(PartialEq, Eq, PartialOrd)]
struct DistanceCoord {
    coord: Coordinate,
    step: usize
}

impl Ord for DistanceCoord {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.step.cmp(&other.step)
    }
}

struct Grid {
    grid: Vec<GridSquare>,
    width: usize,
    height: usize,
    start_coord: Coordinate,
    end_coord: Coordinate,
}
impl FromStr for Grid {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().unwrap().len();
        let height = s.lines().count();

        let mut start_coord = None;
        let mut end_coord = None;

        let mut grid = Vec::with_capacity(width * height);
        for (coord_y, line) in s.lines().enumerate() {
            for (coord_x, c) in line.chars().enumerate() {
                let grid_sq = match c { 
                    'a'..='z' => GridSquare(c as u8 - 'a' as u8),
                    'S' => {            start_coord = Some(Coordinate {
                            x: coord_x,
                            y: coord_y,
                        });
                        GridSquare('a' as u8 - 'a' as u8)
                    }
                    'E' => {
                        end_coord = Some(Coordinate {
                            x: coord_x,
                            y: coord_y,
                        });
                        GridSquare('z' as u8 - 'a' as u8)
                    }
                    _ => unreachable!(),
                };
                grid.push(grid_sq);
            }
        }

        Ok(Grid {
            grid,
            width,
            height,
            start_coord: start_coord.unwrap(),
            end_coord: end_coord.unwrap(),
        })
    }
}

impl Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let grid_sq =
                    ('a' as u8 + self.get_grid_square(Coordinate { x, y }).unwrap().0) as char;
                write!(f, "{grid_sq} ")?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl Grid {
    fn get_grid_square(&self, coord: Coordinate) -> Option<GridSquare> {
        self.grid.get(coord.y * self.width + coord.x).copied()
    }

    fn get_climbable_coordinates_from_coordinate(&self, coord: Coordinate) -> Vec<Coordinate> {
        let coord_grid_square = self.get_grid_square(coord).unwrap();
        let coord_offsets = [(0,1), (0,-1), (-1, 0), (1, 0)];
        coord_offsets
            .iter()
            .filter_map(|&cood_offset| {
                let coord_x = coord.x as i32 + cood_offset.0;
                let coord_y = coord.y as i32 + cood_offset.1;
                if coord_x < 0 || coord_y < 0 { return None; }
                let (coord_x, coord_y) = (coord_x as usize, coord_y as usize);
                if coord_x >= self.width || coord_y >= self.height { return None; }
               
                let adjacent_coord = Coordinate{x: coord_x, y: coord_y};
                let adjacent_grid_square = self.get_grid_square(adjacent_coord).unwrap();
                if adjacent_grid_square.can_climb_to(coord_grid_square) {
                    Some(adjacent_coord)
                } else {
                    None
                }
            })
            .collect()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, Debug)]
struct GridSquare(u8);

impl GridSquare {
    fn can_climb_to(&self, other: GridSquare) -> bool {
        other.0 <= (self.0 + 1)
    }
}
