/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

//
use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        match &mut self.root {
            None => {
                self.root = Some(Box::new(TreeNode::new(value)));
            }
            Some(root) => {
                if value < root.value {
                    if let Some(ref mut l) = root.left {
                        l.insert(value);
                    }else{
                        root.left = Some(Box::new(TreeNode::new(value)));
                    }
                }else if value > root.value {
                    if let Some(ref mut r) = root.right {
                        r.insert(value);
                    }else{
                        root.right = Some(Box::new(TreeNode::new(value)));
                    }
                }else{
                }
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        Self::search_tree_node(&self.root, value)
    }
    fn search_tree_node(node: &Option<Box<TreeNode<T>>>, search_key: T) -> bool {
        match node {
            None => {
                false
            }
            Some(t) => {
                if search_key < t.value   {
                    Self::search_tree_node(&t.left, search_key)
                }else if search_key > t.value {
                    Self::search_tree_node(&t.right, search_key)
                }else{
                    true
                }
            }
        }
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        if value < self.value {
            if let Some(ref mut l) = self.left {
                l.insert(value);
            }else{
                self.left = Some(Box::new(TreeNode::new(value)));
            }
        }else if value > self.value {
            if let Some(ref mut r) = self.right {
                r.insert(value);
            }else{
                self.right = Some(Box::new(TreeNode::new(value)));
            }
        }else {

        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}

fn main() {}
