use itertools::Itertools;

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

fn expand_universe_vertical_pass(universe: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut expanded = Vec::<Vec<char>>::new();

    for row in universe {
        if row.iter().all(|val| val == &'.') {
            expanded.push(row.clone());
            expanded.push(row);
        } else {
            expanded.push(row);
        }
    }

    expanded
}

fn find_galaxy_positions(expanded: Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut galaxy_positions = Vec::<(usize, usize)>::new();
    for i in 0..expanded.len() {
        for j in 0..expanded[0].len() {
            if match expanded[i][j] {
                '#' => true,
                _ => false,
            } {
                galaxy_positions.push((i, j))
            }
        }
    }
    galaxy_positions
}

fn distance(p1: &(usize, usize), p2: &(usize, usize)) -> usize {
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1)
}

fn main() {
    // Part 1
    let universe: Vec<Vec<char>> = include_str!("day11.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    // Display universe
    // universe.iter().for_each(|it| {
    //     println!("{:?}", it);
    // });

    // Expand univese (we can use our simple vertical expansion for both directions if we transpose array!)
    let expanded = transpose(expand_universe_vertical_pass(transpose(
        expand_universe_vertical_pass(universe),
    )));

    // Display expansion
    // expanded.iter().for_each(|it| {
    //     println!("{:?}", it);
    // });

    let galaxy_positions = find_galaxy_positions(expanded);

    let unique_pairs = galaxy_positions
        .iter()
        .cartesian_product(galaxy_positions.iter())
        .filter(|(a, b)| a != b)
        .map(|(a, b)| {
            // Sort inner tuples so our uniqueness check can catch swapped duplicates
            let sorted_pair = if a <= b { (a, b) } else { (b, a) };
            sorted_pair
        })
        .unique()
        .collect::<Vec<(&(usize, usize), &(usize, usize))>>();

    // Calculate total distance between all pairs
    let total_distance: usize = unique_pairs
        .into_iter()
        .map(|(p1, p2)| distance(p1, p2))
        .sum();

    println!("{}", total_distance);
}
