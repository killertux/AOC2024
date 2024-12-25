use std::{
    fs::read_to_string,
    io::{Error, Result},
};

fn main() -> Result<()> {
    println!("Part 1 example: {}", part_1("example.txt")?);
    println!("Part 1 input: {}", part_1("input.txt")?);
    // println!("Part 2 input: {}", part_2("input.txt")?);
    Ok(())
}

fn part_1(file: &str) -> Result<usize> {
    let (locks, keys) = read_input(file)?;
    let mut matches = 0;
    for lock in locks.iter() {
        for key in keys.iter() {
            if lock.iter().zip(key.iter()).all(|(l, k)| l + k <= 5) {
                matches += 1;
            }
        }
    }
    Ok(matches)
}

fn read_input(file: &str) -> Result<(Vec<Vec<usize>>, Vec<Vec<usize>>)> {
    let data = read_to_string(file)?;
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    data.split("\n\n")
        .filter(|part| !part.is_empty())
        .for_each(|part| {
            let lines = part.split('\n').collect::<Vec<&str>>();
            if lines[0] == "#####" {
                locks.push(
                    (0..5)
                        .map(|column| {
                            (1usize..7)
                                .filter(|line| lines[*line].chars().nth(column) == Some('#'))
                                .count()
                        })
                        .collect(),
                )
            } else {
                keys.push(
                    (0..5)
                        .map(|column| {
                            (0usize..6)
                                .filter(|line| lines[*line].chars().nth(column) == Some('#'))
                                .count()
                        })
                        .collect(),
                )
            }
        });
    Ok((locks, keys))
}

trait OkOrInvalidData<T> {
    fn ok_or_invalid_data(self) -> Result<T>;
}

impl<T> OkOrInvalidData<T> for Option<T> {
    fn ok_or_invalid_data(self) -> Result<T> {
        self.ok_or(Error::new(std::io::ErrorKind::InvalidData, "Invalid data"))
    }
}
