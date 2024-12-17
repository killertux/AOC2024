use std::{
    collections::HashMap,
    fs::read_to_string,
    io::{Error, ErrorKind, Result},
};

fn main() -> Result<()> {
    println!("Part 1 example: {}", part_1("example_1.txt")?);
    println!("Part 1 result: {}", part_1("input.txt")?);
    println!("Part 2 example: {}", part_2("example_2.txt")?);
    println!("Part 2 result: {}", part_2("input.txt")?);
    Ok(())
}

fn part_1(file: &str) -> Result<String> {
    let (mut computer, instructions) = read_input(file)?;
    let result = computer.execute(&instructions);
    Ok(result.join(","))
}

fn part_2(file: &str) -> Result<i64> {
    let (computer, instructions) = read_input(file)?;
    let result = search_register_a(
        0,
        (instructions.len() - 1) as isize,
        &computer,
        &instructions,
        8,
    );
    Ok(result.unwrap())
}

fn search_register_a(
    mut register_a: i64,
    mut cursor: isize,
    computer: &Computer,
    instructions: &[u8],
    mut limit: usize,
) -> Option<i64> {
    let desired_output = instructions
        .iter()
        .map(|n| n.to_string())
        .reduce(|mut acc, n| {
            acc.push(',');
            acc += &n;
            acc
        })
        .unwrap();
    loop {
        limit -= 1;
        if limit == 0 {
            break;
        }
        let mut computer_copied = computer.clone();
        computer_copied.register_a = register_a;
        let result = computer_copied.execute(&instructions);
        if result.join(",") == desired_output {
            return Some(register_a);
        }
        if cursor < 0 {
            break;
        }
        if result[0] == instructions[cursor as usize].to_string() {
            if let Some(solution) =
                search_register_a(register_a + 1, cursor, &computer, &instructions, limit)
            {
                return Some(solution);
            }
            limit = 8;
            cursor -= 1;
            register_a *= 8;
            continue;
        }
        register_a += 1;
    }
    None
}

fn read_input(file: &str) -> Result<(Computer, Vec<u8>)> {
    let data = read_to_string(file)?;
    let (registers_part, instructions_part) = data.split_once("\n\n").ok_or_invalid_data()?;
    let (register_a, remaining) = registers_part.split_once("\n").ok_or_invalid_data()?;
    let register_a = register_a
        .split_once("A:")
        .ok_or_invalid_data()?
        .1
        .trim()
        .parse()
        .map_err(|err| Error::new(ErrorKind::InvalidData, err))?;
    let (register_b, register_c) = remaining.split_once("\n").ok_or_invalid_data()?;
    let register_b = register_b
        .split_once("B:")
        .ok_or_invalid_data()?
        .1
        .trim()
        .parse()
        .map_err(|err| Error::new(ErrorKind::InvalidData, err))?;
    let register_c = register_c
        .split_once("C:")
        .ok_or_invalid_data()?
        .1
        .trim()
        .parse()
        .map_err(|err| Error::new(ErrorKind::InvalidData, err))?;
    let input = instructions_part
        .split_once(": ")
        .ok_or_invalid_data()?
        .1
        .split(',')
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .map(|p| {
            p.parse()
                .map_err(|err| Error::new(ErrorKind::InvalidData, err))
        })
        .collect::<Result<Vec<u8>>>()?;
    Ok((
        Computer {
            pc_counter: 0,
            register_a,
            register_b,
            register_c,
        },
        input,
    ))
}

#[derive(Debug, Clone)]
struct Computer {
    register_a: i64,
    register_b: i64,
    register_c: i64,
    pc_counter: usize,
}

impl Computer {
    pub fn execute(&mut self, input: &[u8]) -> Vec<String> {
        let mut result = Vec::new();
        while let Some(opcode) = input.get(self.pc_counter) {
            let opcode = OpCode::from_u8(*opcode);
            let operand = input[self.pc_counter + 1];
            let combo_operand = self.read_combo_operand(operand);
            match opcode {
                OpCode::Adv => {
                    self.register_a /= 1i64.checked_shl(combo_operand as u32).unwrap_or(i64::MAX)
                }
                OpCode::Bxl => self.register_b ^= operand as i64,
                OpCode::Bst => self.register_b = combo_operand % 8,
                OpCode::Jnz if self.register_a != 0 => {
                    self.pc_counter = (operand / 2) as usize;
                    continue;
                }
                OpCode::Jnz => {}
                OpCode::Bxc => self.register_b ^= self.register_c,
                OpCode::Out => result.push((combo_operand % 8).to_string()),
                OpCode::Bdv => {
                    self.register_b =
                        self.register_a / 1i64.checked_shl(combo_operand as u32).unwrap_or(i64::MAX)
                }
                OpCode::Cdv => {
                    self.register_c =
                        self.register_a / 1i64.checked_shl(combo_operand as u32).unwrap_or(i64::MAX)
                }
            }
            self.pc_counter += 2
        }
        result
    }

    fn read_combo_operand(&self, operand: u8) -> i64 {
        match operand {
            0..=3 => operand as i64,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => 0,
        }
    }
}

#[derive(Debug, Clone)]
enum OpCode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl OpCode {
    pub fn from_u8(input: u8) -> Self {
        match input {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => unreachable!("No opcode"),
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
