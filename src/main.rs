use std::collections::HashMap;
use std::collections::VecDeque;
mod tests;

pub fn dfs<'a>(
    mut visited: std::collections::HashMap<&'a str, u8>,
    tree: &'a HashMap<&str, Vec<&str>>,
    node: &'a str,
    on_print: &mut dyn for<'r> FnMut(&'r str),
) {
    if !visited.contains_key(&node) {
        on_print(node);

        if tree[node].len() > 2 {
            panic!(format!("Node: '{}' has more than two child nodes", node))
        }

        visited.insert(node, 1);
        for brother in &tree[node] {
            dfs(visited.clone(), tree, brother, on_print)
        }
    }
}

pub fn level<'a>(
    mut fifo: VecDeque<&'a str>,
    tree: &'a HashMap<&str, Vec<&str>>,
    root_node: &'a str,
    on_print: &mut dyn for<'r> FnMut(&'r str),
) {
    fifo.push_front(root_node);
    while fifo.len() != 0 {
        let poped_node = match fifo.pop_front() {
            Some(node) => node,
            None => break,
        };

        on_print(poped_node);
        let children = &tree[poped_node];

        match children.len() {
            0 => {}
            1 => fifo.push_back(children[0]),
            2 => {
                fifo.push_back(children[0]);
                fifo.push_back(children[1]);
            }
            _ => panic!("Node has more than two children"),
        }
    }
}

fn main() {}
