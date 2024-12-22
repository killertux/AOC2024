use itertools::Itertools;
use std::{
    collections::HashMap,
    fs::read_to_string,
    io::{Error, ErrorKind, Result},
    u64,
};

fn main() -> Result<()> {
    println!("Part 1 example: {}", part_1("example_1.txt")?);
    println!("Part 1 result: {}", part_1("input.txt")?);
    println!("Part 2 example: {}", part_2("example_2.txt")?);
    println!("Part 2 result: {}", part_2("input.txt")?);
    Ok(())
}

fn part_1(file: &str) -> Result<u64> {
    let secret_numbers = read_input(file)?;
    Ok(secret_numbers
        .into_iter()
        .map(|mut secret_number| {
            for _ in 0..2000 {
                secret_number = calculate_next_secret_number(secret_number);
            }
            secret_number
        })
        .sum::<u64>())
}

fn part_2(file: &str) -> Result<u64> {
    let secret_numbers = read_input(file)?;
    let prices_and_diff_by_buyers: Vec<Vec<(u8, i8)>> = secret_numbers
        .into_iter()
        .map(|secret_number| calculate_prices_and_changes(secret_number))
        .collect();

    let buyers_sequences = prices_and_diff_by_buyers
        .into_iter()
        .map(|prices_and_diff| {
            prices_and_diff.into_iter().tuple_windows().fold(
                HashMap::new(),
                |mut acc, (a, b, c, d)| {
                    acc.entry((a.1, b.1, c.1, d.1)).or_insert(d.0 as u64);
                    acc
                },
            )
        })
        .fold(HashMap::new(), |mut acc, sequence_map| {
            for (sequence, price) in sequence_map {
                acc.entry(sequence)
                    .and_modify(|acc_price| *acc_price += price)
                    .or_insert(price);
            }
            acc
        });
    Ok(buyers_sequences
        .into_iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .expect("Found result")
        .1)
}

fn read_input(file: &str) -> Result<Vec<u64>> {
    let data = read_to_string(file)?;
    data.split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.parse()
                .map_err(|err| Error::new(ErrorKind::InvalidData, err))
        })
        .collect()
}

fn calculate_prices_and_changes(mut secret_number: u64) -> Vec<(u8, i8)> {
    let mut result = Vec::new();
    for _ in 0..2000 {
        let new_secret_number = calculate_next_secret_number(secret_number);
        let price = new_secret_number % 10;
        let diff = price as i8 - (secret_number % 10) as i8;
        result.push((price as u8, diff));
        secret_number = new_secret_number
    }
    result
}

fn calculate_next_secret_number(mut secret_number: u64) -> u64 {
    secret_number = (secret_number ^ (secret_number << 6)) % 16777216;
    secret_number = (secret_number ^ (secret_number >> 5)) % 16777216;
    secret_number = (secret_number ^ (secret_number << 11)) % 16777216;
    secret_number
}
