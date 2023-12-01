use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;
use clap::Parser;
use clap::Subcommand;

fn calculate_calibrations(line: &str) -> Option<i32> {
    let searches: HashMap<&str, i32> = HashMap::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut indexes: Vec<(usize, &str)> = searches
        .iter()
        .flat_map(|s| line.match_indices(s.0))
        .collect();
    indexes.sort_by(|a, b| a.0.cmp(&b.0));

    if let Some((_, first_match)) = indexes.first() {
        if let Some((_, last_match)) = indexes.last() {
            if let Some(first) = searches.get(first_match) {
                if let Some(last) = searches.get(last_match) {
                    let as_string = format!("{first}{last}").to_owned();
                    return Some(as_string.parse::<i32>().unwrap());
                }
            }
        }
    }

    None
}

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Day1 { input_path: String },
    Day2 {},
}

fn main() {
    println!("Saving Christmas, one day at a time.");

    let args = Args::parse();

    if let Command::Day1 { input_path } = args.command {
        let mut combined: Vec<i32> = Vec::<i32>::new();
        if let Ok(lines) = read_lines(input_path) {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(line) = line {
                    if let Some(result) = calculate_calibrations(line.as_str()) {
                        combined.push(result)
                    }
                }
            }
        }
        println!("Calibration: {:?}", combined.iter().sum::<i32>());
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod test {
    #[test]
    fn day1_example() {
        let example = "
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
        ";

        let mut calibrations = Vec::<i32>::new();

        for line in example.lines() {
            if let Some(calibration) = super::calculate_calibrations(line) {
                calibrations.push(calibration);
            }
        }

        assert_eq!(calibrations, [29, 83, 13, 24, 42, 14, 76])
    }
}
