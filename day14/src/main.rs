fn horizontal_flip(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    grid.into_iter()
        .map(|row| row.into_iter().rev().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

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

fn str_to_grid(grid_str: &str) -> Vec<Vec<char>> {
    grid_str
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>()
}

// Always tilt 'left' i.e. for each row 'OO..O.#.O' -> 'OOO...#O.'
fn tilt_row_left(row: &Vec<char>) -> Vec<char> {
    // Split into # sections, and build each by counting O and reconstructing
    row.into_iter()
        .collect::<String>()
        .split("#")
        .into_iter()
        .map(|section| {
            let rocks = section
                .chars()
                .map(|c| match c {
                    'O' => 1,
                    _ => 0,
                })
                .sum::<usize>();
            let mut new_rocks = vec!['O'].repeat(rocks);
            new_rocks.append(&mut vec!['.'].repeat(section.len() - rocks));
            new_rocks.push('#');
            new_rocks
        })
        .into_iter()
        .flat_map(|v| v)
        .collect::<Vec<char>>()[..row.len()]
        .to_vec()
}

// Always tilt 'left' i.e. for each row 'OO..O.#.O' -> 'OOO...#O.'
fn tilt_grid_left(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    grid.iter().map(tilt_row_left).collect()
}

fn tilt_north(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    transpose(tilt_grid_left(transpose(grid.to_vec())))
}

fn tilt_west(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    tilt_grid_left(grid)
}

fn tilt_east(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    horizontal_flip(tilt_grid_left(horizontal_flip(grid)))
}

fn tilt_south(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    transpose(horizontal_flip(tilt_grid_left(horizontal_flip(transpose(grid.to_vec())))))
}

fn compute_load(grid: Vec<Vec<char>>) -> u32 {
    grid.iter()
        .enumerate()
        .map(|(i, row)| {
            (grid.len() - i) as u32
                * row
                    .iter()
                    .map(|v| if v == &'O' { 1 } else { 0 })
                    .sum::<u32>()
        })
        .sum()
}

// Tilt North, West, South, East, and return resulting grid
fn cycle(grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
    tilt_east(tilt_south(tilt_west(tilt_north(grid))))
}

fn main() {
    // Part 1
    let platform = str_to_grid(include_str!("day14.txt"));

    let tilted_north = tilt_north(platform.clone());

    let load_part_1 = compute_load(tilted_north);

    println!("{}", load_part_1);

    // Part 2
    // Notice (luckily whilst testing) that we converge on some equilibruim much earlier than 1 billion!
    let mut cycled = platform;
    for _ in 0..1000 {
        cycled = cycle(cycled);
    }
    let load_part_2 = compute_load(cycled);

    println!("{}", load_part_2);
}

#[cfg(test)]
mod tests {
    use crate::tilt_row_left;

    #[test]
    fn test_tilt_row_left() {
        assert_eq!(
            "OOO.#....#".chars().collect::<Vec<char>>(),
            tilt_row_left(&"O.OO#....#".chars().collect())
        );
        assert_eq!(
            "OO..#OO..#".chars().collect::<Vec<char>>(),
            tilt_row_left(&"..OO#.O.O#".chars().collect())
        );
        assert_eq!(
            "O.#O...#.#".chars().collect::<Vec<char>>(),
            tilt_row_left(&"O.#..O.#.#".chars().collect())
        );
    }
}
