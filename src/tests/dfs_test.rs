#[cfg(test)]
use crate::dfs;
use crate::HashMap;

#[test]
pub fn dfs_normal_tree() {
    let mut tree: HashMap<&str, Vec<&str>> = HashMap::new();
    tree.insert("A", vec!["B", "C"]);
    tree.insert("B", vec!["D", "E"]);
    tree.insert("C", vec![]);
    tree.insert("D", vec![]);
    tree.insert("E", vec![]);

    let mut result = "".to_owned();
    dfs(&mut HashMap::new(), &tree, "A", &mut |node| {
        result.push_str(node)
    });

    assert_eq!(result, "ABDEC");
}

#[test]
pub fn dfs_single_node() {
    let mut tree: HashMap<&str, Vec<&str>> = HashMap::new();
    tree.insert("A", vec![]);

    let mut result = "".to_owned();
    dfs(&mut HashMap::new(), &tree, "A", &mut |node| {
        result.push_str(node)
    });

    assert_eq!(result, "A");
}

#[test]
#[should_panic]
pub fn dfs_more_that_two_child_nodes() {
    let mut tree: HashMap<&str, Vec<&str>> = HashMap::new();
    tree.insert("A", vec!["B", "C", "D"]);

    let mut result = "".to_owned();
    dfs(&mut HashMap::new(), &tree, "A", &mut |node| {
        result.push_str(node)
    });
}
