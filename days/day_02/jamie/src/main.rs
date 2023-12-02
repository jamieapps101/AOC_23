use day_02_lib::{LoadGames, Round};

fn main() {
    let stdin = std::io::stdin()
        .lines()
        .map_while(Result::ok)
        .filter(|s| !s.is_empty());
    let sum_of_ids = stdin
        .load_games()
        .map(|r| r.unwrap())
        .map(|g| {
            let mut min_round = Round {
                red: u32::MIN,
                green: u32::MIN,
                blue: u32::MIN,
            };
            g.rounds.iter().for_each(|r| {
                min_round.red = min_round.red.max(r.red);
                min_round.blue = min_round.blue.max(r.blue);
                min_round.green = min_round.green.min(r.green);
            });
            let power = min_round.red * min_round.blue * min_round.green;
            power
        })
        .sum::<u32>();
    println!("sum_of_ids: {sum_of_ids}");
}
