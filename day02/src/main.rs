use std::error;
use std::fmt;
use std::io::{self, Read};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone)]
struct BadInput;

impl fmt::Display for BadInput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid input")
    }
}

impl error::Error for BadInput {}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Copy, Clone, Debug)]
enum Outcome {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

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
    let mut total: u32 = 0;
    for line in input.lines() {
        let (other, mine) = parse_line(line)?;
        total += score(other, mine);
    }
    Ok(total)
}

fn part2(input: &str) -> Result<u32> {
    let mut total: u32 = 0;
    for line in input.lines() {
        let (other, outcome) = parse_line2(line)?;
        let my_shape = choose_shape(other, outcome);
        total += score(other, my_shape);
    }
    Ok(total)
}

fn try_parse_shape(shape: &str) -> Result<Shape> {
    match shape {
        "A" | "X" => Ok(Shape::Rock),
        "B" | "Y" => Ok(Shape::Paper),
        "C" | "Z" => Ok(Shape::Scissors),
        _ => Err(Box::new(BadInput)),
    }
}

fn parse_shape(shape: &str) -> Shape {
    try_parse_shape(shape).expect("bad input")
}

fn parse_line(line: &str) -> Result<(Shape, Shape)> {
    let (a, b) = line.split_once(' ').ok_or_else(|| Box::new(BadInput))?;
    Ok((parse_shape(a), parse_shape(b)))
}

// // alternative
// fn parse_line(line: &str) -> Result<(Shape, Shape)> {
//     let (a, b) = line.split_once(' ').ok_or_else(|| Box::new(BadInput))?;
//     match (try_parse_shape(a), try_parse_shape(b)) {
//         (Ok(a), Ok(b)) => Ok((a, b)),
//         _ => Err(Box::new(BadInput)),
//     }
// }

// // alternative  = list combinations explicitly
// fn parse_line(line: &str) -> Result<(Shape, Shape)> {
//     match line.trim_end() {
//         "A X" => Ok((Shape::Rock, Shape::Rock)),
//         "A Y" => Ok((Shape::Rock, Shape::Paper)),
//         "A Z" => Ok((Shape::Rock, Shape::Scissors)),
//         "B X" => Ok((Shape::Paper, Shape::Rock)),
//         "B Y" => Ok((Shape::Paper, Shape::Paper)),
//         "B Z" => Ok((Shape::Paper, Shape::Scissors)),
//         "C X" => Ok((Shape::Scissors, Shape::Rock)),
//         "C Y" => Ok((Shape::Scissors, Shape::Paper)),
//         "C Z" => Ok((Shape::Scissors, Shape::Scissors)),
//         _ => Err(Box::new(BadInput)),
//     }
// }

// // alternative with map / collect - WIP
// Ok(line
// .splitn(2, ' ')
// .map(|s| match s {
//     "A" | "X" => Shape::Rock,
//     "B" | "Y" => Shape::Paper,
//     "C" | "Z" => Shape::Scissors,
//     _ => panic!("unexpected input: {}", line),
// })
// .take(2)
// .collect::<Shape, Shape>())

// outcome calculates whether my shape defeats the one of the other,
// makes a draw or loses to b.

fn outcome(other: Shape, me: Shape) -> Outcome {
    match (other, me) {
        (other, me) if other == me => Outcome::Draw,
        (Shape::Scissors, Shape::Rock)
        | (Shape::Paper, Shape::Scissors)
        | (Shape::Rock, Shape::Paper) => Outcome::Win,
        _ => Outcome::Loss,
    }
}

fn score(a: Shape, b: Shape) -> u32 {
    let res = outcome(a, b);
    let score = res as u32 + b as u32;
    println!("{:?} {:?}, outcome: {:?}, score: {:?}", a, b, res, score);
    // outcome(a, b) as u32 + b as u32
    score
}

fn parse_outcome(s: &str) -> Outcome {
    match s {
        "X" => Outcome::Loss,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => panic!("invalid input to parse_outcome"),
    }
}

// parse line for part2 - into (Shape, Outcome) tuple
fn parse_line2(line: &str) -> Result<(Shape, Outcome)> {
    let (a, b) = line.split_once(' ').ok_or_else(|| Box::new(BadInput))?;
    Ok((parse_shape(a), parse_outcome(b)))
}

fn choose_shape(other: Shape, outcome: Outcome) -> Shape {
    match outcome {
        Outcome::Draw => other,
        Outcome::Win => match other {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        },
        Outcome::Loss => match other {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        },
    }
}
