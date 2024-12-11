use std::{
    collections::HashMap,
    fs::read_to_string,
    io::{Error, ErrorKind, Result},
};

fn main() -> Result<()> {
    println!("Part 1 example: {}", solve("example.txt", 6)?);
    println!("Part 1 result: {}", solve("input.txt", 25)?);
    println!("Part 2 result: {}", solve_part_two("example.txt", 6)?);
    println!("Part 2 result: {}", solve_part_two("input.txt", 75)?);
    Ok(())
}

fn solve(file: &str, n_iterations: usize) -> Result<usize> {
    let mut stones = read_stones(file)?;

    for blink in 0..n_iterations {
        println!("Iteration {blink} with {} stones", stones.len());
        let mut i = 0;
        let limit = stones.len();
        while i < limit {
            let number = stones[i];
            if number == 0 {
                stones[i] = 1;
                i += 1;
                continue;
            }
            let n_digits = number.ilog10() + 1;
            if n_digits % 2 == 0 {
                let base = 10i64.pow(n_digits / 2);
                let right_part = number % base;
                let left_part = number / base;
                stones[i] = left_part;
                // stones.insert(i + 1, right_part);
                stones.push(right_part);
                i += 1;
                continue;
            }
            stones[i] *= 2024;
            i += 1;
        }
    }

    Ok(stones.len())
}

fn solve_part_two(file: &str, n_iterations: usize) -> Result<usize> {
    let stones = read_stones(file)?;
    let mut result = 0;
    let mut memoization = HashMap::new();
    for stone in stones {
        result += solve_with_memoization(&mut memoization, stone, n_iterations);
    }
    println!("Solved with {} different memoizations", memoization.len());
    Ok(result)
}

fn solve_with_memoization(
    memoization: &mut HashMap<(i64, usize), usize>,
    stone: i64,
    n_iterations: usize,
) -> usize {
    if n_iterations == 0 {
        return 1;
    }
    if let Some(n) = memoization.get(&(stone, n_iterations)) {
        return *n;
    }
    let result = {
        if stone == 0 {
            solve_with_memoization(memoization, 1, n_iterations - 1)
        } else {
            let n_digits = stone.ilog10() + 1;
            if n_digits % 2 == 0 {
                let base = 10i64.pow(n_digits / 2);
                let right_part = stone % base;
                let left_part = stone / base;
                solve_with_memoization(memoization, left_part, n_iterations - 1)
                    + solve_with_memoization(memoization, right_part, n_iterations - 1)
            } else {
                solve_with_memoization(memoization, stone * 2024, n_iterations - 1)
            }
        }
    };
    memoization.insert((stone, n_iterations), result);
    return result;
}

fn read_stones(file: &str) -> Result<Vec<i64>> {
    read_to_string(file)?
        .trim()
        .split(' ')
        .map(|word| {
            Ok(word
                .parse()
                .map_err(|err| Error::new(ErrorKind::InvalidData, err))?)
        })
        .collect::<Result<Vec<i64>>>()
}
