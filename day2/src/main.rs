struct Game {
    number: i64,
    red: i64,
    blue: i64,
    green: i64,
}

fn parse_game(line: &str) -> Game {
    let (game_name, game_rounds) = line.split_once(": ").unwrap();
    let (_, game_number_str) = game_name.split_once(" ").unwrap();

    let mut game = Game {
        number: game_number_str.parse::<i64>().unwrap(),
        red: 0,
        blue: 0,
        green: 0,
    };

    for round in game_rounds.split("; ") {
        for hand in round.split(", ") {
            let mut parts = hand.split(" ");
            let _ = match (
                parts.next().unwrap().parse::<i64>().unwrap(),
                parts.next().unwrap(),
            ) {
                (v, "red") => game.red = game.red.max(v),
                (v, "green") => game.green = game.green.max(v),
                (v, "blue") => game.blue = game.blue.max(v),
                (_, _) => panic!("Oh no!"),
            };
        }
    }
    game
}

fn main() {
    // Part 1
    let possible_game_sum_part_1: i64 = include_str!("day2.txt")
        .lines()
        .map(parse_game)
        .map(|g| {
            if g.red <= 12 && g.green <= 13 && g.blue <= 14 {
                g.number
            } else {
                0
            }
        })
        .sum();

    println!("{:?}", possible_game_sum_part_1);

    // Part 2
    let possible_game_sum_part_2: i64 = include_str!("day2.txt")
        .lines()
        .map(parse_game)
        .map(|g| g.red * g.green * g.blue)
        .sum();

    println!("{:?}", possible_game_sum_part_2);
}
