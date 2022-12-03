use anyhow::{bail, Result};
use std::collections::HashSet;
use std::io::{self, Read};

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().lock().read_to_string(&mut input)?;
    let res1 = part1(&input)?;
    println!("part 1: {res1}");

    let res2 = part2(&input)?;
    println!("part 2: {res2}");
    Ok(())
}

fn part1(input: &str) -> Result<u32> {
    let mut acc: u32 = 0;
    for line in input.lines() {
        let n = line.len();
        if n % 2 != 0 {
            bail!("bad input: {} not an even number, input line: {}", n, line);
        }
        let s1: HashSet<u8> = line[0..n / 2].as_bytes().iter().copied().collect();
        let s2: HashSet<u8> = line[n / 2..n].as_bytes().iter().copied().collect();
        let common: Vec<_> = s1.intersection(&s2).copied().collect();
        if common.len() != 1 {
            bail!(
                "bad input line {}: number of common elements {} != 1: {:?}",
                line,
                common.len(),
                common
            );
        }
        let prio = priority(common[0])?;
        // println!(
        //     "line: {}, s1: {:?}, s2: {:?}, common: {:?}, priority: {}",
        //     line, s1, s2, common, prio,
        // );
        acc += prio as u32;
    }
    Ok(acc)
}

fn priority(n: u8) -> Result<u8> {
    match n {
        b'a'..=b'z' => Ok(n - (b'a' - 1)),  // n - 96
        b'A'..=b'Z' => Ok(n - (b'A' - 27)), // n - 38
        _ => bail!("bad input in priority: input {n} is outside of A-Za-z range"),
    }
}

fn part2(input: &str) -> Result<u32> {
    let mut acc: u32 = 0;

    let mut lines_it = input.lines();
    while let (Some(l1), Some(l2), Some(l3)) = (lines_it.next(), lines_it.next(), lines_it.next()) {
        let s1: HashSet<u8> = l1.as_bytes().iter().copied().collect();
        let s2: HashSet<u8> = l2.as_bytes().iter().copied().collect();
        let s3: HashSet<u8> = l3.as_bytes().iter().copied().collect();
        let s12: HashSet<u8> = s1.intersection(&s2).copied().collect();
        let badge: Vec<_> = s12.intersection(&s3).copied().collect();
        let prio = priority(badge[0])?;
        // println!(
        //     "{}\n{}\n{}\ncommon: {:?}, priority: {}\n",
        //     l1, l2, l3, badge, prio,
        // );
        acc += prio as u32;
    }
    Ok(acc)
}
