use std::{str::FromStr, num::ParseIntError, fmt::Display};

#[derive(Debug, Clone, Copy)]
struct Line {
    start: (usize, usize),
    end: (usize, usize),
}

#[derive(Debug, Clone)]
struct Diagram {
    data: Vec<u8>,
    columns: usize
}

impl Line {
    fn diagonal(&self) -> bool {
        self.start.0 != self.end.0 && self.start.1 != self.end.1
    }
    
    fn max(&self) -> (usize, usize) {
        (usize::max(self.start.0, self.end.0), usize::max(self.start.1, self.end.1))
    }
    
    fn start_end_points(&self) -> ((usize, usize),(usize, usize)) {
        if self.start.0 < self.end.0 || self.start.1 < self.end.1 {
            (self.start, self.end)
        } else {
            (self.end, self.start)
        }
    }
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

impl Diagram {
    fn new(size: (usize, usize)) -> Diagram {
        let data = vec![0; (size.0) * (size.1)];
        Diagram { data, columns: size.0 }
    }

    fn add(&mut self, line: Line) {
        let (start, end) = line.start_end_points();
        //println!("--------------------\n");
        //println!("start: {start:?}\nend: {end:?}");
        let (mut j, mut i) = start;
        loop {
            // data[i][j] = (i * c + j)
            self.data[i * self.columns + j] += 1;

            if (j, i) >= end {
                break;
            }

            println!("({i}, {j}) -> {:?}", end);
            if i < end.1 {
                i += 1;
            }
            if j < end.0 {
                j += 1;
            }
        }
        println!("{self}")
    }

    fn get_overlaps(&self) -> usize {
        let mut sum = 0;
        for &point in &self.data {
            if point > 1 {
                sum += 1;
            }
        }
        sum as usize
    }
}

impl Display for Diagram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        dbg!(self.data.len(), self.columns, self.data.len() / self.columns);
        for row in self.data.chunks(self.data.len() / self.columns) {
            for elem in row {
                if *elem != 0 {
                    write!(f, "{} ", elem)?;
                } else {
                    write!(f, ". ")?;
                }
                
            }
            
            writeln!(f)?;
        }

        Ok(())
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    
    let mut size = (1, 1);
    let mut lines = Vec::new();
    for l in input.lines() {
        let l = l.parse::<Line>().unwrap();
        let line_max = l.max();
        size.0 = size.0.max(line_max.0);
        size.1 = size.1.max(line_max.1);
        lines.push(l);
    }
    
    size.0 += 1;
    size.1 += 1;
    
    let mut diagram = Diagram::new(size);

    for line in lines {
        diagram.add(line)
    }
    
    //diagram.add(Line { start: (0, 0), end: (5, 0) });
    //diagram.add(Line { start: (0, 0), end: (0, 5) });
    
    println!("Overlaps: {}", diagram.get_overlaps())
}

