use std::{
    collections::HashSet,
    fs::read_to_string,
    io::{Error, ErrorKind, Result},
};

fn main() -> Result<()> {
    println!("Part 1 example: {}", part1("example.txt")?);
    println!("Part 1 result: {}", part1("input.txt")?);
    println!("Part 2 example: {}", part2("example.txt")?);
    println!("Part 2 result: {}", part2("input.txt")?);
    Ok(())
}

fn part1(file: &str) -> Result<i32> {
    let data = read_map(file)?;
    let mut trail_heads = 0;
    for y in 0..data.len() {
        for x in 0..data[0].len() {
            if data[y][x] == 0 {
                trail_heads += calc_trail(y, x, &data).len() as i32;
            }
        }
    }
    Ok(trail_heads)
}

fn part2(file: &str) -> Result<i32> {
    let data = read_map(file)?;
    let mut trail_heads = 0;
    for y in 0..data.len() {
        for x in 0..data[0].len() {
            if data[y][x] == 0 {
                trail_heads += calc_trail2(y, x, &data);
            }
        }
    }
    Ok(trail_heads)
}

fn calc_trail(y: usize, x: usize, data: &[Vec<i32>]) -> HashSet<(usize, usize)> {
    let current = data[y][x];
    if current == 9 {
        return HashSet::from([(y, x)]);
    }
    let mut finished_trails = HashSet::new();
    if y > 0 && data[y - 1][x] == current + 1 {
        finished_trails.extend(calc_trail(y - 1, x, data));
    }
    if y < (data.len() - 1) && data[y + 1][x] == current + 1 {
        finished_trails.extend(calc_trail(y + 1, x, data));
    }
    if x > 0 && data[y][x - 1] == current + 1 {
        finished_trails.extend(calc_trail(y, x - 1, data));
    }
    if x < data[y].len() - 1 && data[y][x + 1] == current + 1 {
        finished_trails.extend(calc_trail(y, x + 1, data));
    }
    finished_trails
}

fn calc_trail2(y: usize, x: usize, data: &[Vec<i32>]) -> i32 {
    let current = data[y][x];
    if current == 9 {
        return 1;
    }
    let mut finished_trails = 0;
    if y > 0 && data[y - 1][x] == current + 1 {
        finished_trails += calc_trail2(y - 1, x, data);
    }
    if y < (data.len() - 1) && data[y + 1][x] == current + 1 {
        finished_trails += calc_trail2(y + 1, x, data);
    }
    if x > 0 && data[y][x - 1] == current + 1 {
        finished_trails += calc_trail2(y, x - 1, data);
    }
    if x < data[y].len() - 1 && data[y][x + 1] == current + 1 {
        finished_trails += calc_trail2(y, x + 1, data);
    }
    finished_trails
}

fn read_map(file: &str) -> Result<Vec<Vec<i32>>> {
    read_to_string(file)?
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| {
                    Ok(if c == '.' {
                        -1
                    } else {
                        c.to_digit(10)
                            .ok_or(Error::new(ErrorKind::InvalidData, "Not a digit"))?
                            as i32
                    })
                })
                .collect::<Result<Vec<i32>>>()
        })
        .collect::<Result<Vec<Vec<i32>>>>()
}
