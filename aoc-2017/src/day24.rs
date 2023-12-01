use std::collections::HashSet;

struct Component {
    a: u8,
    b: u8,
    id: u8,
    strength: u8
}

struct Graph {
    nodes: Vec<Component>,
    edges: Vec<Vec<usize>>,
    start_nodes: Vec<usize>,
}

// dynamic programming
// Does adding k_d to the mix
// d_{i+1} = max[ d_i + k_{i+1}, d_i ]

fn strongest_path(graph: &Graph) -> u32 {
    let mut queue: Vec<(usize, u8, HashSet<usize>)> = Vec::with_capacity(graph.nodes.len());
    queue.extend(graph.start_nodes.iter().map(|x| {
        if graph.nodes[*x].a == 0 {
            (*x, graph.nodes[*x].b, HashSet::new())
        } else {
            (*x, graph.nodes[*x].a, HashSet::new())
        }
    }));

    let mut strongest = 0;

    while let Some((node, connector, mut path)) = queue.pop() {
        path.insert(node);

        let is_neighbour = |id: &&usize| {
            let neighbour = &graph.nodes[**id];
            !path.contains(id) && (neighbour.a == connector || neighbour.b == connector)
        };

        for neighbour in graph.edges[node].iter().filter(is_neighbour) {
            let nhs = path.clone();            
            let length: u32 = nhs.iter().map(|x| graph.nodes[*x].strength as u32).sum::<u32>() + graph.nodes[*neighbour].strength as u32;

            if length > strongest {                
                strongest = length;
            }
            
            let neighbour_a = graph.nodes[*neighbour].a;
            let neighbour_b = graph.nodes[*neighbour].b;

            if neighbour_a == connector {
                queue.push((*neighbour, neighbour_b, nhs));
            } else {
                queue.push((*neighbour, neighbour_a, nhs));
            }
        }                
    }

    strongest
}


fn longest_path(graph: &Graph) -> u32 {
    let mut queue: Vec<(usize, u8, HashSet<usize>)> = Vec::with_capacity(graph.nodes.len());
    queue.extend(graph.start_nodes.iter().map(|x| {
        if graph.nodes[*x].a == 0 {
            (*x, graph.nodes[*x].b, HashSet::new())
        } else {
            (*x, graph.nodes[*x].a, HashSet::new())
        }
    }));

    let mut longest_length = 0;
    let mut strongest = 0;

    while let Some((node, connector, mut path)) = queue.pop() {
        path.insert(node);

        let is_neighbour = |id: &&usize| {
            let neighbour = &graph.nodes[**id];
            !path.contains(id) && (neighbour.a == connector || neighbour.b == connector)
        };

        for neighbour in graph.edges[node].iter().filter(is_neighbour) {
            let nhs = path.clone();            
            let strength: u32 = nhs.iter().map(|x| graph.nodes[*x].strength as u32).sum::<u32>() + graph.nodes[*neighbour].strength as u32;

            if nhs.len() + 1 > longest_length || (nhs.len() + 1 == longest_length && strength > strongest) {
                longest_length = nhs.len() + 1;
                strongest = strength;
            }
            
            let neighbour_a = graph.nodes[*neighbour].a;
            let neighbour_b = graph.nodes[*neighbour].b;

            if neighbour_a == connector {
                queue.push((*neighbour, neighbour_b, nhs));
            } else {
                queue.push((*neighbour, neighbour_a, nhs));
            }
        }                
    }

    strongest
}



fn create_graph(input: Vec<Component>) -> Graph {
    let start_nodes: Vec<usize> = input.iter().filter(|x| x.a == 0 || x.b == 0).map(|x| x.id as usize).collect();
    let mut edges: Vec<Vec<usize>> = vec!(Vec::with_capacity(input.len());input.len());
    for i in 0..input.len() {
        for j in (i+1)..input.len() {            
            let node_1 = &input[i];
            let node_2 = &input[j];

            if node_1.a == node_2.a || node_1.a == node_2.b || node_1.b == node_2.a || node_1.b == node_2.b {
                edges[i].push(j);
                edges[j].push(i);
            }
        }
    }

    Graph {
        nodes: input,
        edges,
        start_nodes
    }

}

fn read_input(input: &str) -> Vec<Component> {
    input.lines()
        .enumerate()
        .map(|(index, line)| {
            let mut iterator = line.split('/');
            let a: u8 = iterator.next().unwrap().parse().unwrap();
            let b: u8 = iterator.next().unwrap().parse().unwrap();
            
            Component {
                a, b,
                id: index as u8,
                strength: a + b
            }
        }).collect()
}

#[test]
fn test_examples() {
    let input = r"0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";
    let input = read_input(input);
    let input = create_graph(input);
    
    assert_eq!(strongest_path(&input), 31);
    assert_eq!(longest_path(&input), 19);
    
}

#[test]
fn run24() {
    let input = include_str!("input/day24.txt");
    let input = read_input(input);
    let input = create_graph(input);

    let strongest = strongest_path(&input);
    println!("Strongest size: {}", strongest);
    let longest = longest_path(&input);
    println!("Longest Strongest size: {}", longest);
}