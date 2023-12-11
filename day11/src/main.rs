
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

fn main() {
    // Part 1
    let universe: Vec<Vec<char>> = include_str!("day11.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    // TMP display universe
    universe.iter().for_each(|it| {
        println!("{:?}", it);
    });

    // Expand univese (we can use our simple vertical expansion for both directions if we transpose array!)
    let expanded = transpose(expand_universe_vertical_pass(transpose(
        expand_universe_vertical_pass(universe),
    )));

    // TMP display expansion
    expanded.iter().for_each(|it| {
        println!("{:?}", it);
    });

    // Find pairs

    // Calculate distance between pairs

    // Sum distances
}
