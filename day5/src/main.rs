use std::{collections::HashMap, cmp::{min, max}};

#[derive(Debug)]
struct Range {
    source: u64,
    destination: u64,
    size: u64,
}

impl Range {
    fn from_line(line: &str) -> Range {
        let mut values = line.split_ascii_whitespace();
        Range {
            destination: values.next().unwrap().parse::<u64>().unwrap(),
            source: values.next().unwrap().parse::<u64>().unwrap(),
            size: values.next().unwrap().parse::<u64>().unwrap(),
        }
    }

    fn contains(&self, value: u64) -> bool {
        self.source <= value && value <= self.source + self.size
    }

    fn destination_from_source(&self, value: u64) -> u64 {
        value - self.source + self.destination
    }
}

fn main() {
    fn parse_into_hashmap(map_str: &str) -> HashMap<&str, Vec<Range>> {
        let (map_name_str, map_data) = map_str.split_once("\n").unwrap();
        let map_name = map_name_str.split_once(" ").unwrap().0;
        let ranges = map_data
            .lines()
            .map(Range::from_line)
            .collect::<Vec<Range>>();
        HashMap::from([(map_name, ranges)])
    }

    /// Recursively walk through the graph, updating the value as each range applies an offset
    fn walk_graph(mappings: &HashMap<&str, Vec<Range>>, mut value: u64, start_node: &str) -> u64 {
        let current_node = mappings
            .keys()
            .filter(|&k| k.starts_with(start_node))
            .cloned()
            .next();

        // If there are no more nodes, return the value!
        if current_node.is_none() {
            return value;
        }

        // Find new value from mappings
        for range in mappings.get(&current_node.unwrap()).unwrap() {
            if range.contains(value) {
                value = range.destination_from_source(value);
                break;
            }
        }

        let next_node_prefix = current_node.unwrap().split_once("-to-").unwrap().1;
        return walk_graph(mappings, value, next_node_prefix);
    }

    // Part 1
    let (seeds_str, mappings_str) = include_str!("day5.txt").split_once("\n\n").unwrap();
    let seeds: Vec<u64> = seeds_str
        .split_at(7)
        .1
        .split_ascii_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let mappings = mappings_str
        .split("\n\n")
        .map(parse_into_hashmap)
        .flat_map(|map| map)
        .collect::<HashMap<&str, Vec<Range>>>();

    // For each inital seed, walk 'graph' to find location. Take lowest.
    let closest_location = seeds
        .iter()
        .map(|seed| walk_graph(&mappings, *seed, "seed"))
        .min();

    println!("{:?}", closest_location);

    // Part 2: We can't brute force for every single seed.. but lets try anyway
    let closest_location_seed_ranges = seeds
        .chunks(2)
        .flat_map(|range|range[0]..(range[0]+range[1]))
        .fold(u64::MAX, |global_minumum, seed| {
            min(global_minumum, walk_graph(&mappings, seed, "seed"))
        });

    println!("{:?}", closest_location_seed_ranges);  // Nope!

}
