use std::{fs::read_to_string, io::Result};

fn main() -> Result<()> {
    println!("Part 1 example: {}", part_1("example.txt")?);
    println!("Part 1 result: {}", part_1("input.txt")?);
    println!("Part 2 result: {}", part_2("example.txt")?);
    println!("Part 2 result: {}", part_2("input.txt")?);
    Ok(())
}

fn part_1(file: &str) -> Result<usize> {
    let map = read_map(file)?;
    let regions = map_into_regions(map.clone());
    let mut total_cost = 0;
    for region in regions {
        let mut perimeter = 0;
        for plant in &region.plants {
            if plant.0 > 0 && map[plant.0 - 1][plant.1] != region.plant_type || plant.0 == 0 {
                perimeter += 1;
            }
            if plant.0 < map.len() - 1 && map[plant.0 + 1][plant.1] != region.plant_type
                || plant.0 == map.len() - 1
            {
                perimeter += 1;
            }
            if plant.1 > 0 && map[plant.0][plant.1 - 1] != region.plant_type || plant.1 == 0 {
                perimeter += 1;
            }
            if plant.1 < map[plant.0].len() - 1 && map[plant.0][plant.1 + 1] != region.plant_type
                || plant.1 == map[plant.0].len() - 1
            {
                perimeter += 1;
            }
        }
        total_cost += perimeter * region.plants.len();
    }
    Ok(total_cost)
}

fn part_2(file: &str) -> Result<usize> {
    let map = read_map(file)?;
    let regions = map_into_regions(map.clone());
    let mut total_cost = 0;
    for region in regions {
        let mut corners = 0;
        for plant in &region.plants {
            let neighborhood = create_neighborhood_check(*plant, &map);
            let horizontal_budies = neighborhood[1][0] as usize + neighborhood[1][2] as usize;
            let vertical_budies = neighborhood[0][1] as usize + neighborhood[2][1] as usize;
            let outer_corners = match (horizontal_budies, vertical_budies) {
                (2, _) | (_, 2) => 0,
                (1, 1) => 1,
                (1, 0) | (0, 1) => 2,
                (0, 0) => 4,
                _ => 0,
            };
            let mut inner_corners = 0;
            if neighborhood[0][1] && neighborhood[1][0] && !neighborhood[0][0] {
                inner_corners += 1;
            }
            if neighborhood[0][1] && neighborhood[1][2] && !neighborhood[0][2] {
                inner_corners += 1;
            }
            if neighborhood[2][1] && neighborhood[1][0] && !neighborhood[2][0] {
                inner_corners += 1;
            }
            if neighborhood[2][1] && neighborhood[1][2] && !neighborhood[2][2] {
                inner_corners += 1;
            }
            corners += inner_corners + outer_corners;
        }
        total_cost += corners * region.plants.len();
    }
    Ok(total_cost)
}

fn read_map(file: &str) -> Result<Vec<Vec<char>>> {
    Ok(read_to_string(file)?
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect())
}

fn map_into_regions(mut map: Vec<Vec<char>>) -> Vec<Region> {
    let mut result = Vec::new();
    while let Some((plant_type, position)) = find_plant_type(&map) {
        let plants = extract_region_from_map(position, &mut map);
        result.push(Region { plant_type, plants })
    }
    result
}

fn find_plant_type(map: &[Vec<char>]) -> Option<(char, (usize, usize))> {
    for row in 0..map.len() {
        for column in 0..map[0].len() {
            if map[row][column] != '.' {
                return Some((map[row][column], (row, column)));
            }
        }
    }
    None
}

fn extract_region_from_map(position: (usize, usize), map: &mut [Vec<char>]) -> Vec<(usize, usize)> {
    let plant_type = map[position.0][position.1];
    map[position.0][position.1] = '.';
    let mut result = vec![position];
    if position.0 > 0 && map[position.0 - 1][position.1] == plant_type {
        result.extend(extract_region_from_map((position.0 - 1, position.1), map));
    }
    if position.0 < map.len() - 1 && map[position.0 + 1][position.1] == plant_type {
        result.extend(extract_region_from_map((position.0 + 1, position.1), map));
    }
    if position.1 > 0 && map[position.0][position.1 - 1] == plant_type {
        result.extend(extract_region_from_map((position.0, position.1 - 1), map));
    }
    if position.1 < map[position.0].len() - 1 && map[position.0][position.1 + 1] == plant_type {
        result.extend(extract_region_from_map((position.0, position.1 + 1), map));
    }
    result
}

fn create_neighborhood_check(position: (usize, usize), map: &[Vec<char>]) -> Vec<Vec<bool>> {
    let plant = map[position.0][position.1];

    vec![
        vec![
            position
                .0
                .checked_sub(1)
                .and_then(|p1| position.1.checked_sub(1).map(|p2| (p1, p2)))
                .map(|pos| map[pos.0][pos.1] == plant)
                .unwrap_or(false),
            position
                .0
                .checked_sub(1)
                .map(|pos| map[pos][position.1] == plant)
                .unwrap_or(false),
            position
                .0
                .checked_sub(1)
                .and_then(|pos| map[pos].get(position.1 + 1).map(|p| *p == plant))
                .unwrap_or(false),
        ],
        vec![
            position
                .1
                .checked_sub(1)
                .map(|pos| map[position.0][pos] == plant)
                .unwrap_or(false),
            map[position.0][position.1] == plant,
            map[position.0]
                .get(position.1 + 1)
                .map(|p| *p == plant)
                .unwrap_or(false),
        ],
        vec![
            position
                .1
                .checked_sub(1)
                .and_then(|pos| map.get(position.0 + 1).map(|line| line[pos] == plant))
                .unwrap_or(false),
            map.get(position.0 + 1)
                .map(|line| line[position.1] == plant)
                .unwrap_or(false),
            map.get(position.0 + 1)
                .and_then(|line| line.get(position.1 + 1))
                .map(|c| *c == plant)
                .unwrap_or(false),
        ],
    ]
}

#[derive(Debug)]
struct Region {
    plant_type: char,
    plants: Vec<(usize, usize)>,
}
