use crate::level;
use crate::HashMap;
use crate::VecDeque;

#[cfg(test)]
#[test]
pub fn level_normal_tree() {
    let mut tree: HashMap<&str, Vec<&str>> = HashMap::new();
    tree.insert("A", vec!["B", "C"]);
    tree.insert("B", vec!["D", "E"]);
    tree.insert("C", vec![]);
    tree.insert("D", vec![]);
    tree.insert("E", vec![]);

    let mut result = "".to_owned();
    level(&mut VecDeque::new(), &tree, "A", &mut |node| {
        result.push_str(node)
    });

    assert_eq!(result, "ABCDE");
}

#[test]
pub fn level_single_node() {
    let mut tree: HashMap<&str, Vec<&str>> = HashMap::new();
    tree.insert("A", vec![]);

    let mut result = "".to_owned();
    level(&mut VecDeque::new(), &tree, "A", &mut |node| {
        result.push_str(node)
    });

    assert_eq!(result, "A");
}

#[test]
#[should_panic]
pub fn level_more_that_two_child_nodes() {
    let mut tree: HashMap<&str, Vec<&str>> = HashMap::new();
    tree.insert("A", vec!["B", "C", "D"]);

    let mut result = "".to_owned();
    level(&mut VecDeque::new(), &tree, "A", &mut |node| {
        result.push_str(node)
    });
}
