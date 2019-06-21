
use std::sync::Arc;

pub struct LinkedListPersistent<T> {
    head: Link<T>,
}

impl<T> LinkedListPersistent<T> {

    pub fn new() -> Self {
        LinkedListPersistent { head: None }
    }

    pub fn append(&self, value: T) -> Self {
        LinkedListPersistent { head: Some(Arc::new(Node {
            value: value,
            next: self.head.clone(),
        }))}
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value )
    }

    pub fn tail(&self) -> Self {
        LinkedListPersistent { head: self.head.as_ref().and_then(|node| node.next.clone()) }
    }
}

struct Node<T> {
    value: T,
    next: Link<T>,
}

type Link<T> = Option<Arc<Node<T>>>;


// Iter
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> LinkedListPersistent<T> {
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { next: self.head.as_ref().map(|node| &**node) }
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


// Drop
impl<T> Drop for LinkedListPersistent<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        while let Some(node) = current {
            match Arc::try_unwrap(node) {
                Ok(mut node) => current = node.next.take(),
                _ => break
            }
        }
    }
}