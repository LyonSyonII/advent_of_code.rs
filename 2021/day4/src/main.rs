use std::{str::FromStr, num::ParseIntError, fmt::Display};
use colored::Colorize;

#[derive(Debug, Clone, Copy)]
struct Number {
    num: u32,
    marked: bool,
}

impl Number {
    fn mark(&mut self, n: u32) -> bool {
        if self.num == n {
            self.marked = true;
            true
        } else { 
            false 
        }
    }
}

impl std::iter::Sum for Number {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut sum = 0;
        for num in iter {
            sum += num.num;
        }
        Number { num: sum, marked: false }
    }
}

impl std::ops::Add for Number {
    type Output = Number;
    
    fn add(self, rhs: Self) -> Self::Output {
        Self { marked: false, num: self.num + rhs.num }
    }
}

impl std::ops::Add<u32> for Number {
    type Output = u32;

    fn add(self, rhs: u32) -> Self::Output {
        self.num + rhs
    }
}

impl std::ops::Add<Number> for u32 {
    type Output = u32;

    fn add(self, rhs: Number) -> Self::Output {
        rhs.num + self
    }
}

impl PartialEq<u32> for &mut Number {
    fn eq(&self, other: &u32) -> bool {
        self.num == *other
    }
}

impl FromStr for Number {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let num = s.parse()?;
        Ok(Number {num, marked: false})
    }
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let n = if self.marked { self.num.to_string().white() } else { self.num.to_string().bright_black() };
        if n.len() == 1 {
            write!(f, " {}", n)
        } else {
            write!(f, "{}", n)
        }
    }
}

#[derive(Debug)]
struct Board {
    rows: Vec<Vec<Number>>,
    sum: u32,
    has_won: bool,
}

impl Board {
    fn mark(&mut self, num: u32) {
        for row in &mut self.rows {
            for n in row {
                n.mark(num);
                if n.mark(num) {
                    self.sum -= num;
                }
            }
        }
    }

    fn columns(&self) -> Vec<Vec<Number>> {
        if self.rows.is_empty() || self.rows[0].is_empty() {
            return Vec::new()
        }
        
        let mut vec: Vec<Vec<Number>> = Vec::new();
        for j in 0..self.rows[0].len() {
            let mut col = Vec::new();
            for row in &self.rows {
                col.push(row[j])
            }
            vec.push(col);
        }
        
        vec
    }

    fn check_win(&mut self) -> bool {
        for column in self.columns() {
            if column.iter().all(|f| f.marked) {
                self.has_won = true;
                return true
            }
        }

        for row in &self.rows {
            if row.iter().all(|f| f.marked) {
                self.has_won = true;
                return true
            }
        }

        false
    }
}

impl FromIterator<Vec<Number>> for Board {
    fn from_iter<T: IntoIterator<Item = Vec<Number>>>(iter: T) -> Self {
        let rows: Vec<Vec<Number>> = iter.into_iter().collect();
        let sum = rows.iter().cloned().fold(0u32, |acc, row|  acc + row.into_iter().sum::<Number>());
        Board { rows, sum, has_won: false }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.rows {
            for num in row {
                write!(f, "{} ", num)?;
            }
            writeln!(f)?
        }

        Ok(())
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut chunks = input.split("\n\n");
    let draws = chunks.next().unwrap().split(',').map(|n| n.parse().unwrap()).collect::<Vec<u32>>();
    let mut boards = chunks
        .map(|b| b.split('\n')
            .map(|row| row.split_whitespace()
                .map(|n| n.parse::<Number>().unwrap())
                .collect::<Vec<Number>>())
            .collect::<Board>())
        .collect::<Vec<Board>>();
    
    for num in draws {
        for board in &mut boards { 
            if board.has_won { continue; }
            
            board.mark(num);
            
            if board.check_win() {
                println!("Winner:\n{board}");
                println!("Draw: {num}");
                println!("Number: {}", board.sum * num);
                println!("-----------");
            }
        }
    }
}
