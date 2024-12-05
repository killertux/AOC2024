use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind, Result},
};

fn main() -> std::io::Result<()> {
    println!("Part 1 : {}", part1("input.txt")?);
    println!("Part 2 : {}", part2("input.txt")?);
    Ok(())
}

fn part1(file: &str) -> std::io::Result<i64> {
    let (rules, updates) = load_rules_and_updates(file)?;
    let (correct_updates, _) = split_correct_and_incorrect_updates(updates, &rules);

    Ok(sum_middle_pages(correct_updates))
}

fn part2(file: &str) -> std::io::Result<i64> {
    let (rules, updates) = load_rules_and_updates(file)?;
    let (_, incorrect_updates) = split_correct_and_incorrect_updates(updates, &rules);
    let fixed_updates = fix_updates(incorrect_updates, &rules);

    Ok(sum_middle_pages(fixed_updates))
}

fn load_rules_and_updates(file: &str) -> Result<(HashMap<i64, Vec<i64>>, Vec<Vec<i64>>)> {
    let buf_read = BufReader::new(File::open(file)?);
    let (rules, updates): (Vec<_>, Vec<_>) = buf_read
        .lines()
        .filter(|line| line.as_ref().map(|line| !line.is_empty()).unwrap_or(false))
        .partition(|line| {
            line.as_ref()
                .map(|line| line.contains('|'))
                .unwrap_or(false)
        });
    let rules: HashMap<i64, Vec<i64>> = rules
        .into_iter()
        .map(|rule| {
            let rule = rule?;
            let Some((part1, part2)) = rule.split_once('|') else {
                return Err(Error::new(ErrorKind::InvalidData, ""));
            };
            Ok((
                part1
                    .parse()
                    .map_err(|err| Error::new(ErrorKind::InvalidData, err))?,
                part2
                    .parse()
                    .map_err(|err| Error::new(ErrorKind::InvalidData, err))?,
            ))
        })
        .collect::<Result<Vec<(i64, i64)>>>()?
        .into_iter()
        .fold(HashMap::new(), |mut acc, (key, value)| {
            acc.entry(key).or_default().push(value);
            acc
        });
    let updates = updates
        .into_iter()
        .map(|line| {
            Ok(line?
                .split(',')
                .map(|split| {
                    Ok(split
                        .parse()
                        .map_err(|err| Error::new(ErrorKind::InvalidData, err))?)
                })
                .collect::<Result<Vec<i64>>>()?)
        })
        .collect::<Result<Vec<Vec<i64>>>>()?;
    Ok((rules, updates))
}

fn split_correct_and_incorrect_updates(
    updates: Vec<Vec<i64>>,
    rules: &HashMap<i64, Vec<i64>>,
) -> (Vec<Vec<i64>>, Vec<Vec<i64>>) {
    updates.into_iter().partition(|update| {
        let mut is_valid = true;
        for i in 0..update.len() {
            let element = update[i];
            let Some(rules_for_element) = rules.get(&element) else {
                continue;
            };
            for rule in rules_for_element {
                if update[0..i].contains(rule) {
                    is_valid = false;
                    break;
                }
            }
            if !is_valid {
                break;
            }
        }
        is_valid
    })
}

fn fix_updates(incorrect_updates: Vec<Vec<i64>>, rules: &HashMap<i64, Vec<i64>>) -> Vec<Vec<i64>> {
    let mut fixed_updates = Vec::new();
    for mut update in incorrect_updates {
        loop {
            let mut is_valid = true;
            for i in 0..update.len() {
                let element = update[i];
                let Some(rules_for_element) = rules.get(&element) else {
                    continue;
                };
                for rule in rules_for_element {
                    if let Some(pos) = update[0..i].iter().position(|n| n == rule) {
                        let wrong_element = update.remove(pos);
                        update.insert(i, wrong_element);
                        is_valid = false;
                        break;
                    }
                }
                if !is_valid {
                    break;
                }
            }
            if !is_valid {
                continue;
            }
            fixed_updates.push(update);
            break;
        }
    }
    fixed_updates
}

fn sum_middle_pages(updates: Vec<Vec<i64>>) -> i64 {
    updates
        .into_iter()
        .map(|update| update[update.len() / 2])
        .sum()
}
