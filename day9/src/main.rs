fn main() {
    fn parse_history(line: &str) -> Vec<i64> {
        line.split_ascii_whitespace()
            .map(|value| value.parse().unwrap())
            .collect::<Vec<i64>>()
    }

    fn predict(history: Vec<i64>) -> i64 {
        // Bottom level reached
        if history.iter().all(|value| value == &0) {
            return 0;
        };

        let diffs = history[1..]
            .iter()
            .enumerate()
            .map(|(i, value)| value - history[i])
            .collect::<Vec<i64>>();

        return predict(diffs) + history.last().unwrap();
    }

    let prediction_sum: i64 = include_str!("day9.txt")
        .lines()
        .map(parse_history)
        .map(predict)
        .sum();

    println!("{}", prediction_sum);
}
