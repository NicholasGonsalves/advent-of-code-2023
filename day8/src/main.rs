use std::collections::HashMap;

fn main() {
    // Part 1
    fn parse_graph(input: &str) -> HashMap<&str, (&str, &str)> {
        let mut graph = HashMap::<&str, (&str, &str)>::new();

        // We can guarentee structure is AAA = (BBB, CCC) so we can index in directly
        for line in input.lines() {
            graph.insert(&line[0..3], (&line[7..10], &line[12..15]));
        }

        graph
    }

    fn walk_graph(
        lr_index: usize,
        node: &str,
        steps: u32,
        lrorder: Vec<char>,
        graph: HashMap<&str, (&str, &str)>,
    ) -> u32 {
        if node == "ZZZ" {
            return steps;
        }
        let dir = &lrorder[lr_index];
        let next_node = match dir {
            'L' => graph.get(node).unwrap().0,
            'R' => graph.get(node).unwrap().1,
            _ => panic!("Expected L or R, found {dir}"),
        };
        let next_lr_index = match lr_index {
            lr_index if lr_index + 1 < lrorder.len() => lr_index + 1,
            _ => 0,
        };
        return walk_graph(next_lr_index, next_node, steps + 1, lrorder, graph);
    }

    let (lrorder_str, graph_str) = include_str!("day8.txt").split_once("\n\n").unwrap();

    let graph = parse_graph(graph_str);
    let lrorder = lrorder_str.chars().collect::<Vec<char>>();

    let steps_part_1 = walk_graph(0, "AAA", 0, lrorder, graph);
    println!("{:?}", steps_part_1);
}
