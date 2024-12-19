use std::{
    collections::HashMap,
    fs::read_to_string,
    io::{Error, ErrorKind, Result},
};

fn main() -> Result<()> {
    println!("Part 1 example: {}", part_1("example.txt")?);
    println!("Part 1 result: {}", part_1("input.txt")?);
    println!("Part 2 example: {}", part_2("example.txt")?);
    println!("Part 2 result: {}", part_2("input.txt")?);
    Ok(())
}

fn part_1(file: &str) -> Result<usize> {
    let (available_towels, desired_patterns) = read_input(file)?;
    Ok(desired_patterns
        .iter()
        .filter(|pattern| is_possible(pattern, available_towels.clone()))
        .count())
}

fn part_2(file: &str) -> Result<usize> {
    let (available_towels, desired_patterns) = read_input(file)?;
    let mut memoization: HashMap<String, usize> = HashMap::new();
    Ok(desired_patterns
        .iter()
        .map(|pattern| n_possibilities(pattern, available_towels.clone(), &mut memoization))
        .sum())
}

fn is_possible(pattern: &str, available_towels: Vec<String>) -> bool {
    if pattern.is_empty() {
        return true;
    }
    let mut my_available_towels = available_towels.clone();
    while let Ok(position) = my_available_towels.binary_search_by(|towel| {
        towel
            .chars()
            .next()
            .unwrap()
            .cmp(&pattern.chars().next().unwrap())
    }) {
        let towel = my_available_towels.remove(position);
        if towel.len() > pattern.len() {
            continue;
        }
        if towel == pattern[0..towel.len()]
            && is_possible(&pattern[towel.len()..], available_towels.clone())
        {
            return true;
        }
    }
    false
}

fn n_possibilities(
    pattern: &str,
    available_towels: Vec<String>,
    memoization: &mut HashMap<String, usize>,
) -> usize {
    if pattern.is_empty() {
        return 1;
    }
    if let Some(n) = memoization.get(pattern) {
        return *n;
    }

    let mut my_available_towels = available_towels.clone();
    let mut count = 0;
    while let Ok(position) = my_available_towels.binary_search_by(|towel| {
        towel
            .chars()
            .next()
            .unwrap()
            .cmp(&pattern.chars().next().unwrap())
    }) {
        let towel = my_available_towels.remove(position);
        if towel.len() > pattern.len() {
            continue;
        }
        if towel == pattern[0..towel.len()] {
            count += n_possibilities(
                &pattern[towel.len()..],
                available_towels.clone(),
                memoization,
            )
        }
    }
    memoization.insert(pattern.to_string(), count);
    count
}

fn read_input(file: &str) -> Result<(Vec<String>, Vec<String>)> {
    let data = read_to_string(file)?;
    let (available_towels, desired_patterns) = data.split_once("\n\n").ok_or_invalid_data()?;
    let mut available_towels: Vec<String> = available_towels
        .split(',')
        .map(|towel| towel.trim().to_string())
        .collect();
    available_towels.sort();
    Ok((
        available_towels,
        desired_patterns
            .split('\n')
            .filter(|pattern| !pattern.is_empty())
            .map(|pattern| pattern.trim().to_string())
            .collect(),
    ))
}

trait OkOrInvalidData<T> {
    fn ok_or_invalid_data(self) -> Result<T>;
}

impl<T> OkOrInvalidData<T> for Option<T> {
    fn ok_or_invalid_data(self) -> Result<T> {
        self.ok_or(Error::new(ErrorKind::InvalidData, "Invalid data"))
    }
}
