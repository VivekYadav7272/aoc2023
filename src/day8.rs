use std::collections::{HashMap, HashSet};

// l = 5737016206541192
// n = 358742884351
// 3n = 1076228653053
// lcm(3n, 15989) = 63497490530127
// z/3 = 21165830176709
// steps_req = 5 + 15989*(z/3)

fn build_graph(s: &str) -> HashMap<&str, (&str, &str)> {
    s.lines()
        .map(|line| {
            let (src_node, rest) = line.split_once('=').expect("format spec");
            let src_node = src_node.trim();
            let (left_node, right_node) = rest.trim().split_once(',').expect("format spec");
            let left_node = left_node.trim().strip_prefix('(').expect("format spec");
            let right_node = right_node.trim().strip_suffix(')').expect("format spec");

            (src_node, (left_node, right_node))
        })
        .collect::<HashMap<&str, (&str, &str)>>()
}

pub fn level1(s: &str) -> usize {
    let (directions, graph) = s.split_once("\n\n").expect("format spec");
    let graph = build_graph(graph);
    let mut traversal_node = "AAA";
    let dest_node = "ZZZ";
    let mut steps = 0;

    for direction in directions.chars().cycle() {
        traversal_node = if direction == 'L' {
            graph[traversal_node].0
        } else {
            graph[traversal_node].1
        };

        steps += 1;

        if traversal_node == dest_node {
            break;
        }
    }
    steps
}

pub fn level2(s: &str) -> usize {
    let (directions, graph) = s.split_once("\n\n").expect("format spec");
    let graph = build_graph(graph);
    let a_guys: Vec<&str> = graph
        .keys()
        .filter_map(|key| key.ends_with('A').then_some(*key))
        .collect();

    let found_first_z_guy: Vec<usize> = a_guys
        .iter()
        .map(|&a_guy| {
            let mut traversal_node = a_guy;
            let mut steps = 0;
            for direction in directions.chars().cycle() {
                if traversal_node.ends_with('Z') {
                    break;
                }

                traversal_node = if direction == 'L' {
                    graph[traversal_node].0
                } else {
                    graph[traversal_node].1
                };

                steps += 1;
            }

            steps
        })
        .collect();

    let loopdy: Vec<(usize, usize, usize, &str)> = a_guys
        .iter()
        .map(|&a_guy| {
            let mut visited = HashMap::new();
            let mut traversal_node = a_guy;
            let mut repeat_ind = 0;
            let mut loop_loc = a_guy;
            let mut cycle_start_pt = 0usize;
            let mut steps = 0;

            for (i, direction) in directions.chars().enumerate().cycle() {
                if let Some(cycle_start) = visited.insert((i, traversal_node), steps) {
                    repeat_ind = i;
                    loop_loc = traversal_node;
                    cycle_start_pt = cycle_start;
                    break;
                }

                traversal_node = if direction == 'L' {
                    graph[traversal_node].0
                } else {
                    graph[traversal_node].1
                };

                steps += 1;
            }

            (steps, repeat_ind, cycle_start_pt, loop_loc)
        })
        .collect();

    dbg!(directions.len());

    dbg!(found_first_z_guy
        .iter()
        .zip(a_guys.iter())
        .collect::<Vec<_>>());
    dbg!(loopdy);

    let dt = *found_first_z_guy.iter().min().unwrap();
    let mut t = dt;
    loop {
        if found_first_z_guy.iter().all(|step| t % step == 0) {
            break;
        }
        t += dt;
    }
    t
}
