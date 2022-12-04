use std::io::{self, Read};
use std::ops::Range;

use anyhow::{anyhow, bail, Result};
use lazy_static::lazy_static;
use regex::{Captures, Regex};

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
    let mut cnt: u32 = 0;
    for line in input.lines() {
        let (r1, r2) = line2ranges(line)?;
        if r1.contains(&r2.start) && r1.contains(&(r2.end - 1))
            || r2.contains(&r1.start) && r2.contains(&(r1.end - 1))
        {
            // println!("{line} overlap (one range contains another)");
            cnt += 1;
        } else {
            // println!("{line} - no containment");
        }
    }
    Ok(cnt)
}

fn line2ranges(line: &str) -> Result<(Range<u32>, Range<u32>)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    }
    let capt = RE.captures(line).ok_or_else(|| anyhow!("bad match"))?;
    Ok((
        Range {
            start: capt2u32(&capt, 1)?,
            end: capt2u32(&capt, 2)? + 1,
        },
        Range {
            start: capt2u32(&capt, 3)?,
            end: capt2u32(&capt, 4)? + 1,
        },
    ))
}

fn capt2u32(capt: &Captures, idx: usize) -> Result<u32> {
    if idx == 0 || idx > 4 {
        bail!("bad index in capt2u32, expecting between 1 and 4 (inclusive)");
    }
    Ok(capt
        .get(idx)
        .ok_or_else(|| anyhow!("bad match"))?
        .as_str()
        .parse::<u32>()?)
}

fn part2(input: &str) -> Result<u32> {
    let mut cnt: u32 = 0;
    for line in input.lines() {
        let (r1, r2) = line2ranges(line)?;
        if overlap(&r1, &r2) {
            // println!("{line} ranges overlap");
            cnt += 1;
        } else {
            // println!("{line} - no overlap");
        }
    }
    Ok(cnt)
}

fn overlap(r1: &Range<u32>, r2: &Range<u32>) -> bool {
    r1.contains(&r2.start)
        || r1.contains(&(r2.end - 1))
        || r2.contains(&r1.start)
        || r2.contains(&(r1.end - 1))
}
