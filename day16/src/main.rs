use std::collections::HashSet;

fn str_to_grid(grid_str: &str) -> Vec<Vec<char>> {
    grid_str
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>()
}

fn in_bounds(i: isize, j: isize, grid: &Vec<Vec<char>>) -> bool {
    0 <= i && i < grid.len().try_into().unwrap() && 0 <= j && j < grid[0].len().try_into().unwrap()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Beam {
    position: (usize, usize),
    direction: (isize, isize),
}

fn step(beam: &Beam, grid: &Vec<Vec<char>>) -> Vec<Beam> {
    match grid[beam.position.0][beam.position.1] {
        '.' => {
            // Continue in same direction
            let newx = beam.position.0 as isize + beam.direction.0;
            let newy = beam.position.1 as isize + beam.direction.1;
            if in_bounds(newx, newy, grid) {
                vec![Beam {
                    position: (newx as usize, newy as usize),
                    direction: beam.direction,
                }]
            } else {
                vec![]
            }
        }
        '/' => {
            // Reflect based on direction
            return match beam.direction {
                (0, 1) => {
                    // right input, upwards output
                    if in_bounds(beam.position.0 as isize - 1, beam.position.1 as isize, grid) {
                        vec![Beam {
                            position: (beam.position.0 - 1, beam.position.1),
                            direction: (-1, 0),
                        }]
                    } else {
                        vec![]
                    }
                }
                (1, 0) => {
                    // down input, left output
                    if in_bounds(beam.position.0 as isize, beam.position.1 as isize - 1, grid) {
                        vec![Beam {
                            position: (beam.position.0, beam.position.1 - 1),
                            direction: (0, -1),
                        }]
                    } else {
                        vec![]
                    }
                }
                (-1, 0) => {
                    // up input, right output
                    if in_bounds(beam.position.0 as isize, beam.position.1 as isize + 1, grid) {
                        vec![Beam {
                            position: (beam.position.0, beam.position.1 + 1),
                            direction: (0, 1),
                        }]
                    } else {
                        vec![]
                    }
                }
                (0, -1) => {
                    // left input, down output
                    if in_bounds(beam.position.0 as isize + 1, beam.position.1 as isize, grid) {
                        vec![Beam {
                            position: (beam.position.0 + 1, beam.position.1),
                            direction: (1, 0),
                        }]
                    } else {
                        vec![]
                    }
                }
                _ => panic!("Unexpected direction!"),
            };
        }
        '\\' => {
            // Reflect based on direction
            return match beam.direction {
                (0, 1) => {
                    // right input, down output
                    if in_bounds(beam.position.0 as isize + 1, beam.position.1 as isize, grid) {
                        vec![Beam {
                            position: (beam.position.0 + 1, beam.position.1),
                            direction: (1, 0),
                        }]
                    } else {
                        vec![]
                    }
                }
                (1, 0) => {
                    // down input, right output
                    if in_bounds(beam.position.0 as isize, beam.position.1 as isize + 1, grid) {
                        vec![Beam {
                            position: (beam.position.0 + 0, beam.position.1 + 1),
                            direction: (0, 1),
                        }]
                    } else {
                        vec![]
                    }
                }
                (-1, 0) => {
                    // up input, left output
                    if in_bounds(beam.position.0 as isize, beam.position.1 as isize - 1, grid) {
                        vec![Beam {
                            position: (beam.position.0, beam.position.1 - 1),
                            direction: (0, -1),
                        }]
                    } else {
                        vec![]
                    }
                }
                (0, -1) => {
                    // left input, up output
                    if in_bounds(beam.position.0 as isize - 1, beam.position.1 as isize, grid) {
                        vec![Beam {
                            position: (beam.position.0 - 1, beam.position.1),
                            direction: (-1, 0),
                        }]
                    } else {
                        vec![]
                    }
                }
                _ => panic!("Unexpected direction!"),
            };
        }
        '|' => {
            return match beam.direction {
                (1, 0) | (-1, 0) => {
                    // up or down input, continue in same direction
                    let newx = beam.position.0 as isize + beam.direction.0;
                    let newy = beam.position.1 as isize + beam.direction.1;
                    if in_bounds(newx, newy, grid) {
                        vec![Beam {
                            position: (newx as usize, newy as usize),
                            direction: beam.direction,
                        }]
                    } else {
                        vec![]
                    }
                }
                (0, 1) | (0, -1) => {
                    // left or right input, up and down output
                    let mut output = Vec::<Beam>::new();
                    if in_bounds(beam.position.0 as isize + 1, beam.position.1 as isize, grid) {
                        output.push(Beam {
                            position: (beam.position.0 + 1, beam.position.1),
                            direction: (1, 0),
                        })
                    }
                    if in_bounds(beam.position.0 as isize - 1, beam.position.1 as isize, grid) {
                        output.push(Beam {
                            position: (beam.position.0 - 1, beam.position.1),
                            direction: (-1, 0),
                        })
                    }
                    output
                }
                _ => panic!("Unexpected direction!"),
            };
        }
        '-' => {
            // Split if from top or bottom, otherwise continue
            return match beam.direction {
                (0, 1) | (0, -1) => {
                    // left or right input, continue in same direction
                    let newx = beam.position.0 as isize + beam.direction.0;
                    let newy = beam.position.1 as isize + beam.direction.1;
                    if in_bounds(newx, newy, grid) {
                        vec![Beam {
                            position: (newx as usize, newy as usize),
                            direction: beam.direction,
                        }]
                    } else {
                        vec![]
                    }
                }
                (1, 0) | (-1, 0) => {
                    // up or down input, left and right output
                    let mut output = Vec::<Beam>::new();
                    if in_bounds(beam.position.0 as isize, beam.position.1 as isize + 1, grid) {
                        output.push(Beam {
                            position: (beam.position.0, beam.position.1 + 1),
                            direction: (0, 1),
                        })
                    }
                    if in_bounds(beam.position.0 as isize, beam.position.1 as isize - 1, grid) {
                        output.push(Beam {
                            position: (beam.position.0, beam.position.1 - 1),
                            direction: (0, -1),
                        })
                    }
                    output
                }
                _ => panic!("Unexpected direction!"),
            };
        }
        _ => panic!("whoa unexpected grid value!"),
    }
}

fn main() {
    // Part 1
    let grid = str_to_grid(include_str!("day16.txt"));

    let mut visited = HashSet::<Beam>::new();
    visited.insert(Beam {
        position: (0, 0),
        direction: (0, 1),
    });

    // Initial 'live' beam starts at 0,0 and heading right
    let mut beams = HashSet::<Beam>::new();
    beams.insert(Beam {
        position: (0, 0),
        direction: (0, 1),
    });

    // Loop until we don't get a change in visited beam map
    let mut prev_n_visited = 0;
    while prev_n_visited != visited.len() {
        prev_n_visited = visited.len();
        beams = beams
            .iter()
            .map(|beam| {
                let next_beams = step(beam, &grid);
                for next_beam in &next_beams {
                    visited.insert(next_beam.clone());
                }
                next_beams
            })
            .flat_map(|b| b)
            .collect::<HashSet<Beam>>();
    }

    // todo performance improvement; prune 'beams' as we go if they are already in visited

    // Show beams!
    let positions: HashSet<(usize, usize)> = visited.iter().map(|b| b.position).collect();
    for (y, row) in grid.iter().enumerate() {
        for (x, &_ch) in row.iter().enumerate() {
            let print_char = if positions.contains(&(x, y)) {
                '#'
            } else {
                '.'
            };
            print!("{}", print_char);
        }
        println!();
    }

    println!("{}", positions.len());
}
