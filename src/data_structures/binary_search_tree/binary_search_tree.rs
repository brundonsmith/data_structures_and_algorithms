
use std::mem;

pub struct BinarySearchTree<T: Ord> {
    head: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T: Ord> {
    value: T,
    left: Link<T>,
    right: Link<T>,
}



impl<T: Ord> BinarySearchTree<T> {

    pub fn new() -> Self {
        BinarySearchTree { head: None }
    }

    pub fn insert(&mut self, value: T) {
        match &mut self.head {
            None => self.head = Some(Box::new(Node::new(value))),
            Some(node) => node.insert(value)
        };
    }

    /*
    pub fn remove(&mut self, value: &T) -> Option<T> {
        match &mut self.head {
            None => self.head.map(|n| n.value).take(),
            Some(node) => node.remove(value)
        }
    }*/

    pub fn contains(&self, value: &T) -> bool {
        match &self.head {
            Some(node) => node.contains(value),
            None => false
        }
    }
}

impl<T: Ord> Node<T> {

    fn new(value: T) -> Self {
        Node { 
            value: value, 
            left: None, 
            right: None
        }
    }

    fn insert(&mut self, value: T) {
        if value < self.value {
            match &mut self.left {
                None => self.left = Some(Box::new(Node::new(value))),
                Some(node) => node.insert(value)
            }
        } else {
            match &mut self.right {
                None => self.right = Some(Box::new(Node::new(value))),
                Some(node) => node.insert(value)
            }
        }
    }

    
    fn remove(&mut self, value: &T) -> Option<T> {

        match &self.left {
            Some(node) => {

            },
            None
        }

        if match &self.left {
            
        } else if self.right.map(|n| n.value == *value).unwrap_or(false) {

        }
    }


    fn contains(&self, value: &T) -> bool {
        self.value == *value 
            || match &self.left {
                Some(node) => node.contains(value),
                None => false
            }
            || match &self.right {
                Some(node) => node.contains(value),
                None => false
            }
    }
}