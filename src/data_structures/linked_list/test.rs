

#[cfg(test)]
mod test {
    use crate::data_structures::linked_list::{LinkedList};

    #[test]
    fn basic() {
        let mut foo: LinkedList<i32> = LinkedList::new();

        foo.push(1);
        foo.push(2);
        foo.push(3);
        foo.push(4);

        assert_eq!(foo.pop(), Some(4));
        assert_eq!(*foo.peek().unwrap(), 3);
        //assert_eq!(foo.get(3), Some(1));
    }

    
    #[test]
    fn into_iter() {
        let mut list: LinkedList<i32> = LinkedList::new();

        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);

        let mut iter = list.into_iter();

        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = LinkedList::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = LinkedList::new();
        list.push(1); list.push(2); list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}