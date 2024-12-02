use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::num::ParseIntError;

fn main() -> std::io::Result<()> {
    part_1("input.txt")?;
    part_2("input.txt")?;
    Ok(())
}

fn part_1(file_name: &str) -> std::io::Result<()> {
    let input = BufReader::new(File::open(file_name).unwrap());
    let mut n_valids = 0;
    for line in input.lines() {
        let line = line?;
        let elements: Result<Vec<i64>, ParseIntError> =
            line.split(' ').map(|element| element.parse()).collect();
        let report = elements.map_err(|err| Error::new(ErrorKind::InvalidData, err))?;
        if is_valid_report(&report) {
            n_valids += 1;
        }
    }
    println!("N report valids are: {n_valids}");
    Ok(())
}

fn part_2(file_name: &str) -> std::io::Result<()> {
    let input = BufReader::new(File::open(file_name).unwrap());
    let mut n_valids = 0;
    for line in input.lines() {
        let line = line?;
        let elements: Result<Vec<i64>, ParseIntError> =
            line.split(' ').map(|element| element.parse()).collect();
        let report = elements.map_err(|err| Error::new(ErrorKind::InvalidData, err))?;
        if is_valid_report(&report) {
            n_valids += 1;
            continue;
        }
        for n in 0..report.len() {
            if is_valid_report(&([&report[0..n], &report[n + 1..report.len()]]).concat()) {
                n_valids += 1;
                break;
            }
        }
    }
    println!("N report valids are: {n_valids}");
    Ok(())
}

enum Order {
    Increasing,
    Decreasing,
}

fn is_valid_report(report: &[i64]) -> bool {
    let mut order = None;
    for i in 1..report.len() {
        let diff = (report[i - 1] - report[i]).abs();
        if !(1..=3).contains(&diff) {
            return false;
        }
        match order {
            None if report[i - 1] > report[i] => order = Some(Order::Decreasing),
            None => order = Some(Order::Increasing),
            Some(Order::Increasing) => {
                if report[i - 1] > report[i] {
                    return false;
                }
            }
            Some(Order::Decreasing) => {
                if report[i - 1] < report[i] {
                    return false;
                }
            }
        }
    }
    true
}
