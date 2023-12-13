use std::cmp::min;

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
    let steps = min(boundary, grid.len() - boundary - 2);
    for i in 0..steps + 1 {
        if grid[boundary - i] != grid[boundary + i + 1] {
            return false;
        }
    }
    true
}

fn vertical_scan(grid: &Vec<Vec<char>>) -> usize {
    // Find all possible reflection start points (two identical continuous rows)
    let start_points: Vec<usize> = grid[1..]
        .iter()
        .enumerate()
        .map(|(i, row)| if row == &grid[i] { i } else { usize::MAX })
        .filter(|i| i < &usize::MAX)
        .collect();

    // If we see more than one validate each on whole grid
    for start_point in start_points.into_iter() {
        if validate_vertical_reflection(grid, start_point) {
            return start_point;
        }
    }
    return usize::MAX;
}

fn reflection_value(grid: &Vec<Vec<char>>) -> u32 {
    // Vertical Pass
    let vertical = vertical_scan(grid);
    if vertical < usize::MAX {
        return (100 * (vertical + 1)).try_into().unwrap();
    }
    // Horizontal Pass
    let horizontal = vertical_scan(&transpose(grid.clone()));
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
    let summary: u32 = grids.iter().map(|grid| reflection_value(grid)).sum();

    println!("{}", summary);
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
