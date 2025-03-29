/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

//I AM NOT DONE
#[warn(unused_imports)]
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
        //TODO
          // 处理空树的情况
          if self.root.is_none() {
            self.root = Some(Box::new(TreeNode::new(value)));
            return;
        }

        // 使用可变引用遍历树
        let mut current = &mut self.root;
        loop {
            match current {
                Some(ref mut node) => {
                    match value.cmp(&node.value) {
                        Ordering::Less => {
                            // 向左子树移动
                            current = &mut node.left;
                        }
                        Ordering::Greater => {
                            // 向右子树移动
                            current = &mut node.right;
                        }
                        Ordering::Equal => {
                            // 根据需求处理重复值（这里直接返回）
                            return;
                        }
                    }
                }
                None => {
                    // 找到插入位置，创建新节点
                    *current = Some(Box::new(TreeNode::new(value)));
                    return;
                }
            }
        }
 
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        if self.root.is_none(){
            return false;
        }

        let mut currentNode=& self.root;
        loop{
            match currentNode{
                Some(ref node)=>{
                    match value.cmp(&node.value){
                        Ordering::Less=>{
                            currentNode=& node.left;
                        },
                        Ordering::Greater=>{
                            currentNode=& node.right;
                        },
                        Ordering::Equal=>{
                            return true;
                        }
                    }
                },

                None=>{
                    return false;
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
        //TODO
        if(value>=self.value){
            self.right=Some(Box::new(TreeNode::new(value)));
        }else{
            self.left=Some(Box::new(TreeNode::new(value)));
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


