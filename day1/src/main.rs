use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::AddAssign;

fn main() -> std::io::Result<()> {
    part_1()?;
    part_2()?;
    Ok(())
}

fn part_1() -> std::io::Result<()> {
    let input = BufReader::new(File::open("input.txt").unwrap());
    let mut list_1: BinaryHeap<i64> = BinaryHeap::new();
    let mut list_2: BinaryHeap<i64> = BinaryHeap::new();
    for line in input.lines() {
        let line = line?;
        let (element_list_1, element_list_2) = line.split_once("   ").expect("Invalid input");
        list_1.push(
            element_list_1
                .parse()
                .expect("Failure in reading input as integer"),
        );
        list_2.push(
            element_list_2
                .parse()
                .expect("Failure in reading input as integer"),
        );
    }
    let result: i64 = list_1
        .into_sorted_vec()
        .into_iter()
        .zip(list_2.into_sorted_vec().into_iter())
        .map(|(element1, element2)| element2 - element1)
        .map(|n| n.abs())
        .sum();
    println!("Result: {}", result);
    Ok(())
}

fn part_2() -> std::io::Result<()> {
    let input = BufReader::new(File::open("input.txt").unwrap());
    let mut list_1: Vec<i64> = Vec::new();
    let mut occurences_list_2: HashMap<i64, u64> = HashMap::new();
    for line in input.lines() {
        let line = line?;
        let (element_list_1, element_list_2) = line.split_once("   ").expect("Invalid input");
        list_1.push(
            element_list_1
                .parse()
                .expect("Failure in reading input as integer"),
        );
        let element_list_2: i64 = element_list_2
            .parse()
            .expect("Failure in reading input as integer");
        occurences_list_2
            .entry(element_list_2)
            .or_default()
            .add_assign(1);
    }
    let similarity: i64 = list_1
        .into_iter()
        .map(|element1| element1 * (occurences_list_2.get(&element1).cloned().unwrap_or(0)) as i64)
        .sum();
    println!("Similarity: {}", similarity);
    Ok(())
}
