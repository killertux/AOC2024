use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> std::io::Result<()> {
    println!("Part 1 : {}", part1("input.txt")?);
    println!("Part 2 : {}", part2("input.txt")?);
    Ok(())
}

fn part1(file: &str) -> std::io::Result<i64> {
    let buf_read = BufReader::new(File::open(file)?);
    let lines = buf_read
        .lines()
        .map(|line| Ok(line?.chars().collect()))
        .collect::<Result<Vec<Vec<char>>, std::io::Error>>()?;
    let mut n_xmas = 0;
    for y in 0..lines.len() {
        for x in 0..lines[0].len() {
            n_xmas += check_horizontal(y, x, &lines);
            n_xmas += check_vertical(y, x, &lines);
            n_xmas += check_diagonal(y, x, &lines);
        }
    }
    Ok(n_xmas)
}

fn part2(file: &str) -> std::io::Result<i64> {
    let buf_read = BufReader::new(File::open(file)?);
    let lines = buf_read
        .lines()
        .map(|line| Ok(line?.chars().collect()))
        .collect::<Result<Vec<Vec<char>>, std::io::Error>>()?;
    let mut n_xmas = 0;
    for y in 1..(lines.len() - 1) {
        for x in 1..(lines[0].len() - 1) {
            n_xmas += check_x_mas(y, x, &lines);
        }
    }
    Ok(n_xmas)
}

fn check_horizontal(y: usize, x: usize, lines: &[Vec<char>]) -> i64 {
    let forward: String = lines[y][x..(x + 4).min(lines[y].len())].iter().collect();
    if is_xmas(&forward) {
        return 1;
    }
    0
}

fn check_vertical(y: usize, x: usize, lines: &[Vec<char>]) -> i64 {
    let forward: String = lines.iter().skip(y).take(4).map(|line| line[x]).collect();
    if is_xmas(&forward) {
        return 1;
    }
    0
}

fn check_diagonal(y: usize, x: usize, lines: &[Vec<char>]) -> i64 {
    let mut n_finds = 0;
    let diagonal_1: String = lines
        .iter()
        .enumerate()
        .skip(y)
        .take_while(|(i, _)| *i < (y + 4) && x + (i - y) < lines[y].len())
        .map(|(i, line)| line[x + (i - y)])
        .collect();
    let diagonal_2: String = lines
        .iter()
        .enumerate()
        .skip(y)
        .take_while(|(i, _)| *i < y + 4 && (x as i64) - (i - y) as i64 >= 0)
        .map(|(i, line)| line[x - (i - y)])
        .collect();
    if is_xmas(&diagonal_1) {
        n_finds += 1;
    }
    if is_xmas(&diagonal_2) {
        n_finds += 1;
    }
    n_finds
}

fn check_x_mas(y: usize, x: usize, lines: &[Vec<char>]) -> i64 {
    let diagonal_1: String = lines
        .iter()
        .skip(y - 1)
        .take(3)
        .enumerate()
        .map(|(i, line)| line[(x - 1) + i])
        .collect();
    let diagonal_2: String = lines
        .iter()
        .skip(y - 1)
        .take(3)
        .enumerate()
        .map(|(i, line)| line[(x + 1) - i])
        .collect();
    if is_mas(&diagonal_1) && is_mas(&diagonal_2) {
        return 1;
    }
    0
}

fn is_xmas(text: &str) -> bool {
    text == "XMAS" || text == "SAMX"
}

fn is_mas(text: &str) -> bool {
    text == "MAS" || text == "SAM"
}
