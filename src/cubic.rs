use crate::helpers::node_distance;
use crate::helpers::node_neighbours_cubic;
use ::std::collections::HashMap;
use core::panic;
pub fn a_star_path(
    start_node: (i32, i32, i32),
    nodes: HashMap<(i32, i32, i32), f32>,
    end_node: (i32, i32, i32),
    count_rings: i32,
) -> Vec<(i32, i32, i32)> {
    if !nodes.contains_key(&start_node) {
        panic!(
            "Node data does not contain start node ({},{},{})",
            start_node.0, start_node.1, start_node.2
        );
    }
    if !nodes.contains_key(&end_node) {
        panic!(
            "Node data does not contain end node ({},{},{})",
            end_node.0, end_node.1, end_node.2
        );
    }
    if start_node.0.abs() > count_rings
        || start_node.1.abs() > count_rings
        || start_node.2.abs() > count_rings
    {
        panic!("Start node is outside of searchable grid")
    }
    if end_node.0.abs() > count_rings
        || end_node.1.abs() > count_rings
        || end_node.2.abs() > count_rings
    {
        panic!("End node is outside of searchable grid")
    }

    let mut nodes_weighted: HashMap<(i32, i32, i32), (f32, f32)> = HashMap::new();
    for (k, v) in nodes.iter() {
        nodes_weighted.insert(
            k.to_owned(),
            (v.to_owned(), calculate_node_weight(k, &end_node)),
        );
    }

    let start_weight: f32 = match nodes_weighted.get(&start_node) {
        Some(x) => x.1,
        None => panic!("Unable to find node weight"),
    };

    let mut node_a_star_scores: HashMap<(i32, i32, i32), f32> = HashMap::new();
    node_a_star_scores.insert(start_node, start_weight);
    let mut queue = vec![(
        start_node,
        start_weight,
        Vec::<(i32, i32, i32)>::new(),
        0.0,
    )];

    while queue[0].0 != end_node {
        let current_path = queue.swap_remove(0);
        let available_nodes = node_neighbours_cubic(current_path.0, count_rings);
        for n in available_nodes.iter() {
            let previous_complexities: f32 = current_path.3;
            let current_node_complexity: f32 = match nodes_weighted.get(&current_path.0) {
                Some(x) => x.0 * 0.5,
                None => panic!("Unable to find current node complexity for {:?}", &n),
            };
            let target_node_complexity: f32 = match nodes_weighted.get(n) {
                Some(x) => x.0 * 0.5,
                None => panic!("Unable to find target node complexity for {:?}", &n),
            };
            let complexity =
                previous_complexities + target_node_complexity + current_node_complexity;
            let target_weight: f32 = match nodes_weighted.get(n) {
                Some(x) => x.1,
                None => panic!("Unable to find node weight for {:?}", &n),
            };
            let a_star = a_star_score(complexity, target_weight);
            let mut previous_nodes_traversed = current_path.2.clone();
            previous_nodes_traversed.push(current_path.0);
            if node_a_star_scores.contains_key(n) {
                if node_a_star_scores.get(n) >= Some(&a_star) {
                    node_a_star_scores.insert(*n, a_star);
                    let mut new_queue_item_required_for_node = true;
                    for mut q in queue.iter_mut() {
                        if &q.0 == n {
                            if q.1 >= a_star {
                                new_queue_item_required_for_node = false;
                                q.1 = a_star;
                                q.2 = previous_nodes_traversed.clone();
                                q.3 = complexity;
                            }
                        }
                    }
                    if new_queue_item_required_for_node {
                        queue.push((*n, a_star, previous_nodes_traversed, complexity));
                    }
                }
            } else {
                node_a_star_scores.insert(*n, a_star);
                queue.push((*n, a_star, previous_nodes_traversed, complexity));
            }
        }
        queue.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    }
    let mut best_path = queue[0].2.clone();
    best_path.push(end_node);
    best_path
}

fn a_star_score(complexity: f32, weighting: f32) -> f32 {
    complexity + weighting
}

fn calculate_node_weight(current_node: &(i32, i32, i32), end_node: &(i32, i32, i32)) -> f32 {
    node_distance(*current_node, *end_node) as f32
}

#[cfg(test)]
mod tests {
    use crate::cubic::a_star_path;
    use crate::cubic::calculate_node_weight;
    use std::collections::HashMap;

    #[test]
    fn node_weight_down() {
        let source: (i32, i32, i32) = (0, 0, 0);
        let end_node: (i32, i32, i32) = (2, -3, 1);
        let weight = calculate_node_weight(&source, &end_node);
        let actual_weight = 3.0;
        assert_eq!(actual_weight, weight);
    }
    #[test]
    fn node_weight_towards_origin() {
        let source: (i32, i32, i32) = (-2, -1, 3);
        let end_node: (i32, i32, i32) = (1, 0, -1);
        let weight = calculate_node_weight(&source, &end_node);
        let actual_weight = 4.0;
        assert_eq!(actual_weight, weight);
    }
    #[test]

    fn a_star_tick() {
        let start_node: (i32, i32, i32) = (0, 0, 0);
        let mut nodes: HashMap<(i32, i32, i32), f32> = HashMap::new();
        nodes.insert((0, 0, 0), 1.0);
        nodes.insert((0, -1, 1), 1.0);
        nodes.insert((1, -1, 0), 15.0);
        nodes.insert((1, 0, -1), 14.0);
        nodes.insert((0, 1, -1), 2.0);
        nodes.insert((-1, 1, 0), 6.0);
        nodes.insert((-1, 0, 1), 7.0);
        nodes.insert((0, -2, 2), 1.0);
        nodes.insert((1, -2, 1), 14.0);
        nodes.insert((2, -2, 0), 1.0);
        nodes.insert((2, -1, -1), 1.0);
        nodes.insert((2, 0, -2), 1.0);
        nodes.insert((1, 1, -2), 1.0);
        nodes.insert((0, 2, -2), 1.0);
        nodes.insert((-1, 2, -1), 3.0);
        nodes.insert((-2, 2, 0), 1.0);
        nodes.insert((-2, 1, 1), 8.0);
        nodes.insert((-2, 0, 2), 1.0);
        nodes.insert((-1, -1, 2), 2.0);
        let end_node: (i32, i32, i32) = (2, -2, 0);
        let rings = 2;
        let best = a_star_path(start_node, nodes, end_node, rings);
        let actual = vec![
            (0, 0, 0),
            (0, 1, -1),
            (1, 1, -2),
            (2, 0, -2),
            (2, -1, -1),
            (2, -2, 0),
        ];
        assert_eq!(actual, best);
    }
}
