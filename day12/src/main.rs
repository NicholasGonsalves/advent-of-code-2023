use itertools::Itertools;

/// Validate that the layout is in the form outlined by groups
fn validate_spring(spring: &str, groups: &Vec<u32>) -> bool {
    groups == &count_groups(spring)
}

fn count_groups(spring: &str) -> Vec<u32> {
    let mut spring_groups = vec![];
    let mut curr = 0;
    for c in spring.chars() {
        match c {
            '#' => {
                curr += 1;
            }
            '.' => {
                if curr > 0 {
                    spring_groups.push(curr);
                    curr = 0;
                }
            }
            _ => panic!("You can't validate a spring that is incomplete!"),
        }
    }
    if curr > 0 {
        spring_groups.push(curr);
    }
    spring_groups
}

fn replace_unknowns(spring: &str, guess: &str) -> String {
    let mut new_guess = Vec::<char>::new();
    let mut guess_index: i32 = -1;
    for c in spring.chars() {
        new_guess.push(match c {
            '?' => {
                guess_index += 1;
                guess.chars().nth(guess_index as usize).unwrap()
            }
            _ => c,
        })
    }
    new_guess.iter().collect::<String>()
}

fn count_arrangements(line: &str) -> u32 {
    let (spring, groups_str) = line.split_once(" ").unwrap();
    let groups = groups_str
        .split(",")
        .into_iter()
        .map(|v| v.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    // Get the number of unknwon positions
    let n_unknown = spring
        .chars()
        .map(|c| match c {
            '?' => 1,
            _ => 0,
        })
        .sum();

    // Try every combination of unknown values in turn (cartesian product of '#.') and count valid options
    (0..n_unknown)
        .into_iter()
        .map(|_| vec!['#', '.'])
        .collect::<Vec<Vec<char>>>()
        .into_iter()
        .map(IntoIterator::into_iter)
        .multi_cartesian_product()
        .map(|v| v.iter().collect::<String>())
        .map(|v| replace_unknowns(spring, &v))
        .map(|v| match validate_spring(&v, &groups) {
            true => 1,
            false => 0,
        })
        .sum()
}

fn main() {
    // Part 1
    let arrangements_part_1: u32 = include_str!("day12.txt")
        .lines()
        .map(|line| count_arrangements(line))
        .sum();

    println!("{}", arrangements_part_1);
}

#[cfg(test)]
mod tests {
    use crate::validate_spring;

    #[test]
    fn test_validate_sprint() {
        assert_eq!(true, validate_spring(".###.##.#...", &vec![3, 2, 1]));
        assert_eq!(true, validate_spring(".###.##..#..", &vec![3, 2, 1]));
        assert_eq!(true, validate_spring(".###.##...#.", &vec![3, 2, 1]));
        assert_eq!(true, validate_spring(".###.##....#", &vec![3, 2, 1]));
        assert_eq!(true, validate_spring(".###..##.#..", &vec![3, 2, 1]));
        assert_eq!(true, validate_spring(".###..##..#.", &vec![3, 2, 1]));
        assert_eq!(true, validate_spring(".###..##...#", &vec![3, 2, 1]));
        assert_eq!(true, validate_spring(".###...##.#.", &vec![3, 2, 1]));
        assert_eq!(true, validate_spring(".###...##..#", &vec![3, 2, 1]));
        assert_eq!(true, validate_spring(".###....##.#", &vec![3, 2, 1]));

        assert_eq!(false, validate_spring("##.#.##.#...", &vec![3, 2, 1]));
    }
}
