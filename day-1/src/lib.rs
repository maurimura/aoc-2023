use std::{
    fs::File,
    io::{self, BufRead, BufReader, Error},
};

fn get_line_value(line: String) -> u32 {
    let mut digit_tuple: Option<(u32, u32)> = None;

    for char in line.chars() {
        if !char.is_numeric() {
            continue;
        }

        let number = char.to_digit(10).unwrap();
        match digit_tuple {
            Some((a, _)) => {
                digit_tuple = Some((a, number));
            }
            None => {
                digit_tuple = Some((number, number));
            }
        }
    }

    match digit_tuple {
        Some((a, b)) => {
            return a * 10 + b;
        }
        None => {
            return 0;
        }
    }
}

fn day_1<I>(lines: I) -> u32
where
    I: IntoIterator<Item = io::Result<String>>,
{
    let mut total = 0;
    for line in lines {
        match line {
            Ok(line) => {
                total += get_line_value(line);
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

    let answer = day_1(reader.lines());
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
        let total = day_1(input.into_iter().map(|s| Ok(s)));
        assert_eq!(total, 142)
    }

    #[test]
    fn test_with_answer() {
        let file = File::open("./input").unwrap();
    let reader = BufReader::new(file);

    let answer = day_1(reader.lines());
        assert_eq!(answer, 55017)
    }
}
