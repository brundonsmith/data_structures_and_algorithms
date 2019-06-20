
use std::boxed::Box;

// Took a swing at it myself, got a bad-but-working 
// version, then followed this tutorial: 
// https://rust-unofficial.github.io/too-many-lists/second-iter-mut.html

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct LinkedList<T> {
    first: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {

    pub const fn new() -> Self {
        return LinkedList {
            first: None
        };
    }

    pub fn push(&mut self, val: T) {
        let new_node = Box::new(Node {
            value: val,
            next: self.first.take(),
        });

        self.first = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.first.take().map(|node| {
            self.first = node.next;
            return node.value;
        })
    }

    
    pub fn peek(&self) -> Option<&T> {
        self.first.as_ref().map(|node| &node.value)
    }

    /*
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.first.as_mut().map(|node| &mut node.value)
    }*/
}


// IntoIter
pub struct IntoIter<T>(LinkedList<T>);

impl<T> LinkedList<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        // access fields of a tuple struct numerically
        self.0.pop()
    }
}


// Iter
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> LinkedList<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter { next: self.first.as_ref().map(|node| &**node) }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.value
        })
    }
}


// IterMut
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<T> LinkedList<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut { next: self.first.as_mut().map(|node| &mut **node) }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.value
        })
    }
}