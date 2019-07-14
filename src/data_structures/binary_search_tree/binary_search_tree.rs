
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
        if match &self.left { Some(n) => n.value == *value, _ => false } {
            return Some(self.remove_left());
        } else if match &self.right { Some(n) => n.value == *value, _ => false } {
            return Some(self.remove_right());
        } else {
            return None;
        }
    }

    fn remove_left(&mut self) -> T {
        // safe to unwrap because this method call doesn't make sense if there's no left node
        let mut left = self.left.take().unwrap();

        let left_has_left = match left.left { Some(_) => true, None => false };
        let left_has_right =  match left.right { Some(_) => true, None => false };

        if left_has_left && !left_has_right {
            self.left = left.left;
        } else if !left_has_left && left_has_right {
            self.left = left.right;
        } else if left_has_left && left_has_right {

            // find replacement
            let mut focal_parent;
            let mut focal = unwrap_mut(&mut left.right);
            while focal.left.is_some() {
                focal_parent = mem::replace(&mut focal, unwrap_mut(&mut focal.left));
            }

            // replace replacement
            let focal_replacement = focal.right.take();
            focal_parent.left = focal_replacement;

            mem::replace(focal.left, left.left);
            focal.right = left.right;
            mem::replace(&mut self.left, Some(*focal));
            //self.left = Some(focal);
        }

        return left.value;
    }

    fn remove_right(&mut self) -> T {
        unimplemented!()
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

fn unwrap_ref<'a,T>(opt: &'a Option<T>) -> &'a T {
    opt.as_ref().unwrap()
}

fn unwrap_mut<'a,T>(opt: &'a mut Option<T>) -> &'a mut T {
    opt.as_mut().unwrap()
}



/*
fn func(foo: Option<i32>) {
    let Some(bar) = foo;
    
    println!("{}", bar + 12);
}


fn foo() -> i32 {
    
}
*/