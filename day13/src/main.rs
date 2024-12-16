use std::{
    fs::File,
    io::{BufRead, BufReader, Error, ErrorKind, Result},
};

fn main() -> Result<()> {
    println!("Part 1 example: {}", part_1("example.txt")?);
    println!("Part 1 result: {}", part_1("input.txt")?);
    println!("Part 2 result: {}", part_2("example.txt")?);
    println!("Part 2 result: {}", part_2("input.txt")?);
    Ok(())
}

fn part_1(file: &str) -> Result<i64> {
    let arcades = read_arcades(file)?;
    let mut tokens = 0;
    for arcade in arcades {
        let Some(solution) = solve(
            arcade.prize.0,
            arcade.prize.1,
            arcade.a.x_move,
            arcade.a.y_move,
            arcade.b.x_move,
            arcade.b.y_move,
        ) else {
            continue;
        };
        tokens += solution.0 * 3 + solution.1;
    }
    Ok(tokens)
}
fn part_2(file: &str) -> Result<i64> {
    let arcades = read_arcades(file)?;
    let mut tokens = 0;
    for arcade in arcades {
        let Some(solution) = solve(
            arcade.prize.0 + 10000000000000,
            arcade.prize.1 + 10000000000000,
            arcade.a.x_move,
            arcade.a.y_move,
            arcade.b.x_move,
            arcade.b.y_move,
        ) else {
            continue;
        };
        tokens += solution.0 * 3 + solution.1;
    }
    Ok(tokens)
}

fn read_arcades(file: &str) -> Result<Vec<Arcade>> {
    let buf_read = BufReader::new(File::open(file)?);
    let mut lines = buf_read.lines();
    let mut arcades = Vec::new();
    loop {
        let Some(line1) = lines.next() else {
            break;
        };
        let line2 = lines
            .next()
            .ok_or(Error::new(ErrorKind::InvalidData, "Missing button b"))?;
        let line3 = lines
            .next()
            .ok_or(Error::new(ErrorKind::InvalidData, "Missing prize"))?;
        let a = parse_button(line1?)?;
        let b = parse_button(line2?)?;
        let prize = parse_prize(line3?)?;
        arcades.push(Arcade { a, b, prize });
        if lines.next().is_none() {
            break;
        }
    }
    Ok(arcades)
}

fn parse_button(line: String) -> Result<Button> {
    let Some((part1, part2)) = line.split_once(',') else {
        return Err(Error::new(ErrorKind::InvalidData, "Missing `,`"));
    };
    let x_move = part1
        .split_once('+')
        .map(|(_, n)| {
            n.parse::<i64>()
                .map_err(|err| Error::new(ErrorKind::InvalidData, err))
        })
        .ok_or(Error::new(ErrorKind::InvalidData, "Missing `+`"))??;
    let y_move = part2
        .split_once('+')
        .map(|(_, n)| {
            n.parse::<i64>()
                .map_err(|err| Error::new(ErrorKind::InvalidData, err))
        })
        .ok_or(Error::new(ErrorKind::InvalidData, "Missing `+`"))??;
    Ok(Button { x_move, y_move })
}

fn parse_prize(line: String) -> Result<(i64, i64)> {
    let Some((part1, part2)) = line.split_once(',') else {
        return Err(Error::new(ErrorKind::InvalidData, "Missing `,`"));
    };
    let x = part1
        .split_once('=')
        .map(|(_, n)| {
            n.parse::<i64>()
                .map_err(|err| Error::new(ErrorKind::InvalidData, err))
        })
        .ok_or(Error::new(ErrorKind::InvalidData, "Missing `=`"))??;
    let y = part2
        .split_once('=')
        .map(|(_, n)| {
            n.parse::<i64>()
                .map_err(|err| Error::new(ErrorKind::InvalidData, err))
        })
        .ok_or(Error::new(ErrorKind::InvalidData, "Missing `=`"))??;
    Ok((x, y))
}

/// Solving this as a system of equations
///
/// Ra = Pa*A1 + Pb*B1
/// Rb = Pa*A2 + Pb*B2
///
/// Pa = (Ra - Pb*B1)/ A1
///
/// Rb= (Ra-Pb*B1)/A1*A2 + Pb*B2
/// Rb*A1 = Ra*A2 - Pb*B1*A2 + Pb*B2*A1
/// Rb*A1 = Ra*A2 + Pb * (B2*A1 - B1*A2)
/// Pb = (Rb*A1 - Ra*A2) / (B2*A1 - B1*A2)

fn solve(rx: i64, ry: i64, a1: i64, a2: i64, b1: i64, b2: i64) -> Option<(i64, i64)> {
    let pb = (ry * a1 - rx * a2) / (b2 * a1 - b1 * a2);
    let pa = (rx - pb * b1) / a1;
    if pa * a1 + pb * b1 != rx {
        return None;
    }

    if pa * a2 + pb * b2 != ry {
        return None;
    }
    Some((pa, pb))
}

#[derive(Debug, Clone)]
struct Button {
    x_move: i64,
    y_move: i64,
}

#[derive(Debug, Clone)]
struct Arcade {
    a: Button,
    b: Button,
    prize: (i64, i64),
}
