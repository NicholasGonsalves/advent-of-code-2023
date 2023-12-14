use std::{cmp::min, iter::zip};

/// standard 2d array transpose https://stackoverflow.com/a/64499219
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

fn validate_vertical_reflection(grid: &Vec<Vec<char>>, boundary: usize) -> bool {
    let steps = min(boundary + 1, grid.len() - boundary - 1);
    for i in 1..steps {
        if grid[boundary - i] != grid[boundary + i + 1] {
            return false;
        }
    }
    true
}

fn validate_vertical_reflection_smudge(grid: &Vec<Vec<char>>, boundary: usize) -> bool {
    let mut smudge_count = 0;
    let steps = min(boundary + 1, grid.len() - boundary - 1);
    for i in 1..steps {
        if grid[boundary - i] == grid[boundary + i + 1] {
            continue;
        }
        if off_by_smudge(&grid[boundary - i], &grid[boundary + i + 1]) {
            if smudge_count < 1 {
                smudge_count += 1
            } else {
                return false;
            }
        }
    }
    smudge_count == 1
}

fn vertical_scan(grid: &Vec<Vec<char>>, smudge: bool) -> usize {
    // Find all possible reflection start points (two identical continuous rows)
    if smudge {
        // Try smudged start points
        for start_point in find_start_points(grid, true) {
            if validate_vertical_reflection(grid, start_point) {
                return start_point;
            }
        }
        // Try normal start points, but smudged reflection checks
        for start_point in find_start_points(grid, false) {
            if validate_vertical_reflection_smudge(grid, start_point) {
                return start_point;
            }
        }
    } else {
        // Exact matches only
        for start_point in find_start_points(grid, false) {
            if validate_vertical_reflection(grid, start_point) {
                return start_point;
            }
        }
    }
    return usize::MAX;
}

fn off_by_smudge(row1: &Vec<char>, row2: &Vec<char>) -> bool {
    zip(row1, row2)
        .into_iter()
        .map(|(a, b)| if a != b { 1 } else { 0 })
        .sum::<u32>()
        == 1
}

fn find_start_points(grid: &Vec<Vec<char>>, smudge: bool) -> Vec<usize> {
    let start_points: Vec<usize> = grid[1..]
        .iter()
        .enumerate()
        .map(|(i, row)| match smudge {
            true => {
                if off_by_smudge(row, &grid[i]) {
                    i
                } else {
                    usize::MAX
                }
            }
            false => {
                if row == &grid[i] {
                    i
                } else {
                    usize::MAX
                }
            }
        })
        .filter(|i| i < &usize::MAX)
        .collect();
    start_points
}

fn reflection_value(grid: &Vec<Vec<char>>, smudge: bool) -> u32 {
    // Vertical Pass
    let vertical = vertical_scan(grid, smudge);
    if vertical < usize::MAX {
        return (100 * (vertical + 1)).try_into().unwrap();
    }
    // Horizontal Pass
    let horizontal = vertical_scan(&transpose(grid.clone()), smudge);
    if horizontal < usize::MAX {
        return (horizontal + 1).try_into().unwrap();
    }
    panic!("No reflection point found?!")
}

fn str_to_grid(grid_str: &str) -> Vec<Vec<char>> {
    grid_str
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>()
}

fn main() {
    // Part 1
    let grids = include_str!("day13.txt")
        .split("\n\n")
        .into_iter()
        .map(|grid_str| str_to_grid(grid_str))
        .collect::<Vec<Vec<Vec<char>>>>();

    // Find reflection point for each grid, total
    let summary_part_1: u32 = grids.iter().map(|grid| reflection_value(grid, false)).sum();

    println!("{}", summary_part_1);

    // Part 2:
    // Add in smudge factor (could be neater!)
    let summary_part_2: u32 = grids.iter().map(|grid| reflection_value(grid, true)).sum();

    println!("{}", summary_part_2);
}

#[cfg(test)]
mod tests {
    use crate::str_to_grid;
    use crate::validate_vertical_reflection;

    #[test]
    fn test_validate_vertical_reflection() {
        let grid: Vec<Vec<char>> = str_to_grid(
            "#...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#",
        );
        assert_eq!(false, validate_vertical_reflection(&grid, 1 as usize));
        assert_eq!(false, validate_vertical_reflection(&grid, 1 as usize));
        assert_eq!(false, validate_vertical_reflection(&grid, 2 as usize));
        assert_eq!(true, validate_vertical_reflection(&grid, 3 as usize));
        assert_eq!(false, validate_vertical_reflection(&grid, 4 as usize));
        assert_eq!(false, validate_vertical_reflection(&grid, 5 as usize));
    }
}
