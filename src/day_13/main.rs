use itertools::Itertools;



fn main() {
    let firewall = parse(include_str!("input"));
    println!("{}", get_severity(firewall.clone()));
    println!("{}", get_passage_time(&firewall));
}


#[derive(Clone, Debug)]
struct Layer {
    range: usize,
    scanner_pos: usize,
    forward: bool,
}

impl Layer {
    fn new(range: usize) -> Self {
        Layer { range, scanner_pos: 0, forward: true }
    }

    fn step(&mut self) {
        if self.forward {
            self.scanner_pos += 1;
        } else {
            self.scanner_pos -= 1;
        }
        if self.scanner_pos == 0 {
            self.forward = true;
        } else if self.range - 1 == self.scanner_pos {
            self.forward = false;
        }
    }
}


fn parse(input: &str) -> Vec<Option<Layer>> {
    let mut firewall = vec![];

    for line in input.lines() {
        let (depth, range) = line.split(": ").map(|s| s.parse::<usize>().unwrap()).collect_tuple().unwrap();
        while depth > firewall.len() {
            firewall.push(None);
        }
        firewall.push(Some(Layer::new(range)));
    }

    firewall
}


fn get_severity(mut firewall: Vec<Option<Layer>>) -> usize {
    let mut severity = 0;

    for cur_pos in 0..firewall.len() {
        if let Some(Some(layer)) = firewall.get(cur_pos) {
            if layer.scanner_pos == 0 {
                severity += cur_pos * layer.range;
            }
        }
        step_firewall(&mut firewall);
    }


    severity
}


fn get_passage_time(firewall: &[Option<Layer>]) -> usize {
    'outer: for start_time in 0.. {
        for (range, l) in firewall.iter().enumerate() {
            if let Some(layer) = l {
                if (range + start_time) % ((layer.range-1)*2) == 0 {
                    //println!("Layer {range} ({}) at start {start_time} failed", layer.range);
                    continue 'outer;
                }
            }
        }
        return start_time
    }
    unreachable!()
}


fn step_firewall(firewall: &mut [Option<Layer>]) {
    firewall.iter_mut().for_each(|l| {
        if let Some(layer) = l.as_mut() {
            layer.step();
        }
    });
}


#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = 
    "0: 3\n\
    1: 2\n\
    4: 4\n\
    6: 4";

    #[test]
    fn test_1() {
        assert_eq!(24, get_severity(parse(INPUT)));
    }

    #[test]
    fn test_2() {
        assert_eq!(10, get_passage_time(&parse(INPUT)));
    }
}