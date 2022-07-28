use std::collections::HashMap;

use regex::Regex;



fn main() {
    let particles = parse(include_str!("input"));
    println!("{:?}", part_1(&particles));
    println!("{}", part_2(&particles));

}


fn parse(input: &str) -> Vec<Particle> {
    let re = Regex::new(r"p=<(?P<p1>-?\d+),(?P<p2>-?\d+),(?P<p3>-?\d+)>, v=<(?P<v1>-?\d+),(?P<v2>-?\d+),(?P<v3>-?\d+)>, a=<(?P<a1>-?\d+),(?P<a2>-?\d+),(?P<a3>-?\d+)>").unwrap();

    input.lines().map(|l| {
        let cap = re.captures(l).unwrap();
        Particle {
            p: Vertex::new(
                cap.name("p1").unwrap().as_str().parse().unwrap(),
                cap.name("p2").unwrap().as_str().parse().unwrap(),
                cap.name("p3").unwrap().as_str().parse().unwrap(),
            ),
            v: Vertex::new(
                cap.name("v1").unwrap().as_str().parse().unwrap(),
                cap.name("v2").unwrap().as_str().parse().unwrap(),
                cap.name("v3").unwrap().as_str().parse().unwrap(),
            ),
            a: Vertex::new(
                cap.name("a1").unwrap().as_str().parse().unwrap(),
                cap.name("a2").unwrap().as_str().parse().unwrap(),
                cap.name("a3").unwrap().as_str().parse().unwrap(),
            ),
        }
    }).collect()

}


fn part_1(particles: &[Particle]) -> usize {
    particles.iter().map(|p| {
        p.a.x.pow(2) + p.a.y.pow(2) + p.a.z.pow(2)
    })
    .enumerate()
    .min_by(|(_, a1), (_, a2)| {
        a1.cmp(a2)
    }).unwrap().0
}


fn part_2(particles: &[Particle]) -> usize {
    let mut particles: HashMap<usize, Particle> = particles.iter().enumerate().map(|(idx, p)| {
        (idx, *p)
    }).collect();
    
    let mut last_collision = 0;
    let mut loop_count = 0;
    let mut collisions: HashMap<Vertex, Vec<usize>> = HashMap::new();

    for (idx, p) in particles.iter() {
        collisions.entry(p.p).or_default().push(*idx);
    }

    loop {
        // Look for collisions to remove
        for (_, idxs) in collisions.iter() {
            if idxs.len() > 1 {
                for idx in idxs {
                    particles.remove(idx);
                }
                last_collision = loop_count;
            }
        }
        collisions.clear();

        if last_collision == loop_count {
            println!("Found collision at loop {loop_count}. {} remaining", particles.len());
        }


        // Simulate remaining

        for (idx, p) in particles.iter_mut() {
            p.v.x += p.a.x;
            p.v.y += p.a.y;
            p.v.z += p.a.z;
            p.p.x += p.v.x;
            p.p.y += p.v.y;
            p.p.z += p.v.z;

            collisions.entry(p.p).or_default().push(*idx);
        }


        loop_count += 1;
        if loop_count - last_collision >= 1000 {
            break
        }
    }

    particles.len()
}


#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Particle {
    p: Vertex,
    v: Vertex,
    a: Vertex,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Vertex {
    x: isize,
    y: isize,
    z: isize,
}

impl Vertex {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self { x, y, z }
    }
}


#[cfg(test)]
mod test {
    //use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, 1);
    }
}