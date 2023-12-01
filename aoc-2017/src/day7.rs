use std::collections::HashMap;

#[derive(Debug)]
pub struct TreeNode {
    name: String,
    value: u32,
    parent: Option<String>,
    children: Vec<String>,
    weight: u32
}

pub fn read_input(input: &str) -> HashMap::<String, TreeNode> {
    let mut mapping: HashMap<String, TreeNode> = input.lines().map(|line: &str| {
        let line = line.split_whitespace().collect::<Vec<_>>();
        let name = String::from(line[0]);
        let value = line[1].replace("(", "").replace(")", "").parse::<u32>().unwrap();
        let children = if line.len() > 2 {
            line[3..].into_iter().cloned().map(|s| s.replace(",", "")).collect::<Vec<_>>()
        } else {
            vec!()
        };

        (name.clone(), TreeNode {
            name,
            value,
            parent: None,
            children,
            weight: 0
        })
    }).collect();
    let keys = mapping.keys().cloned().collect::<Vec<_>>();
    for node in keys {
        let children = mapping.get(&node).unwrap().children.clone();
        for child in children {            
            if let Some(mut n) = mapping.get_mut(&child) {
                n.parent = Some(node.clone());
            }
        }
    }   

    mapping
}

fn algorithm1(mapping: HashMap::<String, TreeNode>) -> String {    
    mapping.values().find(|n| n.parent == None).unwrap().name.clone()
}

fn weight(root: String, mapping: &HashMap::<String, TreeNode>) -> Result<u32, (String, u32)> {
    let node = mapping.get(&root).unwrap();

    let children = node.children.iter()
        .map(|child| weight(child.clone(), mapping))
        .collect::<Vec<_>>();        

    // No children
    let children_size = children.len();
    if children_size == 0 {
        return Ok(node.value);
    }   

    let mut iterator = children.into_iter().enumerate();
    let first_value = iterator.next().unwrap().1?;
    let second_value = iterator.next().unwrap().1?;

    if first_value != second_value {
        if children_size == 2 {
            unreachable!();
        }

        let third_value = iterator.next().unwrap().1?;
        //println!("{} - {} - {}", first_value, second_value, third_value);
        // The third value is the same as the first
        // second value is wrong
        if third_value == first_value {
            let wrong_value = mapping[&node.children[1]].value;
            println!("{} - {} - {}: {}", first_value, second_value, third_value, wrong_value);
            return Err((node.children[1].clone(), wrong_value + third_value - second_value));
        } else {
            // otherwise first value is wrong
            //println!("{}", mapping[&node.children[0]].name);
            let wrong_value = mapping[&node.children[0]].value;
            println!("{} - {} - {}: {}", first_value, second_value, third_value, wrong_value);
            return Err((node.children[0].clone(), wrong_value + third_value - first_value));
        }
    } else {        
        while let Some((index, child)) = iterator.next() {
            let third_value = child?;
            if first_value != third_value {
                let wrong_value = mapping[&node.children[index]].value;
                println!("{} - {} - {}: {}", first_value, second_value, third_value, wrong_value);
                return Err((node.children[index].clone(), wrong_value + first_value - third_value));
            }
        }
    }

    return Ok(first_value * children_size as u32 + node.value);
}


#[test]
fn test7() {
    let input = read_input(r"pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)");

    assert_eq!(weight(String::from("tknk"), &input), Err((String::from("ugml"), 60)));
    assert_eq!(algorithm1(input), String::from("tknk"));
}

#[test]
fn run7() {
    let input = read_input(include_str!("input/day7.txt"));

    println!("{:?}", weight(String::from("hmvwl"), &input));
    println!("{}", algorithm1(input));
}