use std::collections::HashMap;

fn dfs<'a>(
    mut visited: std::collections::HashMap<&'a str, u8>,
    tree: &'a HashMap<&str, Vec<&str>>,
    node: &'a str,
) {
    if !visited.contains_key(&node) {
        println!("{}", node);
        visited.insert(node, 1);
        for brother in &tree[node] {
            dfs(visited.clone(), tree, brother)
        }
    }
}

fn main() {
    let mut tree: HashMap<&str, Vec<&str>> = HashMap::new();
    tree.insert("A", vec!["B", "C"]);
    tree.insert("B", vec!["D", "E"]);
    tree.insert("C", vec![]);
    tree.insert("D", vec![]);
    tree.insert("E", vec![]);

    dfs(HashMap::new(), &tree, "A");
}
