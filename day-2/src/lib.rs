use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Debug, PartialEq, Eq)]
enum Day2Result {
    Possible(u32),
    Impossible,
}

fn part_1_validator(amount: usize, color: &str) -> bool {
    match color {
        "red" => amount <= 12,
        "green" => amount <= 13,
        "blue" => amount <= 14,
        _ => false,
    }
}

fn part_one_resolver(line: &str) -> Day2Result {
    let info = line.split(": ").collect::<Vec<&str>>();
    let game_info = info[0];
    let sets = info[1];
    let mut sets = sets.split("; ");

    let game_id = game_info.split(" ").collect::<Vec<&str>>()[1];
    let mut result = Day2Result::Possible(game_id.parse::<u32>().unwrap());
    while let Some(set) = sets.next() {
        let mut set = set.split(", ");

        while let Some(cube) = set.next() {
            let cube = cube.split(" ").collect::<Vec<&str>>();
            let amount = cube[0].parse::<usize>().unwrap();
            let color = cube[1];

            if !part_1_validator(amount, color) {
                result = Day2Result::Impossible;
            }
        }
    }
    result
}

fn part_two_resolver(line: &str) -> usize {
    let info = line.split(": ").collect::<Vec<&str>>();

    let sets = info[1];
    let mut sets = sets.split("; ");

    let mut rgb = (0, 0, 0);

    while let Some(set) = sets.next() {
        let mut set = set.split(", ");

        while let Some(cube) = set.next() {
            let cube = cube.split(" ").collect::<Vec<&str>>();
            let amount = cube[0].parse::<usize>().unwrap();
            let color = cube[1];

            match color {
                "red" => {
                    if amount > rgb.0 {
                        rgb.0 = amount;
                    }
                }
                "green" => {
                    if amount > rgb.1 {
                        rgb.1 = amount;
                    }
                }
                "blue" => {
                    if amount > rgb.2 {
                        rgb.2 = amount;
                    }
                }
                _ => {}
            }
        }
    }

    rgb.0 * rgb.1 * rgb.2
}

fn day_2<I>(lines: I, part_two: bool) -> u32
where
    I: IntoIterator<Item = io::Result<String>>,
{
    let mut results = vec![];
    let mut results_pt2 = vec![];
    for line in lines {
        match line {
            Ok(line) => {
                if part_two {
                    results_pt2.push(part_two_resolver(&line));
                } else {
                    results.push(part_one_resolver(&line));
                }
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
            }
        }
    }

    if part_two {
        return results_pt2.iter().sum::<usize>() as u32;
    }

    results
        .iter()
        .filter(|result| match result {
            Day2Result::Possible(_) => true,
            Day2Result::Impossible => false,
        })
        .map(|result| match result {
            Day2Result::Possible(value) => *value,
            Day2Result::Impossible => 0,
        })
        .sum::<u32>()
}

pub fn run() -> Result<(), io::Error> {
    let file = File::open("day-2/input")?;
    let reader = BufReader::new(file);
    let answer = day_2(reader.lines(), true);

    println!("Answer: {}", answer);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_2() {
        let input = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];
        let answer = day_2(input.into_iter().map(|s| Ok(s.to_string())), false);
        assert_eq!(answer, 8);
    }
    #[test]
    fn test_day_2_pt2() {
        let input = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];
        let answer = day_2(input.into_iter().map(|s| Ok(s.to_string())), true);
        assert_eq!(answer, 2286);
    }
}
