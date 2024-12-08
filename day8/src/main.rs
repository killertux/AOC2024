use std::collections::{HashMap, HashSet};
use std::fs::read_to_string;
use std::io::{Error, ErrorKind, Result};
use std::ops::Div;

fn main() -> Result<()> {
    println!("Part 1 : {}", part1("input.txt")?);
    println!("Part 2 : {}", part2("input.txt")?);
    Ok(())
}

fn part1(file: &str) -> Result<usize> {
    let set_of_antinodes = process_and_return_list_of_nodes(file, |x, y, antenna_1, antenna_2| {
        let y_diff_1 = y as i32 - antenna_1.y as i32;
        let x_diff_1 = x as i32 - antenna_1.x as i32;
        let y_diff_2 = y as i32 - antenna_2.y as i32;
        let x_diff_2 = x as i32 - antenna_2.x as i32;
        if (y_diff_1 as f32).div(x_diff_1 as f32) != (y_diff_2 as f32).div(x_diff_2 as f32) {
            return false;
        }
        let dist_1 = x_diff_1.pow(2) + y_diff_1.pow(2);
        let dist_2 = x_diff_2.pow(2) + y_diff_2.pow(2);
        if dist_1 * 4 == dist_2 || dist_2 * 4 == dist_1 {
            return true;
        }
        false
    })?;

    Ok(set_of_antinodes.len())
}

fn part2(file: &str) -> Result<usize> {
    let set_of_antinodes = process_and_return_list_of_nodes(file, |x, y, antenna_1, antenna_2| {
        let y_diff_1 = y as i32 - antenna_1.y as i32;
        let x_diff_1 = x as i32 - antenna_1.x as i32;
        let y_diff_2 = y as i32 - antenna_2.y as i32;
        let x_diff_2 = x as i32 - antenna_2.x as i32;
        if (y_diff_1 == 0 && x_diff_1 == 0) || (y_diff_2 == 0 && x_diff_2 == 0) {
            return true;
        }
        if (y_diff_1 as f32).div(x_diff_1 as f32) != (y_diff_2 as f32).div(x_diff_2 as f32) {
            return false;
        }
        true
    })?;

    Ok(set_of_antinodes.len())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Antenna {
    x: usize,
    y: usize,
    frequency: char,
}

fn process_and_return_list_of_nodes(
    file: &str,
    checker: impl Fn(usize, usize, &Antenna, &Antenna) -> bool,
) -> Result<HashSet<(usize, usize)>> {
    let input = read_to_string(file)?;
    let y_size = input.chars().filter(|c| *c == '\n').count();
    let x_size = input
        .split_once('\n')
        .map(|(first_line, _)| first_line.chars().count())
        .ok_or(Error::new(ErrorKind::InvalidData, "Error getting map size"))?;
    let mut map_of_frequencies = HashMap::new();
    for (y, line) in input.split('\n').enumerate() {
        for (x, freq) in line.chars().enumerate().filter(|(_, c)| *c != '.') {
            map_of_frequencies
                .entry(freq)
                .or_insert(Vec::new())
                .push(Antenna {
                    x,
                    y,
                    frequency: freq,
                });
        }
    }
    let mut set_of_antinodes = HashSet::new();
    for y in 0..y_size {
        for x in 0..x_size {
            for (_, antennas) in map_of_frequencies.iter() {
                for (i, antenna_1) in antennas.iter().enumerate() {
                    for antenna_2 in antennas.iter().skip(i + 1) {
                        if antenna_1 == antenna_2 {
                            continue;
                        }
                        if checker(x, y, antenna_1, antenna_2) {
                            set_of_antinodes.insert((x, y));
                        }
                    }
                }
            }
        }
    }
    Ok(set_of_antinodes)
}
