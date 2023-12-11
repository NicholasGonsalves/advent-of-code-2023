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

    for row in &universe {
        if row.iter().all(|val| val == &'.' || val == &'@') {
            expanded.push((0..universe.len()).map(|_| '@').collect::<Vec<char>>());
        } else {
            expanded.push(row.clone());
        }
    }

    expanded
}

fn find_galaxy_positions(expanded: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
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

fn distance(
    p1: &(usize, usize),
    p2: &(usize, usize),
    expansion_constant: usize,
    expanded_universe: &Vec<Vec<char>>,
) -> usize {
    // Count number of @ (wormhole!) crossings
    let (mut x1, mut y1) = p1.clone();
    let (mut x2, mut y2) = p2.clone();

    // Swap so we always walk from smaller to larger value
    if x1 > x2 {
        (x1, x2) = (x2.clone(), x1.clone());
    }
    if y1 > y2 {
        (y1, y2) = (y2.clone(), y1.clone());
    }

    // 'Walk' between galaxies and count the number of wormhole crossings
    let mut wormholes = 0;
    for i in x1..x2 + 1 {
        if expanded_universe[i][y1] == '@' {
            wormholes += 1;
        }
    }
    for j in y1..y2 + 1 {
        if expanded_universe[x2][j] == '@' {
            wormholes += 1;
        }
    }

    // Calculate the distance, including the addtional distances that the 'wormholes' represent
    p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1) + (wormholes * expansion_constant) - wormholes
}

fn main() {
    // Part 1
    let universe: Vec<Vec<char>> = include_str!("day11.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    // Expand univese (we can use our simple vertical expansion for both directions if we transpose array!)
    // Expansion modified to handle very large expansion constants - we mark the expansion boundary with @ (a wormhole?!)
    let expanded = transpose(expand_universe_vertical_pass(transpose(
        expand_universe_vertical_pass(universe),
    )));

    let galaxy_positions = find_galaxy_positions(&expanded);

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
    let total_distance_expansion_2: usize = unique_pairs
        .iter()
        .map(|(p1, p2)| distance(p1, p2, 2, &expanded))
        .sum();

    println!("{}", total_distance_expansion_2);

    // Part 2
    let total_distance_expansion_1_000_000: usize = unique_pairs
        .iter()
        .map(|(p1, p2)| distance(p1, p2, 1_000_000, &expanded))
        .sum();

    println!("{}", total_distance_expansion_1_000_000);
}
