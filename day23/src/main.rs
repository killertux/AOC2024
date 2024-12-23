use std::{collections::HashSet, fs::read_to_string, io::Result};

fn main() -> Result<()> {
    println!("Part 1 example: {}", part_1("example.txt")?);
    println!("Part 1 result: {}", part_1("input.txt")?);
    println!("Part 2 example: {}", part_2("example.txt")?);
    println!("Part 2 result: {}", part_2("input.txt")?);
    Ok(())
}

fn part_1(file: &str) -> Result<usize> {
    let (adjacency_matrix, list_of_nodes) = read_input(file)?;
    let mut adj_with_node_with_t = list_of_nodes
        .iter()
        .enumerate()
        .filter(|(_, node)| node.starts_with('t'))
        .flat_map(|(pos1, n1)| {
            let mut result = Vec::new();
            for (pos2, n2) in list_of_nodes.iter().enumerate() {
                for (pos3, n3) in list_of_nodes.iter().enumerate() {
                    if n1 == n2 || n1 == n3 || n2 == n3 {
                        continue;
                    }
                    if adjacency_matrix[pos1][pos2] > 0
                        && adjacency_matrix[pos1][pos3] > 0
                        && adjacency_matrix[pos2][pos3] > 0
                    {
                        let mut vec = vec![n1.clone(), n2.clone(), n3.clone()];
                        vec.sort();
                        result.push(vec);
                    }
                }
            }
            result.sort();
            result.dedup();
            result
        })
        .collect::<Vec<Vec<String>>>();
    adj_with_node_with_t.sort();
    adj_with_node_with_t.dedup();
    Ok(adj_with_node_with_t.len())
}

fn part_2(file: &str) -> Result<String> {
    let (adjacency_matrix, list_of_nodes) = read_input(file)?;
    let mut largest_clique: Option<Vec<(usize, String)>> = None;
    for (pos, node) in list_of_nodes.iter().enumerate() {
        let mut clique = vec![(pos, node.clone())];
        for (pos_n, node_n) in list_of_nodes.iter().enumerate() {
            if let Err(clique_pos) = clique.binary_search_by(|p| p.1.cmp(node_n)) {
                if clique.iter().all(|(p, _)| adjacency_matrix[*p][pos_n] > 0) {
                    clique.insert(clique_pos, (pos_n, node_n.clone()));
                }
            }
        }
        if largest_clique
            .as_ref()
            .map(|l_clique| l_clique.len() < clique.len())
            .unwrap_or(true)
        {
            largest_clique = Some(clique);
        }
    }
    Ok(largest_clique
        .unwrap()
        .into_iter()
        .map(|(_, node)| node)
        .collect::<Vec<String>>()
        .join(","))
}

fn read_input(file: &str) -> Result<(Vec<Vec<u8>>, Vec<String>)> {
    let data = read_to_string(file)?;
    let pairs: Vec<(&str, &str)> = data
        .split('\n')
        .filter(|line| !line.is_empty())
        .flat_map(|line| line.split_once('-'))
        .collect();
    let nodes: HashSet<&str> = pairs.iter().flat_map(|pair| [pair.0, pair.1]).collect();
    let mut nodes: Vec<String> = nodes.into_iter().map(|node| node.to_string()).collect();
    nodes.sort();
    let mut adjacency_matrix = vec![vec![0; nodes.len()]; nodes.len()];
    pairs.into_iter().for_each(|(node_a, node_b)| {
        let node_a = nodes
            .binary_search_by(|p| p.as_str().cmp(node_a))
            .expect("We know the element is there");
        let node_b = nodes
            .binary_search_by(|p| p.as_str().cmp(node_b))
            .expect("We know the element is there");
        adjacency_matrix[node_a][node_b] += 1;
        adjacency_matrix[node_b][node_a] += 1;
        adjacency_matrix[node_b][node_b] = 1;
        adjacency_matrix[node_a][node_a] = 1;
    });
    Ok((adjacency_matrix, nodes))
}
