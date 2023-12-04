use std::collections::HashSet;

fn main() {
    fn create_split_hashset(numbers: &str) -> HashSet<u32> {
        HashSet::<u32>::from_iter(
            numbers
                .split_ascii_whitespace()
                .map(|v| v.parse::<u32>().unwrap())
                .collect::<Vec<u32>>(),
        )
    }

    fn count_matches(line: &str) -> u32 {
        let (_, scorecard) = line.split_once(": ").unwrap();
        let (winning_numbers, our_numbers) = scorecard.split_once(" | ").unwrap();
        let winning_set = create_split_hashset(winning_numbers);
        let our_set = create_split_hashset(our_numbers);
        winning_set
            .intersection(&our_set)
            .count()
            .try_into()
            .unwrap()
    }

    // Part 1:
    let winnings: u32 = include_str!("day4.txt")
        .lines()
        .map(count_matches)
        .filter(|matches| matches > &0)
        .map(|matches| 2_u32.pow(matches - 1))
        .sum();

    println!("{}", winnings);
}
