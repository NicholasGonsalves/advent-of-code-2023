fn main() {
    
    
    fn parse_history(line: &str) -> Vec<u64> {
        line.split_ascii_whitespace().map(|value| value.parse().unwrap()).collect::<Vec<u64>>()
    }

    fn predict(history: Vec<u64>) -> u64 {
        if history.iter().sum() == 0 {
            // Bottom level reached

        };


    }

    let prediction_sum: u64 = include_str!("day9.txt")
        .lines()
        .map(parse_history)
        .map(predict)
        .sum();

    println!("{}", prediction_sum);

}
