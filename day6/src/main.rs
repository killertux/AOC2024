use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

fn main() -> std::io::Result<()> {
    println!("Part 1 : {}", part1("input.txt")?);
    println!("Part 2 : {}", part2("input.txt")?);
    Ok(())
}

fn part1(file: &str) -> std::io::Result<i64> {
    let buf_read = BufReader::new(File::open(file)?);
    let map = buf_read
        .lines()
        .map(|line| Ok(line?.chars().collect()))
        .collect::<Result<Vec<Vec<char>>>>()?;
    let guard = Guard::from_map(&map).ok_or(Error::new(ErrorKind::InvalidData, "Missing guard"))?;

    Ok(walk_and_return_postions(guard, &map).len() as i64)
}

fn part2(file: &str) -> std::io::Result<i64> {
    let buf_read = BufReader::new(File::open(file)?);
    let mut map = buf_read
        .lines()
        .map(|line| Ok(line?.chars().collect()))
        .collect::<Result<Vec<Vec<char>>>>()?;

    let initial_guard =
        Guard::from_map(&map).ok_or(Error::new(ErrorKind::InvalidData, "Missing guard"))?;
    let mut guard_path = walk_and_return_postions(initial_guard.clone(), &map);
    guard_path.remove(&(initial_guard.y, initial_guard.x));
    let mut found_loops = 0;
    for (y, x) in guard_path {
        map[y as usize][x as usize] = '#';
        if do_we_have_a_loop(initial_guard.clone(), &map) {
            found_loops += 1;
        }
        map[y as usize][x as usize] = '.';
    }

    Ok(found_loops)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Guard {
    x: isize,
    y: isize,
    direction: Direction,
}

impl Guard {
    pub fn from_map(map: &[Vec<char>]) -> Option<Self> {
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                if map[y][x] == '^' {
                    return Some(Guard {
                        x: x as isize,
                        y: y as isize,
                        direction: Direction::Up,
                    });
                }
            }
        }
        None
    }

    pub fn is_inside_map(&self, map: &[Vec<char>]) -> bool {
        if self.y < 0 {
            return false;
        }
        if self.y >= map.len() as isize {
            return false;
        }
        if self.x < 0 {
            return false;
        }
        if self.x >= map[0].len() as isize {
            return false;
        }
        true
    }

    pub fn walk(&mut self, map: &[Vec<char>]) {
        match self.direction {
            Direction::Up => {
                if self.y != 0 {
                    if map[self.y as usize - 1][self.x as usize] == '#' {
                        self.direction = Direction::Right;
                        return self.walk(map);
                    }
                }
                self.y -= 1;
            }
            Direction::Down => {
                if self.y as usize != map.len() - 1 {
                    if map[self.y as usize + 1][self.x as usize] == '#' {
                        self.direction = Direction::Left;
                        return self.walk(map);
                    }
                }
                self.y += 1;
            }
            Direction::Right => {
                if self.x as usize != map[0].len() - 1 {
                    if map[self.y as usize][self.x as usize + 1] == '#' {
                        self.direction = Direction::Down;
                        return self.walk(map);
                    }
                }
                self.x += 1;
            }
            Direction::Left => {
                if self.x != 0 {
                    if map[self.y as usize][self.x as usize - 1] == '#' {
                        self.direction = Direction::Up;
                        return self.walk(map);
                    }
                }
                self.x -= 1;
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn walk_and_return_postions(mut guard: Guard, map: &[Vec<char>]) -> HashSet<(isize, isize)> {
    let mut list_of_guard_positions: HashSet<(isize, isize)> = HashSet::new();
    while guard.is_inside_map(&map) {
        list_of_guard_positions.insert((guard.y, guard.x));
        guard.walk(&map);
    }
    list_of_guard_positions
}

fn do_we_have_a_loop(mut guard: Guard, map: &[Vec<char>]) -> bool {
    let mut list_of_guard_positions: HashSet<Guard> = HashSet::new();
    let mut loop_found = false;
    while guard.is_inside_map(&map) {
        if !list_of_guard_positions.insert(guard.clone()) {
            loop_found = true;
            break;
        }
        guard.walk(&map);
    }
    loop_found
}
