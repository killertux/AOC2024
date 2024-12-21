use itertools::iproduct;
use std::{
    collections::{BinaryHeap, HashMap},
    fs::read_to_string,
    io::Result,
    u64, usize,
};

static NUMERIC_KEYPAD: [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    ['#', '0', 'A'],
];
static NUMERIC_START: (usize, usize) = (3, 2);

static DIRECTIONAL_KEYPAD: [[char; 3]; 2] = [['#', '^', 'A'], ['<', 'V', '>']];
static DIRECTIONAL_START: (usize, usize) = (0, 2);

fn main() -> Result<()> {
    println!("Part 1 example: {}", part_1("example.txt")?);
    println!("Part 1 result: {}", part_1("input.txt")?);
    // println!("Part 2 example: {}", part_2("example.txt")?);
    // println!("Part 2 result: {}", part_2("input.txt", 100)?);
    Ok(())
}

fn part_1(file: &str) -> Result<u64> {
    let data = read_to_string(file)?;
    let codes = data.split('\n').filter(|c| !c.is_empty());
    let mut result = 0;
    for code in codes {
        let code: Vec<char> = code.chars().collect();
        let n_code: u64 = code
            .iter()
            .collect::<String>()
            .trim_end_matches('A')
            .parse()
            .expect("Error parsing code");
        let paths = find_path(code, &NUMERIC_KEYPAD, &NUMERIC_START);
        let paths = handle_directional_keypad(paths);
        let paths = handle_directional_keypad(paths);
        result += n_code * paths[0].len() as u64;
    }
    Ok(result)
}

fn handle_directional_keypad(paths: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut min_paths = Vec::new();
    let mut min_path_size = usize::MAX;
    for path in paths {
        let n_paths = find_path(path, &DIRECTIONAL_KEYPAD, &DIRECTIONAL_START);
        if n_paths[0].len() < min_path_size {
            min_path_size = n_paths[0].len();
            min_paths = n_paths;
        } else if n_paths[0].len() == min_path_size {
            min_paths.extend(n_paths);
        }
    }
    min_paths
}

fn find_path(code: Vec<char>, keypad: &[[char; 3]], start: &(usize, usize)) -> Vec<Vec<char>> {
    let mut result_paths = Vec::new();
    let mut start = *start;
    for i in 0..code.len() {
        let button = code[i];
        let result = inner_path_find(button, &keypad, &start);
        if result_paths.is_empty() {
            result_paths = result.0;
        } else {
            result_paths = iproduct!(result_paths, result.0)
                .map(|(a, b)| {
                    let mut a = a.clone();
                    a.extend(b);
                    a
                })
                .collect();
        }
        start = result.1
    }
    result_paths
}

fn inner_path_find(
    code: char,
    keypad: &[[char; 3]],
    start: &(usize, usize),
) -> (Vec<Vec<char>>, (usize, usize)) {
    let mut heap = BinaryHeap::new();
    heap.push(Node {
        button: keypad[start.0][start.1],
        position: start.clone(),
        cost: 0,
        path: Vec::new(),
    });
    let mut possible_paths = Vec::new();
    let mut min_cost = u64::MAX;
    let mut visited: HashMap<(usize, usize), u64> = HashMap::new();
    while let Some(mut node) = heap.pop() {
        if node.button == code {
            if node.cost < min_cost {
                min_cost = node.cost;
            }
            node.path.push('A');
            possible_paths.push(node);
            continue;
        }
        if let Some(cost) = visited.get(&node.position) {
            if *cost < node.cost {
                continue;
            }
        }
        if min_cost < node.cost {
            continue;
        }
        visited.insert(node.position, node.cost);
        if node.position.0 > 0 && keypad[node.position.0 - 1][node.position.1] != '#' {
            let mut path = node.path.clone();
            path.push('^');
            heap.push(Node {
                button: keypad[node.position.0 - 1][node.position.1],
                position: (node.position.0 - 1, node.position.1),
                cost: node.cost + 1,
                path,
            });
        }
        if keypad
            .get(node.position.0 + 1)
            .map(|b| b[node.position.1] != '#')
            .unwrap_or(false)
        {
            let mut path = node.path.clone();
            path.push('V');
            heap.push(Node {
                button: keypad[node.position.0 + 1][node.position.1],
                position: (node.position.0 + 1, node.position.1),
                cost: node.cost + 1,
                path,
            });
        }
        if node.position.1 > 0 && keypad[node.position.0][node.position.1 - 1] != '#' {
            let mut path = node.path.clone();
            path.push('<');
            heap.push(Node {
                button: keypad[node.position.0][node.position.1 - 1],
                position: (node.position.0, node.position.1 - 1),
                cost: node.cost + 1,
                path,
            });
        }
        if keypad[node.position.0]
            .get(node.position.1 + 1)
            .map(|b| *b != '#')
            .unwrap_or(false)
        {
            let mut path = node.path.clone();
            path.push('>');
            heap.push(Node {
                button: keypad[node.position.0][node.position.1 + 1],
                position: (node.position.0, node.position.1 + 1),
                cost: node.cost + 1,
                path,
            });
        }
    }
    let pos = possible_paths[0].position;
    (
        possible_paths
            .into_iter()
            .filter(|n| n.cost == min_cost)
            .map(|n| n.path)
            .collect(),
        pos,
    )
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Node {
    button: char,
    position: (usize, usize),
    cost: u64,
    path: Vec<char>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
