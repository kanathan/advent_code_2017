use std::collections::HashMap;
use regex::Regex;


#[derive(Clone, Hash)]
struct Program {
    weight: u32,
    tot_weight: u32,
    children: Vec<String>,
    parent: Option<String>,
}


fn main() {
    let mut programs = parse(include_str!("input"));

    let parent = get_parent(&programs).to_string();
    println!("{}", parent);
    parse_weights(&mut programs, &parent);
}


fn get_parent(programs: &HashMap<String, Program>) -> &str {
    let (mut name, mut prog) = programs.iter().next().unwrap();

    while let Some(parent_name) = &prog.parent {
        name = parent_name;
        prog = programs.get(name).unwrap();
    }

    name
}

fn parse_weights(programs: &mut HashMap<String, Program>, parent: &str) -> u32 {
    let mut tot_weight = programs.get(parent).unwrap().weight;
    let children = programs.get(parent).unwrap().children.clone();
    let mut bad_weight = false;

    let mut child_weight = None;
    for child_name in children.iter() {
        let weight = parse_weights(programs, child_name);
        if child_weight.is_none() {
            child_weight = Some(weight);
        } else if child_weight.unwrap() != weight {
            bad_weight = true;
            /*let delta_weight = u32::abs_diff(child_weight.unwrap(), weight);
            if child_weight.unwrap() > weight {
                let bad_child = &children[0];
                let needed_weight = programs.get(bad_child).unwrap().weight - delta_weight;
                println!("{bad_child} needs to be {needed_weight}");
            } else {
                let needed_weight = programs.get(child_name).unwrap().weight - delta_weight;
                println!("{child_name} needs to be {needed_weight}");
            }*/

        }
        tot_weight += weight;
    }

    if bad_weight {
        println!("{}", parent);
        for child_name in children.iter() {
            println!("\t{} {} ({})", child_name, programs.get(child_name).unwrap().tot_weight, programs.get(child_name).unwrap().weight);
        }
    }

    programs.get_mut(parent).unwrap().tot_weight = tot_weight;

    tot_weight
}


fn parse(input: &str) -> HashMap<String, Program> {
    let mut programs: HashMap<String, Program> = HashMap::new();

    let re = Regex::new(r"(?P<name>\w+) \((?P<weight>\d+)\)( \-> (?P<children>.+))?").unwrap();

    for line in input.lines() {
        if let Some(caps) = re.captures(line) {
            let name = caps.name("name").unwrap().as_str();
            let weight: u32 = caps.name("weight").unwrap().as_str().parse().unwrap();
            let mut children = vec![];
            if let Some(child_m) = caps.name("children") {
                for child in child_m.as_str().split(", ") {
                    children.push(child.to_string());

                    if let Some(child_prog) = programs.get_mut(child) {
                        child_prog.parent = Some(name.to_string());
                    } else {
                        let child_prog = Program {
                            weight: 0,
                            tot_weight: 0,
                            children: vec![],
                            parent: Some(name.to_string())
                        };
                        programs.insert(child.to_string(), child_prog);
                    }
                }
            }
            
            if let Some(program) = programs.get_mut(name) {
                program.weight = weight;
                program.children = children;
            } else {
                let program = Program {
                    weight,
                    tot_weight: 0,
                    children,
                    parent: None,
                };
                programs.insert(name.to_string(), program);
            }
        }

    }

    programs
}


#[cfg(test)]
mod test {
    //use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, 1);
    }
}