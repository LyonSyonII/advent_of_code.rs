use std::fs::read_to_string;

fn main() {
    let inp = read_to_string("input.txt").unwrap();
    let mut oxygen = inp.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut co2 = oxygen.clone();
    keep_elems(&mut oxygen, |count| count.0 > count.1);
    keep_elems(&mut co2, |count| count.0 <= count.1);
    
    let oxygen = oxygen.into_iter().flatten().fold(0, |acc, e| acc * 2 + e.to_digit(10).unwrap());
    let co2 = co2.into_iter().flatten().fold(0, |acc, e| acc * 2 + e.to_digit(10).unwrap());
    dbg!(oxygen * co2);
}

fn keep_elems(vec: &mut Vec<Vec<char>>, f: impl Fn((u32, u32)) -> bool) {
    for i in 0..vec[0].len() {
        if vec.len() == 1 {
            break;
        }

        let mut count = (0, 0);
        for row in vec.iter() {
            match row[i] {
                '0' => count.0 += 1,
                '1' => count.1 += 1,
                _ => unreachable!()
            }
        }

        if f(count) {
            vec.retain(|elem| elem[i] == '0')
        } else {
            vec.retain(|elem| elem[i] == '1')
        }
    }
}
