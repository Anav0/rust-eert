use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
mod tests;

//TODO: change HashMap to HashSet
pub fn dfs<'a>(
    visited: &mut HashSet<&'a str>,
    tree: &'a HashMap<&str, Vec<&str>>,
    node: &'a str,
    on_node: &mut dyn for<'r> FnMut(&'r str),
) {
    if !visited.contains(&node) {
        on_node(node);

        if tree[node].len() > 2 {
            panic!(format!("Node: '{}' has more than two child nodes", node))
        }

        visited.insert(node);
        for brother in &tree[node] {
            dfs(visited, tree, brother, on_node)
        }
    }
}

pub fn level<'a>(
    fifo: &mut VecDeque<&'a str>,
    tree: &'a HashMap<&str, Vec<&str>>,
    root_node: &'a str,
    on_node: &mut dyn for<'r> FnMut(&'r str),
) {
    fifo.push_front(root_node);
    while fifo.len() != 0 {
        let poped_node = match fifo.pop_front() {
            Some(node) => node,
            None => break,
        };

        on_node(poped_node);
        let children = &tree[poped_node];

        match children.len() {
            0 => {}
            1 => fifo.push_back(children[0]),
            2 => {
                fifo.push_back(children[0]);
                fifo.push_back(children[1]);
            }
            _ => panic!(format!("Node: {} has more than two children", poped_node)),
        }
    }
}

fn main() {}
