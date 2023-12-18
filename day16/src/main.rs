use std::{cmp::max, collections::HashSet};

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

fn energise_beam(mut beams: HashSet<Beam>, visited: &mut HashSet<Beam>, grid: &Vec<Vec<char>>) {
    // Loop until we don't get a change in visited beam map
    let mut prev_n_visited = 0;
    while prev_n_visited != visited.len() {
        prev_n_visited = visited.len();
        beams = beams
            .iter()
            .map(|beam| step(beam, grid))
            .flat_map(|b| b)
            .filter(|beam| !&visited.contains(beam))
            .collect::<HashSet<Beam>>();
        for beam in &beams {
            visited.insert(beam.clone());
        }
    }
}

fn energise_with_start_beam(start_beam: Beam, grid: &Vec<Vec<char>>) -> HashSet<Beam> {
    let mut visited = HashSet::<Beam>::new();
    visited.insert(start_beam.clone());

    // Initial 'live' beam starts at 0,0 and heading right
    let mut beams = HashSet::<Beam>::new();
    beams.insert(start_beam.clone());

    energise_beam(beams, &mut visited, grid);
    visited
}

fn main() {
    // Part 1
    let grid = str_to_grid(include_str!("day16.txt"));

    let start_beam = Beam {
        position: (0, 0),
        direction: (0, 1),
    };
    let visited = energise_with_start_beam(start_beam, &grid);

    // Find unique positions
    let positions: HashSet<(usize, usize)> = visited.iter().map(|b| b.position).collect();
    println!("{}", positions.len());

    // Part 2, try each start position
    let mut max_energised_positions = 0;
    // Top grid
    for i in 0..grid[0].len() {
        let start_beam = Beam {
            position: (0, i),
            direction: (1, 0),
        };
        max_energised_positions = max(
            max_energised_positions,
            energise_with_start_beam(start_beam, &grid)
                .iter()
                .map(|b| b.position)
                .collect::<HashSet<(usize, usize)>>()
                .len(),
        )
    }
    // Bottom grid
    for i in 0..grid[0].len() {
        let start_beam = Beam {
            position: (grid.len() - 1, i),
            direction: (-1, 0),
        };
        max_energised_positions = max(
            max_energised_positions,
            energise_with_start_beam(start_beam, &grid)
                .iter()
                .map(|b| b.position)
                .collect::<HashSet<(usize, usize)>>()
                .len(),
        )
    }
    // Left grid
    for i in 0..grid.len() {
        let start_beam = Beam {
            position: (0, i),
            direction: (0, 1),
        };
        max_energised_positions = max(
            max_energised_positions,
            energise_with_start_beam(start_beam, &grid)
                .iter()
                .map(|b| b.position)
                .collect::<HashSet<(usize, usize)>>()
                .len(),
        )
    }
    // Right grid
    for i in 0..grid.len() {
        let start_beam = Beam {
            position: (grid[0].len() - 1, i),
            direction: (0, -1),
        };
        max_energised_positions = max(
            max_energised_positions,
            energise_with_start_beam(start_beam, &grid)
                .iter()
                .map(|b| b.position)
                .collect::<HashSet<(usize, usize)>>()
                .len(),
        )
    }

    println!("{}", max_energised_positions);
}
