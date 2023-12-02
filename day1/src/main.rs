use std::collections::HashMap;

fn main() {
    // Part 1
    fn find_first_value(line: &str, reverse: bool) -> String {
        // Use Box<dyn Iterator<Item = char>> to create a trait object that can represent both Chars and Rev<Chars>.
        // We can't know size at compile time, so we throw this onto the heap.
        let chars: Box<dyn Iterator<Item = char>> = match reverse {
            true => Box::new(line.chars().rev()),
            false => Box::new(line.chars()),
        };
        for char in chars {
            if char.is_ascii_digit() {
                return char.to_string();
            }
        }
        panic!("Whoa, no digit found in the line {line} at all!")
    }

    let calibration_values_part_1: i64 = include_str!("day1.txt")
        .lines()
        .map(|v| find_first_value(&v, false) + &find_first_value(&v, true))
        .map(|v| v.parse::<i64>().unwrap())
        .sum();

    println!("{}", calibration_values_part_1);

    // Part 2
    // To avoid issues with replaces interfering with other numbers (i.e. oneight),
    // our lookup can include the digit to replace along with the original work where possible
    let lookup = HashMap::from([
        ("zero", "zer0o"),
        ("one", "on1e"),
        ("two", "tw2o"),
        ("three", "thre3e"),
        ("four", "4"),
        ("five", "fiv5e"),
        ("six", "6"),
        ("seven", "seve7n"),
        ("eight", "eigh8t"),
        ("nine", "nin9e"),
    ]);

    fn replace_str_with_digit(mut line: String, lookup: &HashMap<&str, &str>) -> String {
        for &digit_name in lookup.keys() {
            line = line.replace(digit_name, lookup.get(digit_name).unwrap());
        }
        line
    }

    let calibration_values_part_2: i64 = include_str!("day1.txt")
        .lines()
        .map(|v| replace_str_with_digit(v.to_string(), &lookup))
        .map(|v| find_first_value(&v, false) + &find_first_value(&v, true))
        .map(|v| v.parse::<i64>().unwrap())
        .sum();

    println!("{}", calibration_values_part_2)
}
