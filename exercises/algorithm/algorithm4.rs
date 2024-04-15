/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;
use std::ptr::NonNull;


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
        T: Ord + Copy,
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
        T: Ord + Copy,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        if let Some(ref mut node) = self.root {
            node.insert(value);
        } else {
            self.root = Some(Box::new(TreeNode::new(value)));
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        if let Some(ref node) = self.root {
            return node.search(value);
        }
        false
    }
}

impl<T> TreeNode<T> where T: Ord + Clone,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) where T: Ord + Copy {
        match value {
            n if n < self.value => {
                match self.left {
                    Some(ref mut left_node) => {
                        left_node.insert(value);
                    }
                    None => {
                        self.left = Some(Box::new(TreeNode::new(value)));
                    }
                }
            }
            n if n > self.value => {
                match self.right {
                    Some(ref mut right_node) => {
                        right_node.insert(value);
                    }
                    None => {
                        self.right = Some(Box::new(TreeNode::new(value)));
                    }
                }
            }
            _ => {}
        }
    }

    fn search(&self, value: T) -> bool where T: Ord + Copy {
        if self.value == value {
            return true;
        }

        if let Some(ref left) = self.left {
            if left.search(value) {
                return true;
            }
        }

        if let Some(ref right) = self.right {
            if right.search(value) {
                return true;
            }
        }

        false
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
            }
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


