

fn main() {
    let directions = parse(include_str!("input"));
    let output = get_dist(&directions);
    println!("{}", output.0);
    println!("{}", output.1);
}


#[derive(Clone, Copy)]
enum Dir {
    N,
    S,
    NE,
    NW,
    SE,
    SW,
}


#[derive(Copy, Clone)]
struct Coords {
    q: isize,
    r: isize,
    s: isize,
}


fn parse(input: &str) -> Vec<Dir> {
    input.lines().next().unwrap()
        .split(',')
        .map(|s| match s {
            "n" => Dir::N,
            "s" => Dir::S,
            "ne" => Dir::NE,
            "nw" => Dir::NW,
            "se" => Dir::SE,
            "sw" => Dir::SW,
            _ => unreachable!()
        }).collect()
}


fn get_dist(directions: &[Dir]) -> (usize, usize) {
    // Use cube coords
    // https://www.redblobgames.com/grids/hexagons/

    let mut coords: Coords = Coords { q: 0, r: 0, s: 0 };
    let mut max_dist = 0;

    for &dir in directions {
        match dir {
            Dir::N => { coords.r -= 1; coords.s += 1 },
            Dir::S => { coords.r += 1; coords.s -= 1 },
            Dir::NE => { coords.q += 1; coords.r -= 1 },
            Dir::SW => { coords.q -= 1; coords.r += 1 },
            Dir::NW => { coords.q -= 1; coords.s += 1 },
            Dir::SE => { coords.q += 1; coords.s -= 1 },
        }
        max_dist = max_dist.max(((coords.q.abs() + coords.r.abs() + coords.s.abs()) / 2) as usize);
    }


    (((coords.q.abs() + coords.r.abs() + coords.s.abs()) / 2) as usize, max_dist)
}


#[cfg(test)]
mod test {
    //use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, 1);
    }
}