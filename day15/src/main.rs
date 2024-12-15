use std::{
    fs::read_to_string,
    io::{Error, ErrorKind, Result},
    mem,
};

fn main() -> Result<()> {
    println!("Part 1 example: {}", part_1("example.txt")?);
    println!("Part 1 result: {}", part_1("input.txt")?);
    println!("Part 1 example: {}", part_2("example.txt")?);
    println!("Part 1 result: {}", part_2("input.txt")?);
    Ok(())
}

fn part_1(file: &str) -> Result<usize> {
    let (mut map, movements) = read_input(file)?;
    let mut robot_position = find_robot(&map);
    for movement in movements {
        if let Some(position) = try_to_move(&robot_position, movement, &mut map) {
            robot_position = position;
        }
    }
    print_map(&map);
    Ok(sum_all_boxes_coordinates(&map))
}

fn part_2(file: &str) -> Result<usize> {
    let (mut map, movements) = read_input_2(file)?;
    let mut robot_position = find_robot(&map);
    for movement in movements {
        if can_move(&robot_position, &movement, &mut map) {
            robot_position = do_move(&robot_position, &movement, &mut map);
        }
    }
    print_map(&map);
    Ok(sum_all_boxes_coordinates(&map))
}

fn read_input(file: &str) -> Result<(Vec<Vec<Tile>>, Vec<Movement>)> {
    let data = read_to_string(file)?;
    let (map, movements) = data.split_once("\n\n").ok_or_invalid_data()?;
    Ok((
        map.split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().map(Tile::from_char).collect())
            .collect(),
        movements
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(Movement::from_char)
            .collect(),
    ))
}

fn read_input_2(file: &str) -> Result<(Vec<Vec<Tile>>, Vec<Movement>)> {
    let data = read_to_string(file)?;
    let (map, movements) = data.split_once("\n\n").ok_or_invalid_data()?;
    Ok((
        map.split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.chars()
                    .flat_map(|c| match c {
                        '#' => [Tile::from_char('#'), Tile::from_char('#')],
                        '@' => [Tile::from_char('@'), Tile::from_char('.')],
                        '.' => [Tile::from_char('.'), Tile::from_char('.')],
                        'O' => [Tile::from_char('['), Tile::from_char(']')],
                        _ => panic!("Non expected char in map"),
                    })
                    .collect()
            })
            .collect(),
        movements
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(Movement::from_char)
            .collect(),
    ))
}

fn find_robot(map: &[Vec<Tile>]) -> (usize, usize) {
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == Tile::Robot {
                return (y, x);
            }
        }
    }
    panic!("Robot not found")
}

fn try_to_move(
    position: &(usize, usize),
    movement: Movement,
    map: &mut [Vec<Tile>],
) -> Option<(usize, usize)> {
    if map[position.0][position.1] == Tile::Wall {
        return None;
    }
    let desired_new_position = match movement {
        Movement::Up => (position.0 - 1, position.1),
        Movement::Down => (position.0 + 1, position.1),
        Movement::Left => (position.0, position.1 - 1),
        Movement::Right => (position.0, position.1 + 1),
    };
    if map[desired_new_position.0][desired_new_position.1] == Tile::Empty {
        let tile = mem::take(&mut map[position.0][position.1]);
        map[desired_new_position.0][desired_new_position.1] = tile;
        return Some(desired_new_position);
    }
    if try_to_move(&desired_new_position, movement, map).is_some() {
        let tile = mem::take(&mut map[position.0][position.1]);
        map[desired_new_position.0][desired_new_position.1] = tile;
        return Some(desired_new_position);
    }
    None
}

fn can_move(position: &(usize, usize), movement: &Movement, map: &mut [Vec<Tile>]) -> bool {
    let desired_new_position = match movement {
        Movement::Up => (position.0.saturating_sub(1), position.1),
        Movement::Down => (position.0 + 1, position.1),
        Movement::Left => (position.0, position.1.saturating_sub(1)),
        Movement::Right => (position.0, position.1 + 1),
    };
    match map[position.0][position.1] {
        Tile::Wall => false,
        Tile::Empty => true,
        Tile::Robot | Tile::Box => can_move(&desired_new_position, movement, map),
        Tile::WideBoxL if *movement == Movement::Up || *movement == Movement::Down => {
            can_move(&desired_new_position, movement, map)
                && can_move(
                    &(desired_new_position.0, desired_new_position.1 + 1),
                    movement,
                    map,
                )
        }
        Tile::WideBoxL => can_move(&desired_new_position, movement, map),
        Tile::WideBoxR if *movement == Movement::Up || *movement == Movement::Down => {
            can_move(&desired_new_position, movement, map)
                && can_move(
                    &(desired_new_position.0, desired_new_position.1 - 1),
                    movement,
                    map,
                )
        }
        Tile::WideBoxR => can_move(&desired_new_position, movement, map),
    }
}

fn do_move(
    position: &(usize, usize),
    movement: &Movement,
    map: &mut [Vec<Tile>],
) -> (usize, usize) {
    let desired_new_position = match movement {
        Movement::Up => (position.0 - 1, position.1),
        Movement::Down => (position.0 + 1, position.1),
        Movement::Left => (position.0, position.1 - 1),
        Movement::Right => (position.0, position.1 + 1),
    };
    match map[position.0][position.1] {
        Tile::Wall => panic!("You should not move a wall"),
        Tile::Empty => desired_new_position,
        Tile::Robot | Tile::Box => {
            do_move(&desired_new_position, movement, map);
            let tile = mem::take(&mut map[position.0][position.1]);
            map[desired_new_position.0][desired_new_position.1] = tile;
            desired_new_position
        }
        Tile::WideBoxL => {
            do_move(&desired_new_position, movement, map);
            if *movement == Movement::Up || *movement == Movement::Down {
                do_move(
                    &(desired_new_position.0, desired_new_position.1 + 1),
                    movement,
                    map,
                );
                let tile_2 = mem::take(&mut map[position.0][position.1 + 1]);
                map[desired_new_position.0][desired_new_position.1 + 1] = tile_2;
            }

            let tile_1 = mem::take(&mut map[position.0][position.1]);
            map[desired_new_position.0][desired_new_position.1] = tile_1;
            desired_new_position
        }
        Tile::WideBoxR => {
            do_move(&desired_new_position, movement, map);
            if *movement == Movement::Up || *movement == Movement::Down {
                do_move(
                    &(desired_new_position.0, desired_new_position.1 - 1),
                    movement,
                    map,
                );
                let tile_2 = mem::take(&mut map[position.0][position.1 - 1]);
                map[desired_new_position.0][desired_new_position.1 - 1] = tile_2;
            }

            let tile_1 = mem::take(&mut map[position.0][position.1]);
            map[desired_new_position.0][desired_new_position.1] = tile_1;
            desired_new_position
        }
    }
}

fn sum_all_boxes_coordinates(map: &[Vec<Tile>]) -> usize {
    let mut result = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            if matches!(tile, Tile::Box | Tile::WideBoxL) {
                result += y * 100 + x;
            }
        }
    }
    result
}

fn print_map(map: &[Vec<Tile>]) {
    for row in map {
        for tile in row {
            let c = match tile {
                Tile::Box => 'O',
                Tile::Empty => '.',
                Tile::Robot => '@',
                Tile::Wall => '#',
                Tile::WideBoxL => '[',
                Tile::WideBoxR => ']',
            };
            print!("{c}");
        }
        println!();
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Box,
    WideBoxL,
    WideBoxR,
    Robot,
}

impl Default for Tile {
    fn default() -> Self {
        Self::Empty
    }
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            'O' => Tile::Box,
            '@' => Tile::Robot,
            '[' => Tile::WideBoxL,
            ']' => Tile::WideBoxR,
            other => panic!("Invalid char {other}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Movement {
    Right,
    Left,
    Up,
    Down,
}

impl Movement {
    fn from_char(c: char) -> Self {
        match c {
            '<' => Self::Left,
            '>' => Self::Right,
            '^' => Self::Up,
            'v' => Self::Down,
            other => panic!("Invalid char {other}"),
        }
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
