use std::{iter::zip, ops::Div};

fn main() {
    // Part 1
    fn parse(to_parse: &str) -> Vec<i64> {
        to_parse
            .split_once(":")
            .unwrap()
            .1
            .split_ascii_whitespace()
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<i64>>()
    }

    let (time_str, distance_str) = include_str!("day6.txt").split_once("\n").unwrap();

    fn compute_quadratic_roots(time: i64, dist: i64) -> i64 {
        let d = f64::sqrt((time.pow(2) as f64) - (4 * dist) as f64);
        return (f64::ceil((-time as f64 + d).div(2.0) as f64)
            - f64::floor((-time as f64 - d).div(2.0) as f64)) as i64
            - 1;
    }

    let part1: i64 = zip(parse(time_str), parse(distance_str))
        .map(|(time, dist)| compute_quadratic_roots(time, dist))
        .product();

    println!("{:?}", part1);

    // Part 2  (we needed to swap to 64 bit numbers to parse largest input)
    fn parse_as_single(to_parse: &str) -> i64 {
        to_parse
            .split_once(":")
            .unwrap()
            .1
            .split_ascii_whitespace()
            .collect::<Vec<&str>>()
            .join("")
            .parse::<i64>()
            .unwrap()
    }

    let part2: i64 =
        compute_quadratic_roots(
            parse_as_single(time_str), 
            parse_as_single(distance_str),
        );

    println!("{:?}", part2);
}
