use std::collections::{HashMap, HashSet};
use regex::Regex;

fn main() {
    let pipes = parse(include_str!("input"));

    println!("{}", get_group_size(&pipes, 0));
    println!("{}", get_group_count(&pipes));

}


fn parse(input: &str) -> HashMap<usize, HashSet<usize>> {
    let re = Regex::new(r"(\d+) <-> (.+)").unwrap();

    let mut pipes: HashMap<usize, HashSet<usize>> = HashMap::new();

    for line in input.lines() {
        let cap = re.captures(line).unwrap();
        let left = cap.get(1).unwrap().as_str().parse().unwrap();
        for right in cap.get(2).unwrap().as_str().split(", ").map(|s| s.parse::<usize>().unwrap()) {
            pipes.entry(left).or_default().insert(right);
            pipes.entry(right).or_default().insert(left);
        }
    }

    pipes
}

fn get_group_size(pipes: &HashMap<usize, HashSet<usize>>, id: usize) -> usize {
    let mut visited = HashSet::new();
    get_neighbors(pipes, &mut visited, id);

    visited.len()
}

fn get_group_count(pipes: &HashMap<usize, HashSet<usize>>) -> usize {
    let mut groups: Vec<HashSet<usize>> = vec![];

    for parent_id in pipes.keys() {
        if groups.iter().any(|h| {
            h.contains(parent_id)
        }) { continue }
        else {
            let mut visited = HashSet::new();
            get_neighbors(pipes, &mut visited, *parent_id);
            groups.push(visited);
        }
    }

    groups.len()
}


fn get_neighbors(pipes: &HashMap<usize, HashSet<usize>>, visited: &mut HashSet<usize>, parent: usize) {
    visited.insert(parent);
    if let Some(neighbors) = pipes.get(&parent) {
        for neighbor in neighbors.iter() {
            if visited.contains(neighbor) {
                continue
            } else {
                get_neighbors(pipes, visited, *neighbor);
            }
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str =
    "0 <-> 2\n\
    1 <-> 1\n\
    2 <-> 0, 3, 4\n\
    3 <-> 2, 4\n\
    4 <-> 2, 3, 6\n\
    5 <-> 6\n\
    6 <-> 4, 5";

    #[test]
    fn test_1() {
        let pipes = parse(INPUT);

        assert_eq!(6, get_group_size(&pipes, 0));
    }

    #[test]
    fn test_2() {
        let pipes = parse(INPUT);

        assert_eq!(2, get_group_count(&pipes));
    }
}