use std::ops::Div;
use tailcall::tailcall;

const DOWN: (isize, isize) = (1, 0);
const LEFT: (isize, isize) = (0, -1);
const UP: (isize, isize) = (-1, 0);
const RIGHT: (isize, isize) = (0, 1);

const DIRS: [(isize, isize); 4] = [DOWN, LEFT, UP, RIGHT];

type Pos = (isize, isize);

fn find_start(grid: &Vec<Vec<char>>) -> Pos {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'S' {
                return (i.try_into().unwrap(), j.try_into().unwrap());
            }
        }
    }
    panic!("Start character 'S' not found in grid!");
}

fn in_bounds(i: isize, j: isize, grid: &Vec<Vec<char>>) -> bool {
    0 <= i && i < grid.len().try_into().unwrap() && 0 <= j && j < grid[0].len().try_into().unwrap()
}

fn locate_starting_step(pos: (isize, isize), grid: &Vec<Vec<char>>) -> (isize, isize) {
    // Find first of two possible starting routes
    for dir in DIRS {
        if in_bounds(pos.0 + dir.0, pos.1 + dir.1, grid) {
            let look = grid[(pos.0 + dir.0) as usize][(pos.1 + dir.1) as usize];
            let look_pos = (pos.0 + dir.0, pos.1 + dir.1);
            return match dir {
                LEFT => match look {
                    '-' | 'L' | 'F' => look_pos,
                    _ => continue,
                },
                RIGHT => match look {
                    '-' | 'J' | '7' => look_pos,
                    _ => continue,
                },
                UP => match look {
                    '|' | '7' | 'F' => look_pos,
                    _ => continue,
                },
                DOWN => match look {
                    '|' | 'L' | 'J' => look_pos,
                    _ => continue,
                },
                _ => panic!("{:?} is not a valid unit direction!", dir),
            };
        }
    }
    panic!("We've looked in every direction, but we're stuck!")
}

fn next_step(pos: Pos, prev_pos: Pos, dir1: Pos, dir2: Pos) -> Pos {
    if (pos.0 + dir1.0, pos.1 + dir1.1) != prev_pos {
        (pos.0 + dir1.0, pos.1 + dir1.1)
    } else {
        (pos.0 + dir2.0, pos.1 + dir2.1)
    }
}

#[tailcall] // required so we don't hit a stack overflow, force reuse of last stackframe!
fn step(pos: Pos, mut visited: Vec<Pos>, start: Pos, grid: &Vec<Vec<char>>) -> Vec<Pos> {
    // Walk through grid, keeping track of the path, until we get back to the start!
    if pos == start && visited.len() > 0 {
        return visited;
    }

    let current_symbol = grid[pos.0 as usize][pos.1 as usize];
    let prev_pos = match visited.last() {
        Some(pos) => pos.clone(),
        None => start.clone(),
    };

    visited.push(pos);

    let next_pos: Pos = match current_symbol {
        'S' => locate_starting_step(pos, grid),
        '|' => next_step(pos, prev_pos, UP, DOWN),
        '-' => next_step(pos, prev_pos, LEFT, RIGHT),
        'L' => next_step(pos, prev_pos, UP, RIGHT),
        'J' => next_step(pos, prev_pos, UP, LEFT),
        '7' => next_step(pos, prev_pos, LEFT, DOWN),
        'F' => next_step(pos, prev_pos, DOWN, RIGHT),
        _ => panic!("We shouldn't be on a non-pipe symbol!"),
    };

    return step(next_pos, visited, start, grid);
}

fn main() {
    // Part 1
    // Load into grid
    let grid = include_str!("day10.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // Find start position
    let start = find_start(&grid);

    // Walk route!
    let route = step(start, vec![], start, &grid);

    // Find distance to furthest point
    let furthest_dist = route.len().div(2);

    println!("{:?}", furthest_dist);

    // Part 2
    fn cast_ray(
        interesctions: i32,
        pos: Pos,
        boundary: Vec<(isize, isize)>,
        grid: &Vec<Vec<char>>,
    ) -> i32 {
        // Cast ray in some direction, count boundary intersections
        if !in_bounds(pos.0, pos.1, grid) {
            return interesctions;
        }
        // and c2 != "L" and c2 != "7"
        let intersections = match boundary.contains(&pos) {
            true => match grid[pos.0 as usize][pos.1 as usize] {
                'L' | '7' => interesctions, // don't count corners!
                _ => interesctions + 1,
            },
            false => interesctions,
        };
        // Cast ray along diagonal (otherwise we get weird edge cases if we cast a ray along a boundary!)
        cast_ray(intersections, (pos.0 + 1, pos.1 + 1), boundary, grid)
    }

    // Use ray casting technique from graphics software (apparently!) whereby we
    // cast out a ray in some direction, and if it intersects the object boundary (our path!)
    // and *even* number of times, then it's outside the object,
    // otherwise (*odd* intersections) it's inside.
    fn is_inside(pos: Pos, boundary: Vec<(isize, isize)>, grid: &Vec<Vec<char>>) -> bool {
        if boundary.contains(&pos) {
            return false;
        }
        // If we have odd intersections along the diaongal then we are inside
        cast_ray(0, pos, boundary.clone(), grid) % 2 == 1
    }

    // For every point, raycast and count number of points with odd intersections
    let mut inside_area = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            inside_area += match is_inside((i as isize, j as isize), route.clone(), &grid) {
                true => 1,
                false => 0,
            }
        }
    }
    println!("{:?}", inside_area);
}
