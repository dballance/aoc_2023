use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use clap::Parser;
use clap::Subcommand;


fn calculate_calibrations(line: &str) -> Option<i32> {
    let digits: Vec<char> = line.chars().filter(|x| x.is_numeric()).collect();

    if let Some(first) = digits.first() {
        if let Some(last) = digits.last() {
            let as_string = format!("{first}{last}").to_owned();
            return Some(as_string.parse::<i32>().unwrap());
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
    Day2 {}
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
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod test {
    #[test]
    fn day1_example() {
        let example = "
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
        ";

        let mut calibrations = Vec::<i32>::new();

        for line in example.lines() {
            if let Some(calibration) = super::calculate_calibrations(line) {
                calibrations.push(calibration);
            }
        }

        assert_eq!(calibrations, [12, 38, 15, 77])
    }
}