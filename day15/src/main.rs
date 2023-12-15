use std::ops::Rem;

fn hash(input: &str) -> u32 {
    let mut current = 0;
    for c in input.chars() {
        current += c as u32;
        current *= 17;
        current = current.rem(256);
    }
    current
}

fn main() {
    // Part 1
    let output_part_1: u32 = include_str!("day15.txt").split(",").map(hash).sum();
    println!("{}", output_part_1);
}

#[cfg(test)]
mod tests {
    use crate::hash;

    #[test]
    fn test_hash() {
        assert_eq!(52, hash("HASH"));
    }
}
