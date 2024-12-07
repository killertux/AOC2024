use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Result};

fn main() -> std::io::Result<()> {
    println!("Part 1 : {}", part1("input.txt")?);
    println!("Part 2 : {}", part2("input.txt")?);
    Ok(())
}

fn part1(file: &str) -> std::io::Result<u64> {
    let equations = read_equations(file)?;
    let operators = vec![Operators::Add, Operators::Mul];
    let sum_of_valid = equations
        .into_iter()
        .filter(|equation| validate_equation(equation, &operators))
        .map(|equation| equation.result)
        .sum();

    Ok(sum_of_valid)
}

fn part2(file: &str) -> std::io::Result<u64> {
    let equations = read_equations(file)?;
    let operators = vec![Operators::Add, Operators::Mul, Operators::Concatenate];
    let sum_of_valid = equations
        .into_iter()
        .filter(|equation| validate_equation(equation, &operators))
        .map(|equation| equation.result)
        .sum();
    Ok(sum_of_valid)
}

fn read_equations(file: &str) -> Result<Vec<Equation>> {
    let buf_read = BufReader::new(File::open(file)?);
    buf_read
        .lines()
        .map(|line| {
            let line = line?;
            let Some((result, numbers)) = line.split_once(':') else {
                return Err(Error::new(ErrorKind::InvalidData, "Not found result"));
            };
            let numbers = numbers
                .trim()
                .split(" ")
                .map(|n| n.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e)))
                .collect::<Result<Vec<u64>>>()?;
            Ok(Equation {
                result: result
                    .parse()
                    .map_err(|e| Error::new(ErrorKind::InvalidData, e))?,
                numbers,
            })
        })
        .collect::<Result<Vec<Equation>>>()
}

fn validate_equation(equation: &Equation, operators: &[Operators]) -> bool {
    let operators = get_combination_of_operators(equation.numbers.len() - 1, operators);

    for operator_combination in operators {
        let mut result = equation.numbers[0];
        for (num, op) in equation.numbers[1..].iter().zip(operator_combination) {
            match op {
                Operators::Add => {
                    result += num;
                }
                Operators::Mul => {
                    let Some(mul) = result.checked_mul(*num) else {
                        return false; // If we overflowed, than we do not have a valid result
                    };
                    result = mul;
                }
                Operators::Concatenate => {
                    let Some(shifted_number) = result.checked_mul(10u64.pow(num.ilog10() + 1))
                    else {
                        return false; // If we overflowed, than we do not have a valid result
                    };
                    result = shifted_number + num;
                }
            }
        }
        if result == equation.result {
            return true;
        }
    }
    false
}

#[derive(Debug)]
struct Equation {
    result: u64,
    numbers: Vec<u64>,
}

#[derive(Debug, Clone)]
enum Operators {
    Mul,
    Add,
    Concatenate,
}

fn get_combination_of_operators(n: usize, operators: &[Operators]) -> Vec<Vec<Operators>> {
    if n == 0 {
        return vec![];
    }
    if n == 1 {
        return operators.into_iter().map(|e| vec![e.clone()]).collect();
    }
    let mut result = Vec::new();
    let inner_result = get_combination_of_operators(n - 1, operators);
    for operator in operators {
        for mut i_result in inner_result.iter().cloned() {
            i_result.insert(0, operator.clone());
            result.push(i_result);
        }
    }
    result
}
