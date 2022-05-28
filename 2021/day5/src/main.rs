use std::{str::FromStr, num::ParseIntError};

#[derive(Debug, Clone, Copy)]
struct Line {
    start: (u16, u16),
    end: (u16, u16),
}

#[derive(Debug, Clone)]
struct Diagram {
    points: Vec<Line>
}


impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once(" -> ").unwrap();
        let (x1, y1) = start.split_once(',').unwrap();
        let (x2, y2) = end.split_once(',').unwrap();
        Ok(Line { start: (x1.parse()?, y1.parse()?), end: (x2.parse()?, y2.parse()?)})
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    for line in input.lines() {
        let line: Line = line.parse().unwrap();
        println!("{:?}", line)
    }
}
