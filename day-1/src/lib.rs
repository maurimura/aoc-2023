use std::{
    collections::VecDeque,
    fs::File,
    io::{self, BufRead, BufReader, Error},
};

fn save_digit(digit: u32, digit_tuple: Option<(u32, u32)>) -> Option<(u32, u32)> {
    match digit_tuple {
        Some((a, _)) => Some((a, digit)),
        None => Some((digit, digit)),
    }
}

fn digit_case_handler(char: char, digit_tuple: Option<(u32, u32)>) -> Option<(u32, u32)> {
    if !char.is_numeric() {
        return digit_tuple;
    }

    let number = char.to_digit(10).unwrap();
    save_digit(number, digit_tuple)
}

const SPELLED_NUMBERS: &'static [(&str, u32); 9] = &[
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn is_valid_char_at_position(char: char, queue: &VecDeque<char>) -> bool {
    let mut valid = false;
    let position = queue.len();
    for (spelled_number, _) in SPELLED_NUMBERS {
        if spelled_number.len() <= position {
            continue;
        }

        let spelled_number_current_char = spelled_number.chars().nth(position).unwrap();

        if spelled_number_current_char == char
            && spelled_number.starts_with(queue.iter().collect::<String>().as_str())
        {
            valid = true;
            break;
        }
    }
    valid
}

fn process_queue(letters_queue: &VecDeque<char>) -> Option<u32> {
    let mut maybe_digit: Option<u32> = None;

    for (spelled_number, number) in SPELLED_NUMBERS {
        if spelled_number.len() != letters_queue.len() {
            continue;
        }

        let mut valid = true;
        for (i, char) in letters_queue.iter().enumerate() {
            if spelled_number.chars().nth(i).unwrap() != *char {
                valid = false;
                break;
            }
        }

        if valid {
            maybe_digit = Some(*number);
            break;
        }
    }
    maybe_digit
}

fn maybe_digit_from_queue(char: char, letters_queue: &mut VecDeque<char>) -> Option<u32> {
    let queue_len = letters_queue.len();
    if !is_valid_char_at_position(char, &letters_queue) {
        if queue_len > 0 {
            letters_queue.clear();
            return maybe_digit_from_queue(char, letters_queue);
        } else {
            return None;
        }
    }
    letters_queue.push_back(char);
    process_queue(letters_queue)
}

fn spelled_case_handler(
    char: char,
    digit_tuple: Option<(u32, u32)>,
    letters_queue: &mut VecDeque<char>,
) -> Option<(u32, u32)> {
    if !char.is_alphabetic() {
        letters_queue.clear();
        return digit_tuple;
    }
    let maybe_digit = maybe_digit_from_queue(char, letters_queue);
    match maybe_digit {
        Some(digit) => {
            letters_queue.clear();
            let digit_tuple = save_digit(digit, digit_tuple);
            digit_tuple
        }
        None => digit_tuple,
    }
}

fn parse_tuple_valu(t: Option<(u32, u32)>) -> u32 {
    match t {
        Some((a, b)) => {
            return a * 10 + b;
        }
        None => {
            return 0;
        }
    }
}

fn get_line_value_part_1(line: String) -> u32 {
    let mut digit_tuple: Option<(u32, u32)> = None;

    for char in line.chars() {
        digit_tuple = digit_case_handler(char, digit_tuple);
    }

    parse_tuple_valu(digit_tuple)
}

fn get_line_value_part_2(line: String) -> u32 {
    let mut digit_tuple: Option<(u32, u32)> = None;
    let mut letters_queue: VecDeque<char> = VecDeque::new();
    for char in line.chars() {
        if !char.is_alphanumeric() {
            letters_queue.clear();
            continue;
        }

        if char.is_numeric() {
            letters_queue.clear();
            digit_tuple = digit_case_handler(char, digit_tuple);
            continue;
        }

        if char.is_alphabetic() {
            digit_tuple = spelled_case_handler(char, digit_tuple, &mut letters_queue);
            continue;
        }
    }
    if line.chars().next().unwrap().is_alphabetic()
        && line.chars().last().unwrap().is_alphabetic()
        && line.len() > 32
    {
        println!("{} {:?}\n", line, digit_tuple)
    }
    parse_tuple_valu(digit_tuple)
}

fn day_1<I>(lines: I, part_two: bool) -> u32
where
    I: IntoIterator<Item = io::Result<String>>,
{
    let mut total = 0;
    for line in lines {
        match line {
            Ok(line) => {
                if part_two {
                    total += get_line_value_part_2(line);
                } else {
                    total += get_line_value_part_1(line);
                }
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                total += 0;
            }
        }
    }

    total
}

pub fn run() -> Result<(), Error> {
    let file = File::open("day-1/input")?;
    let reader = BufReader::new(file);

    let answer = day_1(reader.lines(), true);
    println!("Answer: {}", answer);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_1() {
        let input = vec![
            "1abc2".to_string(),
            "pqr3stu8vwx".to_string(),
            "a1b2c3d4e5f".to_string(),
            "treb7uchet".to_string(),
        ];
        let total = day_1(input.into_iter().map(|s| Ok(s)), false);
        assert_eq!(total, 142)
    }

    #[test]
    fn test_with_part_1_answer() {
        let file = File::open("./input").unwrap();
        let reader = BufReader::new(file);

        let answer = day_1(reader.lines(), false);
        assert_eq!(answer, 55017)
    }

    #[test]
    fn test_day_2() {
        let input = vec![
            "two1nine".to_string(),
            "eightwothree".to_string(),
            "abcone2threexyz".to_string(),
            "xtwone3four".to_string(),
            "4nineeightseven2".to_string(),
            "zoneight234".to_string(),
            "7pqrstsixteen".to_string(),
        ];
        let total = day_1(input.into_iter().map(|s| Ok(s)), true);
        assert_eq!(total, 281)
    }
    #[test]
    fn test_day_2_more_data() {
        let input = vec![
            "gsntbddbnone4cjqjmspzcsxmvvthreefive".to_string(),
            "one7three9threeoneonetwo".to_string(),
            "8pztdljxbjjthreenineeightseven7crkdr8eightwocb".to_string(),
        ];
        let total = day_1(input.into_iter().map(|s| Ok(s)), true);
        assert_eq!(total, 15 + 12 + 88)
    }
}
