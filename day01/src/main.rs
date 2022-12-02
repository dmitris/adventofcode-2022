use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn std::error::Error>>;

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
    let mut energies: Vec<u32> = Vec::new();
    let mut acc: u32 = 0;
    for line in input.lines() {
        match line.parse::<u32>() {
            Ok(n) => acc += n,
            Err(_) => {
                energies.push(acc);
                acc = 0;
            }
        }
    }
    // need to include the last result as well
    energies.push(acc);
    // println!("Energies (1): {:?}", &energies);
    return Ok(*energies.iter().max().unwrap());
}

use std::collections::BinaryHeap;
fn part2(input: &str) -> Result<u32> {
    let mut energies = BinaryHeap::new();
    let mut acc: u32 = 0;
    for line in input.lines() {
        match line.parse::<u32>() {
            Ok(n) => acc += n,
            Err(_) => {
                energies.push(acc);
                acc = 0;
            }
        }
    }
    // need to include the last result as well
    energies.push(acc);
    // println!("Energies (2): {:?}", &energies);
    Ok(energies.iter().take(3).sum::<u32>())

    // Alternative:
    // Ok(energies.pop().unwrap() + energies.pop().unwrap() + energies.pop().unwrap())
}
