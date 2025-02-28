/// binary_tree
use std::fmt::{Debug, Display};

type Link<T> = Option<Box<BinaryTree<T>>>;

#[derive(Debug, Clone)]
struct BinaryTree<T> {
    key: T,
    left: Link<T>,
    right: Link<T>,
}

impl<T: Clone> BinaryTree<T> {
    fn new(key: T) -> Self {
        BinaryTree {
            key,
            left: None,
            right: None,
        }
    }

    fn insert_left_tree(&mut self, key: T) {
        if self.left.is_none() {
            let node = BinaryTree::new(key);
            self.left = Some(Box::new(node));
        } else {
            let mut node = BinaryTree::new(key);
            node.left = self.left.take();
            self.left = Some(Box::new(node));
        }
    }

    fn insert_right_tree(&mut self, key: T) {
        if self.right.is_none() {
            let node = BinaryTree::new(key);
            self.right = Some(Box::new(node));
        } else {
            let mut node = BinaryTree::new(key);
            node.right = self.right.take();
            self.right = Some(Box::new(node));
        }
    }

    fn get_left(&self) -> Link<T> {
        self.left.clone()
    }

    fn get_right(&self) -> Link<T> {
        self.right.clone()
    }

    fn get_key(&self) -> T {
        self.key.clone()
    }

    fn set_key(&mut self, key: T) {
        self.key = key;
    }
}

// 前序遍历
fn preorder<T: Clone + Debug>(bt: Link<T>) {
    if !bt.is_none() {
        println!("key is {:?};", bt.as_ref().unwrap().get_key());
        preorder(bt.as_ref().unwrap().get_left());
        preorder(bt.as_ref().unwrap().get_right());
    }
}

// 后序遍历
fn postorder<T: Clone + Debug>(bt: Link<T>) {
    if !bt.is_none() {
        postorder(bt.as_ref().unwrap().get_left());
        postorder(bt.as_ref().unwrap().get_right());
        println!("key is {:?};", bt.as_ref().unwrap().get_key());
    }
}

// 中序遍历
fn inorder<T: Clone + Debug>(bt: Link<T>) {
    if !bt.is_none() {
        inorder(bt.as_ref().unwrap().get_left());
        println!("key is {:?};", bt.as_ref().unwrap().get_key());
        inorder(bt.as_ref().unwrap().get_right());
    }
}

impl<T: Clone + Debug> BinaryTree<T> {
    // 前序遍历
    fn preorder(&self) {
        println!("key is {:?};", self.key);
        if !self.left.is_none() {
            println!("left is {:?};", self.right.as_ref().unwrap().preorder());
        }
        if !self.right.is_none() {
            println!("right is {:?};", self.right.as_ref().unwrap().preorder());
        }
    }

    // 后序遍历
    fn postorder(&self) {
        if !self.left.is_none() {
            self.left.as_ref().unwrap().postorder();
        }
        if !self.right.is_none() {
            self.right.as_ref().unwrap().postorder();
        }
        println!("key is {:?};", &self.key);
    }

    // 中序遍历
    fn inorder(&self) {
        if !self.left.is_none() {
            self.left.as_ref().unwrap().inorder();
        }
        println!("key is {:?};", &self.key);
        if !self.right.is_none() {
            self.right.as_ref().unwrap().inorder();
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_binary_tree() {
        let mut bt = BinaryTree::new('A');
        let root = bt.get_key();
        println!("bt root is:{:?} ", root);
        let left = bt.get_left();
        println!("bt left is:{:?} ", left);
        let right = bt.get_right();
        println!("bt right is:{:?} ", right);

        bt.insert_left_tree('B');
        bt.insert_right_tree('C');

        let left = bt.get_left();
        println!("bt left is:{:?} ", left.unwrap().get_key());
        let right = bt.get_right();
        println!("bt right is:{:?} ", right.unwrap().get_key());
    }
}

