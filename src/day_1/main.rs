

fn main() {
    let input = include_str!("input");
    let vals = parse(input);
    println!("{}",get_sum(&vals, 1));
    println!("{}",get_sum(&vals, vals.len()/2));
}


fn parse(input: &str) -> Vec<i32> {
    input.chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect()
}


fn get_sum(input: &Vec<i32>, step: usize) -> i32 {
    let mut sum = 0;
    for idx in 0..input.len() {
        if input[idx] == input[(idx + step) %input.len()] {
            sum += input[idx];
        }
    }
    sum
}


#[cfg(test)]
mod test {
    //use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, 1);
    }
}