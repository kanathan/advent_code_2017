use itertools::Itertools;

fn main() {
    let inputs = parse(include_str!("input"));
    println!("{}", part1(&inputs));
    println!("{}", part2(&inputs));
}


fn parse(input: &str) -> Vec<Vec<String>> {
    input.lines()
        .map(|line| line.split_ascii_whitespace().map(|s| s.to_string()).collect())
        .collect()
}


fn part1(inputs: &Vec<Vec<String>>) -> i32 {
    let mut count = 0;

    for line in inputs {
        if !line.iter().combinations(2).any(|combo| combo[0] == combo[1]) {
            count += 1;
        }
    }

    count
}


fn part2(inputs: &Vec<Vec<String>>) -> i32 {

    let mut sorted_inputs = inputs.clone();

    for line in sorted_inputs.iter_mut() {
        for word in line.iter_mut() {
            *word = word.chars().sorted().collect();
        }
    }

    let mut count = 0;

    for line in sorted_inputs {
        if !line.iter().combinations(2).any(|combo| combo[0] == combo[1]) {
            count += 1;
        }
    }

    count
}


#[cfg(test)]
mod test {
    //use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, 1);
    }
}