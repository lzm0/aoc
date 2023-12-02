use std::fs;

#[derive(Debug)]
struct Cubes {
    red: u32,
    green: u32,
    blue: u32,
}

type Game = Vec<Cubes>;

fn part_one(games: &Vec<Game>) -> u32 {
    let red_max: u32 = 12;
    let green_max: u32 = 13;
    let blue_max: u32 = 14;

    let mut sum = 0;
    let mut id: u32 = 1;
    for game in games {
        if game
            .iter()
            .all(|cube| cube.red <= red_max && cube.green <= green_max && cube.blue <= blue_max)
        {
            sum += id;
        }
        id += 1;
    }
    sum
}

fn part_two(games: &Vec<Game>) -> u32 {
    let mut sum = 0;
    for game in games {
        let mut red_max = 0;
        let mut green_max = 0;
        let mut blue_max = 0;

        game.iter().for_each(|cube| {
            red_max = red_max.max(cube.red);
            green_max = green_max.max(cube.green);
            blue_max = blue_max.max(cube.blue);
        });

        sum += red_max * green_max * blue_max;
    }
    sum
}

fn parse_input(input: String) -> Vec<Game> {
    input
        .lines()
        .map(|game| {
            game.split_once(": ")
                .unwrap()
                .1
                .split("; ")
                .map(|draw| {
                    let mut red = 0;
                    let mut green = 0;
                    let mut blue = 0;
                    draw.split(", ").for_each(|color| {
                        let mut cube = color.split(" ");
                        let value = cube.next().unwrap().parse().unwrap();
                        let name = cube.next().unwrap();
                        match name {
                            "red" => red = value,
                            "green" => green = value,
                            "blue" => blue = value,
                            _ => panic!("Unknown color"),
                        }
                    });
                    Cubes { red, green, blue }
                })
                .collect::<Vec<Cubes>>()
        })
        .collect::<Vec<Game>>()
}

fn main() {
    let input = fs::read_to_string("src/input/2023-02.txt").unwrap();
    let games = parse_input(input);

    println!("Part one: {}", part_one(&games));
    println!("Part two: {}", part_two(&games));
}
