use std::{
    fs::read_to_string,
    io::{Error, ErrorKind, Result},
    iter::repeat,
};

fn main() -> Result<()> {
    println!("Part 1 : {}", part1("input.txt")?);
    println!("Part 2 : {}", part2("input.txt")?);
    Ok(())
}

fn part1(file: &str) -> Result<i64> {
    let mut unpacked_disk = read_and_unpack_disck(file)?;
    for i in 0..unpacked_disk.len() {
        if unpacked_disk[i] == -1 {
            let Some((pos, block)) = unpacked_disk
                .iter()
                .rev()
                .enumerate()
                .find(|(_, element)| **element != -1)
            else {
                break;
            };
            let block_pos = unpacked_disk.len() - 1 - pos;
            if block_pos < i {
                break;
            }

            unpacked_disk[i] = *block;
            unpacked_disk[block_pos] = -1;
        }
    }

    Ok(checksum(unpacked_disk))
}

fn part2(file: &str) -> Result<i64> {
    let mut unpacked_disk = read_and_unpack_disck(file)?;
    let mut i = (unpacked_disk.len() - 1) as isize;
    while i >= 0 {
        if unpacked_disk[i as usize] != -1 {
            let file_id = unpacked_disk[i as usize];
            let file_size = unpacked_disk[0..=i as usize]
                .iter()
                .rev()
                .take_while(|n| **n == file_id)
                .count();
            i -= file_size as isize - 1;
            let mut start_cursor_to_find_free_space = 0;
            loop {
                let Some((start_free_space, _)) = unpacked_disk[start_cursor_to_find_free_space..]
                    .iter()
                    .enumerate()
                    .find(|(_, element)| **element == -1)
                else {
                    break;
                };
                let start_free_space = start_cursor_to_find_free_space + start_free_space;
                let Some((free_space_size, _)) = unpacked_disk[start_free_space..]
                    .iter()
                    .enumerate()
                    .find(|(_, element)| **element != -1)
                else {
                    break;
                };
                if start_free_space as isize > i {
                    break;
                }
                if file_size > free_space_size {
                    start_cursor_to_find_free_space = start_free_space + free_space_size;
                    continue;
                }
                unpacked_disk.copy_within(i as usize..(i as usize + file_size), start_free_space);
                unpacked_disk[i as usize..(i as usize + file_size)]
                    .iter_mut()
                    .for_each(|e| *e = -1);
                break;
            }
        }
        i -= 1
    }
    Ok(checksum(unpacked_disk))
}

fn read_and_unpack_disck(file: &str) -> Result<Vec<i32>> {
    let data = read_to_string(file)?;
    let mut unpacked_disk: Vec<i32> = Vec::new();
    for (pos, c) in data.chars().enumerate() {
        if c == '\n' {
            break;
        }
        let Some(c_as_number) = c.to_digit(10) else {
            return Err(Error::new(ErrorKind::InvalidData, "Not a valid number"));
        };
        if pos % 2 == 0 {
            unpacked_disk.extend(repeat((pos / 2) as i32).take(c_as_number as usize));
        } else {
            unpacked_disk.extend(repeat(-1).take(c_as_number as usize));
        }
    }
    Ok(unpacked_disk)
}

fn checksum(disk: Vec<i32>) -> i64 {
    disk.into_iter()
        .enumerate()
        .filter(|(_, n)| *n != -1)
        .map(|(pos, n)| pos as i64 * n as i64)
        .sum()
}
