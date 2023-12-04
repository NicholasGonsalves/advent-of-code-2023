use std::{cmp::min, collections::HashSet};

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

    // Part 2:
    let all_scorecards = include_str!("day4.txt").lines().collect::<Vec<&str>>();

    // Luckily, a scorecard win can only give you more *later* scorecards, (so we can start from the beginning).
    let mut counts = vec![1_u32; all_scorecards.len()];

    for (i, &line) in all_scorecards.iter().enumerate() {
        let winnings = count_matches(line);
        for j in i..min(i + usize::try_from(winnings).unwrap(), all_scorecards.len()) {
            counts[j + 1] += counts[i];
        }
    }

    let total_scorecards: u32 = counts.iter().sum();
    println!("{:#?}", total_scorecards);
}
