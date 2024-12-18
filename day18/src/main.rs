use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    fs::read_to_string,
    io::{Error, ErrorKind, Result},
};

fn main() -> Result<()> {
    println!("Part 1 example: {}", part_1("example.txt", 6, 12)?);
    println!("Part 1 result: {}", part_1("input.txt", 70, 1024)?);
    // To solve with brute force
    // println!("Part 2 example: {}", part_2("example.txt", 6, 12)?);
    // println!("Part 2 result: {}", part_2("input.txt", 70, 1024)?);
    println!("Part 2 example: {}", part_2("example.txt", 6)?);
    println!("Part 2 result: {}", part_2("input.txt", 70)?);
    Ok(())
}

fn part_1(file: &str, grid_size: usize, bytes_fallen: usize) -> Result<u64> {
    let bytes_to_fall = read_input(file)?;
    let fallen_bytes: HashSet<(usize, usize)> =
        bytes_to_fall.into_iter().take(bytes_fallen).collect();
    let shortest_path_cost = find_path_cost(grid_size, &fallen_bytes, true);
    Ok(shortest_path_cost.unwrap())
}

// Part 2 using brute force
// fn part_2(file: &str, grid_size: usize, bytes_fallen: usize) -> Result<String> {
//     let bytes_to_fall = read_input(file)?;
//     let mut fallen_bytes: HashSet<(usize, usize)> =
//         bytes_to_fall.iter().take(bytes_fallen).cloned().collect();
//     for byte in bytes_to_fall.into_iter().skip(bytes_fallen) {
//         fallen_bytes.insert(byte.clone());
//         if find_path_cost(grid_size, &fallen_bytes, false).is_none() {
//             return Ok(format!("{},{}", byte.0, byte.1));
//         }
//     }
//     Ok("".to_string())
// }

// Part 2 using binary search
fn part_2(file: &str, grid_size: usize) -> Result<String> {
    let bytes_to_fall = read_input(file)?;
    let mut start = 0;
    let mut end = bytes_to_fall.len();
    let mut mid = bytes_to_fall.len() / 2;
    loop {
        let fallen_bytes: HashSet<(usize, usize)> =
            bytes_to_fall.iter().take(mid).cloned().collect();
        if find_path_cost(grid_size, &fallen_bytes, false).is_none() {
            if mid - start == 1 {
                let byte = bytes_to_fall
                    .iter()
                    .skip(start)
                    .take(1)
                    .next()
                    .expect("Should exist a byte");
                return Ok(format!("{},{}", byte.0, byte.1));
            }
            end = mid;
            mid = end / 2;
            continue;
        }
        let fallen_bytes: HashSet<(usize, usize)> =
            bytes_to_fall.iter().take(end).cloned().collect();
        if find_path_cost(grid_size, &fallen_bytes, false).is_none() {
            if end - mid == 1 {
                let byte = bytes_to_fall
                    .iter()
                    .skip(mid)
                    .take(1)
                    .next()
                    .expect("Should exist a byte");
                return Ok(format!("{},{}", byte.0, byte.1));
            }
            start = mid;
            mid = (end - start) / 2 + start;
            continue;
        }
    }
}

fn find_path_cost(
    grid_size: usize,
    fallen_bytes: &HashSet<(usize, usize)>,
    shortest: bool,
) -> Option<u64> {
    let mut nodes = BinaryHeap::new();
    nodes.push(Node {
        cost: 0,
        position: (0, 0),
    });
    let mut visited_nodes: HashMap<(usize, usize), u64> = HashMap::new();
    let mut min_cost = None;
    while let Some(node) = nodes.pop() {
        if node.position == (grid_size, grid_size) {
            if !shortest {
                return Some(node.cost);
            }
            if min_cost.map(|c| c > node.cost).unwrap_or(true) {
                min_cost = Some(node.cost);
            }
            continue;
        }
        if min_cost.map(|cost| node.cost > cost).unwrap_or(false) {
            continue;
        }
        if !visited_nodes.contains_key(&node.position) {
            visited_nodes.insert(node.position, node.cost);
        } else {
            if visited_nodes[&node.position] > node.cost {
                visited_nodes.insert(node.position, node.cost);
            } else {
                continue;
            }
        }
        if node.position.0 > 0 && !fallen_bytes.contains(&(node.position.0 - 1, node.position.1)) {
            nodes.push(Node {
                cost: node.cost + 1,
                position: (node.position.0 - 1, node.position.1),
            });
        }
        if node.position.0 < grid_size
            && !fallen_bytes.contains(&(node.position.0 + 1, node.position.1))
        {
            nodes.push(Node {
                cost: node.cost + 1,
                position: (node.position.0 + 1, node.position.1),
            });
        }
        if node.position.1 > 0 && !fallen_bytes.contains(&(node.position.0, node.position.1 - 1)) {
            nodes.push(Node {
                cost: node.cost + 1,
                position: (node.position.0, node.position.1 - 1),
            });
        }
        if node.position.1 < grid_size
            && !fallen_bytes.contains(&(node.position.0, node.position.1 + 1))
        {
            nodes.push(Node {
                cost: node.cost + 1,
                position: (node.position.0, node.position.1 + 1),
            });
        }
    }
    min_cost
}

fn read_input(file: &str) -> Result<Vec<(usize, usize)>> {
    read_to_string(file)?
        .split('\n')
        .filter(|line| !line.is_empty())
        .flat_map(|line| line.split_once(','))
        .map(|(x, y)| {
            Ok((
                x.parse()
                    .map_err(|err| Error::new(ErrorKind::InvalidData, err))?,
                y.parse()
                    .map_err(|err| Error::new(ErrorKind::InvalidData, err))?,
            ))
        })
        .collect()
}

#[derive(Eq, PartialEq, Debug, Clone)]
struct Node {
    cost: u64,
    position: (usize, usize),
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
