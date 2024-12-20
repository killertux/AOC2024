use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    io::Result,
};

fn main() -> Result<()> {
    println!("Part 1 example: {}", part_1("example.txt")?);
    println!("Part 1 result: {}", part_1("input.txt")?);
    println!("Part 2 example: {}", part_2("example.txt", 50)?);
    println!("Part 2 result: {}", part_2("input.txt", 100)?);
    Ok(())
}

fn part_1(file: &str) -> Result<usize> {
    let (walls, start, _end) = read_input(file)?;
    let time_matrix = calculate_time_of_race(&walls, &start);
    let cheats = calculate_possible_cheats(&time_matrix, &walls);
    Ok(cheats.into_iter().filter(|cheat| *cheat >= 100).count())
}

fn part_2(file: &str, limit: u64) -> Result<usize> {
    let (walls, start, _end) = read_input(file)?;
    let time_matrix = calculate_time_of_race(&walls, &start);
    let cheats = calculate_possible_cheats_2(&time_matrix);
    Ok(cheats.into_iter().filter(|cheat| *cheat >= limit).count())
}

fn calculate_time_of_race(
    walls: &HashSet<(usize, usize)>,
    start: &(usize, usize),
) -> HashMap<(usize, usize), u64> {
    let mut cost_matrix = HashMap::new();
    let mut nodes = vec![(start.clone(), 0)];

    while let Some((node, cost)) = nodes.pop() {
        if cost_matrix.contains_key(&node) {
            continue;
        }
        cost_matrix.insert(node.clone(), cost);

        if !walls.contains(&(node.0 - 1, node.1)) {
            nodes.push(((node.0 - 1, node.1), cost + 1));
        }
        if !walls.contains(&(node.0 + 1, node.1)) {
            nodes.push(((node.0 + 1, node.1), cost + 1));
        }
        if !walls.contains(&(node.0, node.1 - 1)) {
            nodes.push(((node.0, node.1 - 1), cost + 1));
        }
        if !walls.contains(&(node.0, node.1 + 1)) {
            nodes.push(((node.0, node.1 + 1), cost + 1));
        }
    }
    cost_matrix
}

fn calculate_possible_cheats(
    time_matrix: &HashMap<(usize, usize), u64>,
    walls: &HashSet<(usize, usize)>,
) -> Vec<u64> {
    let mut cheats = Vec::new();
    for wall in walls.iter() {
        if wall.0 == 0 || wall.1 == 0 {
            continue;
        }
        if let Some(cheat) = check_vertical_cheat(wall, time_matrix, walls) {
            cheats.push(cheat);
        }
        if let Some(cheat) = check_horizontal_cheat(wall, time_matrix, walls) {
            cheats.push(cheat);
        }
    }
    cheats
}

fn check_vertical_cheat(
    wall: &(usize, usize),
    time_matrix: &HashMap<(usize, usize), u64>,
    walls: &HashSet<(usize, usize)>,
) -> Option<u64> {
    if !walls.contains(&(wall.0 - 1, wall.1)) && !walls.contains(&(wall.0 + 1, wall.1)) {
        let Some(cost_1) = time_matrix.get(&(wall.0 - 1, wall.1)) else {
            return None;
        };
        let Some(cost_2) = time_matrix.get(&(wall.0 + 1, wall.1)) else {
            return None;
        };
        return Some(cost_1.max(cost_2) - cost_1.min(cost_2) - 2);
    }
    None
}

fn check_horizontal_cheat(
    wall: &(usize, usize),
    time_matrix: &HashMap<(usize, usize), u64>,
    walls: &HashSet<(usize, usize)>,
) -> Option<u64> {
    if !walls.contains(&(wall.0, wall.1 - 1)) && !walls.contains(&(wall.0, wall.1 + 1)) {
        let Some(cost_1) = time_matrix.get(&(wall.0, wall.1 - 1)) else {
            return None;
        };
        let Some(cost_2) = time_matrix.get(&(wall.0, wall.1 + 1)) else {
            return None;
        };
        return Some(cost_1.max(cost_2) - cost_1.min(cost_2) - 2);
    }
    None
}

fn calculate_possible_cheats_2(time_matrix: &HashMap<(usize, usize), u64>) -> Vec<u64> {
    let mut cheats: HashMap<((usize, usize), (usize, usize)), u64> = HashMap::new();
    for (position, cost) in time_matrix.iter() {
        for y in position.0.saturating_sub(20)..=position.0 + 20 {
            for x in position.1.saturating_sub(20)..=position.1 + 20 {
                let distance = calculate_distance(position, &(y, x));
                if distance > 20 {
                    continue;
                }
                if let Some(t_cost) = time_matrix.get(&(y, x)) {
                    if cost + distance < *t_cost {
                        cheats.insert((position.clone(), (y, x)), t_cost - cost - distance);
                    }
                }
            }
        }
    }
    cheats.values().cloned().collect()
}

fn calculate_distance(point_1: &(usize, usize), point_2: &(usize, usize)) -> u64 {
    let y_diff = if point_1.0 > point_2.0 {
        point_1.0 - point_2.0
    } else {
        point_2.0 - point_1.0
    };
    let x_diff = if point_1.1 > point_2.1 {
        point_1.1 - point_2.1
    } else {
        point_2.1 - point_1.1
    };
    (y_diff + x_diff) as u64
}

fn read_input(file: &str) -> Result<(HashSet<(usize, usize)>, (usize, usize), (usize, usize))> {
    let data = read_to_string(file)?;
    let mut start = None;
    let mut end = None;
    let mut walls = HashSet::new();
    for (y, row) in data.split('\n').enumerate().filter(|r| !r.1.is_empty()) {
        for (x, tile) in row.chars().enumerate() {
            if tile == '#' {
                walls.insert((y, x));
                continue;
            }
            if tile == 'S' {
                start = Some((y, x));
                continue;
            }
            if tile == 'E' {
                end = Some((y, x));
                continue;
            }
        }
    }
    Ok((
        walls,
        start.expect("Not found start"),
        end.expect("Not found end"),
    ))
}
