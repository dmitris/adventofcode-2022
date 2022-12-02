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
    _ = input;
    Ok(0)
}

fn part2(input: &str) -> Result<u32> {
    _ = input;
    Ok(0)
}
