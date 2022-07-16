use itertools::Itertools;

fn main() {
    let input = include_str!("input");
    let vals = parse(input);
    println!("{}",get_checksum(&vals));
    println!("{}",get_div_checksum(&vals));
}


fn parse(input: &str) -> Vec<Vec<i32>> {
    input.lines()
        .map(|line| line.split_ascii_whitespace().map(|s| s.parse().unwrap()).collect())
        .collect()
}


fn get_checksum(input: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for line in input {
        if line.is_empty() {
            break
        }
        let min = line.iter().min().unwrap();
        let max = line.iter().max().unwrap();
        sum += max - min;
    }
    sum
}

fn get_div_checksum(input: &Vec<Vec<i32>>) -> i32 {
    let mut sum = 0;
    for line in input {
        if line.is_empty() {
            break
        }
        for pair in line.iter().combinations(2) {
            let min = pair.iter().min().unwrap();
            let max = pair.iter().max().unwrap();
            if **max % **min == 0 {
                sum += **max / **min;
                break
            }
        }
    }
    sum
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, 1);
    }
}