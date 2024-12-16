use std::{
    collections::{btree_map::Entry, BTreeMap, HashMap, HashSet},
    fmt::Debug,
    fs::read_to_string,
    io::Result,
    rc::Rc,
    u64,
};

fn main() -> Result<()> {
    println!("Part 1 example 1: {}", part_1("example_1.txt")?);
    println!("Part 1 example 2: {}", part_1("example_2.txt")?);
    println!("Part 1 result: {}", part_1("input.txt")?);
    println!("Part 2 example 1: {}", part_2("example_1.txt")?);
    println!("Part 2 example 2: {}", part_2("example_2.txt")?);
    println!("Part 2 result: {}", part_2("input.txt")?);
    Ok(())
}

fn part_1(file: &str) -> Result<u64> {
    let map = read_input(file)?;
    Ok(find_least_cost_paths(map.reindeer, &map.destination, &map.walls).0)
}

fn part_2(file: &str) -> Result<usize> {
    let map = read_input(file)?;
    let hashset_of_tiles_in_best_paths =
        find_least_cost_paths(map.reindeer, &map.destination, &map.walls)
            .1
            .into_iter()
            .flat_map(|l| {
                l.iter()
                    .map(|reindeer| reindeer.position)
                    .collect::<Vec<(usize, usize)>>()
            })
            .collect::<HashSet<(usize, usize)>>();
    Ok(hashset_of_tiles_in_best_paths.len())
}

fn find_least_cost_paths(
    reindeer: Reindeer,
    destination: &(usize, usize),
    walls: &HashSet<(usize, usize)>,
) -> (u64, Vec<List<Reindeer>>) {
    let mut list_of_nodes_to_visit = BTreeMap::new();
    list_of_nodes_to_visit.insert(0, vec![List::new().prepend(reindeer)]);
    let mut cost_map = HashMap::new();
    let mut possible_paths = Vec::new();
    let mut min_cost = u64::MAX;
    loop {
        let Some((cost, path)) = list_of_nodes_to_visit
            .first_entry()
            .and_then(|mut entry| Some((*entry.key(), entry.get_mut().pop()?)))
        else {
            break;
        };
        if list_of_nodes_to_visit[&cost].is_empty() {
            list_of_nodes_to_visit.remove(&cost);
        }
        let reindeer = path
            .head()
            .expect("Should always have at least one element");
        if reindeer.position.0 == destination.0 && reindeer.position.1 == destination.1 {
            possible_paths.push((cost, path));
            if min_cost > cost {
                min_cost = cost
            }
            continue;
        }
        if let Some(mapped_cost) = cost_map.get(reindeer) {
            if *mapped_cost < cost {
                continue;
            } else {
                cost_map.insert(reindeer.clone(), cost);
            }
        } else {
            cost_map.insert(reindeer.clone(), cost);
        }
        if cost > min_cost {
            continue;
        }
        match reindeer.direction {
            Direction::East => {
                if !walls.contains(&(reindeer.position.0, reindeer.position.1 + 1)) {
                    let reindeer = Reindeer::new(
                        (reindeer.position.0, reindeer.position.1 + 1),
                        Direction::East,
                    );
                    let cost = cost + 1;
                    match list_of_nodes_to_visit.entry(cost) {
                        Entry::Occupied(mut paths) => {
                            paths.get_mut().push(path.prepend(reindeer));
                        }
                        Entry::Vacant(vacant) => {
                            vacant.insert(vec![path.prepend(reindeer)]);
                        }
                    };
                }
                if !walls.contains(&(reindeer.position.0 - 1, reindeer.position.1)) {
                    let reindeer = Reindeer::new(
                        (reindeer.position.0 - 1, reindeer.position.1),
                        Direction::North,
                    );
                    let cost = cost + 1001;
                    match list_of_nodes_to_visit.entry(cost) {
                        Entry::Occupied(mut paths) => {
                            paths.get_mut().push(path.prepend(reindeer));
                        }
                        Entry::Vacant(vacant) => {
                            vacant.insert(vec![path.prepend(reindeer)]);
                        }
                    };
                }
                if !walls.contains(&(reindeer.position.0 + 1, reindeer.position.1)) {
                    let reindeer = Reindeer::new(
                        (reindeer.position.0 + 1, reindeer.position.1),
                        Direction::South,
                    );
                    let cost = cost + 1001;
                    match list_of_nodes_to_visit.entry(cost) {
                        Entry::Occupied(mut paths) => {
                            paths.get_mut().push(path.prepend(reindeer));
                        }
                        Entry::Vacant(vacant) => {
                            vacant.insert(vec![path.prepend(reindeer)]);
                        }
                    };
                }
            }
            Direction::West => {
                if !walls.contains(&(reindeer.position.0, reindeer.position.1 - 1)) {
                    let reindeer = Reindeer::new(
                        (reindeer.position.0, reindeer.position.1 - 1),
                        Direction::West,
                    );
                    let cost = cost + 1;
                    match list_of_nodes_to_visit.entry(cost) {
                        Entry::Occupied(mut paths) => {
                            paths.get_mut().push(path.prepend(reindeer));
                        }
                        Entry::Vacant(vacant) => {
                            vacant.insert(vec![path.prepend(reindeer)]);
                        }
                    };
                }
                if !walls.contains(&(reindeer.position.0 - 1, reindeer.position.1)) {
                    let reindeer = Reindeer::new(
                        (reindeer.position.0 - 1, reindeer.position.1),
                        Direction::North,
                    );
                    let cost = cost + 1001;
                    match list_of_nodes_to_visit.entry(cost) {
                        Entry::Occupied(mut paths) => {
                            paths.get_mut().push(path.prepend(reindeer));
                        }
                        Entry::Vacant(vacant) => {
                            vacant.insert(vec![path.prepend(reindeer)]);
                        }
                    };
                }
                if !walls.contains(&(reindeer.position.0 + 1, reindeer.position.1)) {
                    let reindeer = Reindeer::new(
                        (reindeer.position.0 + 1, reindeer.position.1),
                        Direction::South,
                    );
                    let cost = cost + 1001;
                    match list_of_nodes_to_visit.entry(cost) {
                        Entry::Occupied(mut paths) => {
                            paths.get_mut().push(path.prepend(reindeer));
                        }
                        Entry::Vacant(vacant) => {
                            vacant.insert(vec![path.prepend(reindeer)]);
                        }
                    };
                }
            }
            Direction::North => {
                if !walls.contains(&(reindeer.position.0 - 1, reindeer.position.1)) {
                    let reindeer = Reindeer::new(
                        (reindeer.position.0 - 1, reindeer.position.1),
                        Direction::North,
                    );
                    let cost = cost + 1;
                    match list_of_nodes_to_visit.entry(cost) {
                        Entry::Occupied(mut paths) => {
                            paths.get_mut().push(path.prepend(reindeer));
                        }
                        Entry::Vacant(vacant) => {
                            vacant.insert(vec![path.prepend(reindeer)]);
                        }
                    };
                }
                if !walls.contains(&(reindeer.position.0, reindeer.position.1 + 1)) {
                    let reindeer = Reindeer::new(
                        (reindeer.position.0, reindeer.position.1 + 1),
                        Direction::East,
                    );
                    let cost = cost + 1001;
                    match list_of_nodes_to_visit.entry(cost) {
                        Entry::Occupied(mut paths) => {
                            paths.get_mut().push(path.prepend(reindeer));
                        }
                        Entry::Vacant(vacant) => {
                            vacant.insert(vec![path.prepend(reindeer)]);
                        }
                    };
                }
                if !walls.contains(&(reindeer.position.0, reindeer.position.1 - 1)) {
                    let reindeer = Reindeer::new(
                        (reindeer.position.0, reindeer.position.1 - 1),
                        Direction::West,
                    );
                    let cost = cost + 1001;
                    match list_of_nodes_to_visit.entry(cost) {
                        Entry::Occupied(mut paths) => {
                            paths.get_mut().push(path.prepend(reindeer));
                        }
                        Entry::Vacant(vacant) => {
                            vacant.insert(vec![path.prepend(reindeer)]);
                        }
                    };
                }
            }
            Direction::South => {
                if !walls.contains(&(reindeer.position.0 + 1, reindeer.position.1)) {
                    let reindeer = Reindeer::new(
                        (reindeer.position.0 + 1, reindeer.position.1),
                        Direction::South,
                    );
                    let cost = cost + 1;
                    match list_of_nodes_to_visit.entry(cost) {
                        Entry::Occupied(mut paths) => {
                            paths.get_mut().push(path.prepend(reindeer));
                        }
                        Entry::Vacant(vacant) => {
                            vacant.insert(vec![path.prepend(reindeer)]);
                        }
                    };
                }
                if !walls.contains(&(reindeer.position.0, reindeer.position.1 + 1)) {
                    let reindeer = Reindeer::new(
                        (reindeer.position.0, reindeer.position.1 + 1),
                        Direction::East,
                    );
                    let cost = cost + 1001;
                    match list_of_nodes_to_visit.entry(cost) {
                        Entry::Occupied(mut paths) => {
                            paths.get_mut().push(path.prepend(reindeer));
                        }
                        Entry::Vacant(vacant) => {
                            vacant.insert(vec![path.prepend(reindeer)]);
                        }
                    };
                }
                if !walls.contains(&(reindeer.position.0, reindeer.position.1 - 1)) {
                    let reindeer = Reindeer::new(
                        (reindeer.position.0, reindeer.position.1 - 1),
                        Direction::West,
                    );
                    let cost = cost + 1001;
                    match list_of_nodes_to_visit.entry(cost) {
                        Entry::Occupied(mut paths) => {
                            paths.get_mut().push(path.prepend(reindeer));
                        }
                        Entry::Vacant(vacant) => {
                            vacant.insert(vec![path.prepend(reindeer)]);
                        }
                    };
                }
            }
        }
    }
    (
        min_cost,
        possible_paths
            .into_iter()
            .filter(|(cost, _)| *cost == min_cost)
            .map(|(_, path)| path)
            .collect(),
    )
}

fn read_input(file: &str) -> Result<Map> {
    let mut walls = HashSet::new();
    let mut reindeer = None;
    let mut destination = None;
    for (y, line) in read_to_string(file)?
        .split('\n')
        .filter(|line| !line.is_empty())
        .enumerate()
    {
        for (x, column) in line.chars().enumerate() {
            match column {
                '#' => {
                    walls.insert((y, x));
                }
                'S' => {
                    reindeer = Some(Reindeer {
                        position: (y, x),
                        direction: Direction::East,
                    })
                }
                'E' => {
                    destination = Some((y, x));
                }
                _ => {}
            }
        }
    }
    Ok(Map {
        walls,
        reindeer: reindeer.expect("Reindeer not found"),
        destination: destination.expect("Destination not found"),
    })
}

#[derive(Debug, Clone)]
struct Map {
    walls: HashSet<(usize, usize)>,
    reindeer: Reindeer,
    destination: (usize, usize),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Reindeer {
    position: (usize, usize),
    direction: Direction,
}

impl Reindeer {
    pub fn new(position: (usize, usize), direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    West,
    East,
    South,
}

/// Copied from https://rust-unofficial.github.io/too-many-lists/third-final.html
/// Using an imuttable linked list to avoid unecessary clones
#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn prepend(&self, elem: T) -> List<T> {
        List {
            head: Some(Rc::new(Node {
                elem,
                next: self.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> List<T> {
        List {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();
        while let Some(node) = head {
            if let Ok(mut node) = Rc::try_unwrap(node) {
                head = node.next.take();
            } else {
                break;
            }
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}
