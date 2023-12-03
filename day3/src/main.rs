use std::collections::HashMap;

fn main() {
    // Part 1
    fn in_bounds(i: isize, j: isize, grid: &Vec<Vec<char>>) -> bool {
        0 <= i
            && i < grid.len().try_into().unwrap()
            && 0 <= j
            && j < grid[0].len().try_into().unwrap()
    }

    fn check_symbol_adjacent(
        i: isize,
        j: isize,
        length: isize,
        engine: &Vec<Vec<char>>,
        gears: &mut HashMap<(usize, usize), Vec<i32>>,
        buffer: &Vec<char>,
    ) -> bool {
        // i & j are the final position of a set of characters making up the overall number
        // therefore we must also know the length of the set, so we can search the entire space
        let dirs: Vec<(isize, isize)> = vec![
            (-1, 0),
            (0, -1),
            (1, 0),
            (0, 1),
            (-1, -1),
            (-1, 1),
            (1, -1),
            (1, 1),
        ];
        for offset in 0..length {
            for (i_dir, j_dir) in &dirs {
                if in_bounds(i + i_dir, j + j_dir - offset, &engine) {
                    let i_off: usize = (i + i_dir).try_into().unwrap();
                    let j_off: usize = (j + j_dir - offset).try_into().unwrap();
                    if !engine[i_off][j_off].is_alphanumeric() && engine[i_off][j_off] != '.' {
                        if engine[i_off][j_off] == '*' {
                            // Store gear ratios for part 2
                            let value = buffer.iter().collect::<String>().parse::<i32>().unwrap();
                            gears
                                .entry((i_off, j_off))
                                .or_insert(Vec::new())
                                .push(value);
                        };
                        return true;
                    }
                }
            }
        }
        false
    }

    // Load input into 2D vec
    let engine = include_str!("day3.txt")
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();

    // Scan through arr, checking each number for an adjacent symbol
    let mut total = 0;
    let mut buffer = Vec::<char>::new();
    let mut gears = HashMap::<(usize, usize), Vec<i32>>::new(); // (Part 2)

    for i in 0..engine.len() {
        for j in 0..engine[0].len() {
            if engine[i][j].is_ascii_digit() {
                buffer.push(engine[i][j]);
            } else if !buffer.is_empty() {
                // Evaluate buffer
                if check_symbol_adjacent(
                    i.try_into().unwrap(),
                    (j - 1).try_into().unwrap(),
                    buffer.len().try_into().unwrap(),
                    &engine,
                    &mut gears,
                    &buffer,
                ) {
                    // This number is valid, so empty the buffer, and add the value to the total
                    total += buffer
                        .drain(0..)
                        .collect::<String>()
                        .parse::<i32>()
                        .unwrap();
                } else {
                    // Empty buffer
                    buffer.drain(0..);
                }
            }
        }
        // End of line, so evaluate the buffer if not empty because numbers cannot cross-lines
        if !buffer.is_empty() {
            // Evaluate buffer
            if check_symbol_adjacent(
                i.try_into().unwrap(),
                (engine[0].len() - 1).try_into().unwrap(),
                buffer.len().try_into().unwrap(),
                &engine,
                &mut gears,
                &buffer,
            ) {
                // This number is valid, so empty the buffer, and add the value to the total
                total += buffer
                    .drain(0..)
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap();
            }
        }
        buffer.drain(0..);
    }

    // Part 1
    println!("{}", total);

    // Part 2
    let gear_ratio_total: i32 = gears
        .values()
        .filter(|ratios| ratios.len() == 2)
        .map(|ratios| ratios.iter().product::<i32>())
        .sum();

    println!("{}", gear_ratio_total);
}
