// Minimal test to reproduce SBT remove bug
use trees::{Node, SizeBalanced, Tree};

struct SimpleTree {
    nodes: Vec<Node<usize>>,
}

impl SimpleTree {
    fn new(capacity: usize) -> Self {
        Self {
            nodes: vec![Node::default(); capacity],
        }
    }
}

impl Tree<usize> for SimpleTree {
    fn get(&self, idx: usize) -> Option<Node<usize>> {
        self.nodes.get(idx).copied()
    }

    fn set(&mut self, idx: usize, node: Node<usize>) {
        if let Some(n) = self.nodes.get_mut(idx) {
            *n = node;
        }
    }

    fn left_mut(&mut self, idx: usize) -> Option<&mut usize> {
        self.nodes.get_mut(idx)?.left.as_mut()
    }

    fn right_mut(&mut self, idx: usize) -> Option<&mut usize> {
        self.nodes.get_mut(idx)?.right.as_mut()
    }

    fn is_left_of(&self, first: usize, second: usize) -> bool {
        first < second
    }

    fn insert(&mut self, root: Option<usize>, idx: usize) -> Option<usize> {
        self.insert_sbt(root, idx)
    }

    fn remove(&mut self, root: Option<usize>, idx: usize) -> Option<usize> {
        self.remove_sbt(root, idx)
    }
}

impl SizeBalanced<usize> for SimpleTree {}

fn main() {
    let mut tree = SimpleTree::new(10);

    // Insert elements in a specific order to trigger the bug
    let mut root = None;
    root = tree.insert(root, 1);
    root = tree.insert(root, 2);
    root = tree.insert(root, 3);
    root = tree.insert(root, 4);

    println!("Inserted nodes 1, 2, 3, 4");
    println!("Root: {:?}", root);

    // Try removing node 1 - this might trigger the infinite loop
    println!("Removing node 1...");
    root = tree.remove(root, 1);
    println!("After remove, root: {:?}", root);

    // Try removing node 3
    println!("Removing node 3...");
    root = tree.remove(root, 3);
    println!("After remove, root: {:?}", root);
}
