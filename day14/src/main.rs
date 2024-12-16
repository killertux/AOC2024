use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind, Result},
};

fn main() -> Result<()> {
    println!("Part 1 example: {}", part_1("example.txt", (11, 7))?);
    println!("Part 1 result: {}", part_1("input.txt", (101, 103))?);
    // println!("Part 1 example: {}", part_2("example.txt", (11, 7))?);
    println!("Part 1 result: {}", part_2("input.txt", (101, 103))?);
    Ok(())
}

fn part_1(file: &str, map_size: (usize, usize)) -> Result<i64> {
    let mut robots = read_robots(file)?;
    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.walk(&map_size);
        }
    }
    Ok(calculate_robots_per_quadrant(&map_size, &robots)
        .iter()
        .product())
}

fn part_2(file: &str, map_size: (usize, usize)) -> Result<i64> {
    let mut robots = read_robots(file)?;

    let n = find_iteration_with_highest_quadrant_density(robots.clone(), &map_size, 10000) + 1;
    println!("Chosen iteration {n}");
    let mut map: Vec<Vec<char>> = (0..map_size.1)
        .map(|_| (0..map_size.0).map(|_| ' ').collect())
        .collect();
    for _ in 0..n {
        for robot in robots.iter_mut() {
            map[robot.position.1 as usize][robot.position.0 as usize] = ' ';
            robot.walk(&map_size);
            map[robot.position.1 as usize][robot.position.0 as usize] = '█';
        }
    }
    draw_map(&map);

    Ok(0)
}

fn read_robots(file: &str) -> Result<Vec<Robot>> {
    let buf_read = BufReader::new(File::open(file)?);
    let mut lines = buf_read.lines();
    let mut robots = Vec::new();
    loop {
        let Some(line) = lines.next() else {
            break;
        };
        let line = line?;
        let remainder = line.strip_prefix("p=").ok_or_invalid_data()?;
        let (part1, remainder) = remainder.split_once(' ').ok_or_invalid_data()?;
        let (pos_x, pos_y) = part1.split_once(',').ok_or_invalid_data()?;
        let pos_x: i64 = pos_x
            .parse()
            .map_err(|err| Error::new(ErrorKind::InvalidData, err))?;
        let pos_y: i64 = pos_y
            .parse()
            .map_err(|err| Error::new(ErrorKind::InvalidData, err))?;
        let remainder = remainder.strip_prefix("v=").ok_or_invalid_data()?;
        let (vel_x, vel_y) = remainder.split_once(',').ok_or_invalid_data()?;
        let vel_x: i64 = vel_x
            .parse()
            .map_err(|err| Error::new(ErrorKind::InvalidData, err))?;
        let vel_y: i64 = vel_y
            .parse()
            .map_err(|err| Error::new(ErrorKind::InvalidData, err))?;
        robots.push(Robot {
            position: (pos_x, pos_y),
            velocity: (vel_x, vel_y),
        });
    }
    Ok(robots)
}

fn calculate_robots_per_quadrant(map_size: &(usize, usize), robots: &[Robot]) -> Vec<i64> {
    let quadrant_1 = (0..(map_size.0 / 2), 0..map_size.1 / 2);
    let quadrant_2 = (((map_size.0 / 2) + 1)..map_size.0, 0..map_size.1 / 2);
    let quadrant_3 = (0..(map_size.0 / 2), ((map_size.1 / 2) + 1)..map_size.1);
    let quadrant_4 = (
        ((map_size.0 / 2) + 1)..map_size.0,
        ((map_size.1 / 2) + 1)..map_size.1,
    );
    let mut n_quadrant_1 = 0;
    let mut n_quadrant_2 = 0;
    let mut n_quadrant_3 = 0;
    let mut n_quadrant_4 = 0;
    for robot in robots {
        if quadrant_1.0.contains(&(robot.position.0 as usize))
            && quadrant_1.1.contains(&(robot.position.1 as usize))
        {
            n_quadrant_1 += 1;
            continue;
        }
        if quadrant_2.0.contains(&(robot.position.0 as usize))
            && quadrant_2.1.contains(&(robot.position.1 as usize))
        {
            n_quadrant_2 += 1;
            continue;
        }
        if quadrant_3.0.contains(&(robot.position.0 as usize))
            && quadrant_3.1.contains(&(robot.position.1 as usize))
        {
            n_quadrant_3 += 1;
            continue;
        }
        if quadrant_4.0.contains(&(robot.position.0 as usize))
            && quadrant_4.1.contains(&(robot.position.1 as usize))
        {
            n_quadrant_4 += 1;
            continue;
        }
    }
    vec![n_quadrant_1, n_quadrant_2, n_quadrant_3, n_quadrant_4]
}

fn find_iteration_with_highest_quadrant_density(
    mut robots: Vec<Robot>,
    map_size: &(usize, usize),
    limit: usize,
) -> usize {
    let mut highest_quadrant_density = 0;
    let mut chosen_iteration = 0;
    for i in 0..limit {
        for robot in robots.iter_mut() {
            robot.walk(map_size);
        }
        let mut quadrants = calculate_robots_per_quadrant(map_size, &robots);
        quadrants.sort();
        if quadrants[3] > highest_quadrant_density {
            highest_quadrant_density = quadrants[3];
            chosen_iteration = i;
        }
    }
    chosen_iteration
}

#[derive(Debug, Clone)]
struct Robot {
    position: (i64, i64),
    velocity: (i64, i64),
}

impl Robot {
    pub fn walk(&mut self, map_size: &(usize, usize)) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        if self.position.0 < 0 {
            self.position.0 += map_size.0 as i64;
        }
        if self.position.1 < 0 {
            self.position.1 += map_size.1 as i64;
        }
        if self.position.0 > (map_size.0 - 1) as i64 {
            self.position.0 %= map_size.0 as i64;
        }
        if self.position.1 > (map_size.1 - 1) as i64 {
            self.position.1 %= map_size.1 as i64;
        }
    }
}

fn draw_map(map: &[Vec<char>]) {
    for row in map {
        for column in row {
            print!("{}", column);
        }
        println!();
    }
}

trait OkOrInvalidData<T> {
    fn ok_or_invalid_data(self) -> Result<T>;
}

impl<T> OkOrInvalidData<T> for Option<T> {
    fn ok_or_invalid_data(self) -> Result<T> {
        self.ok_or(Error::new(ErrorKind::InvalidData, "Invalid data"))
    }
}
