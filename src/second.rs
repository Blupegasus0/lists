use std::mem;


struct Node<T> {
    elem: T,
    next: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct List<T> {
    head: Link<T>
}


impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push (&mut self, elem: T) {
        let new_node = Box::new( Node { 
            elem: elem,
            next: self.head.take(),
        });

        self.head = Link::Some(new_node);
    }

    pub fn pop (&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty behavior
        assert_eq!(list.pop(), None);

        // Add to the list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Add more to ensure no cells are currupt
        list.push(4);
        list.push(5);

        // Check removal again
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion 
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);

    }
}

#[test]
fn peek() {
    let mut list: List<i32> = List::new();

    // Check while empty
    assert_eq!(list.peek(), None);
    assert_eq!(list.peek_mut(), None);
    
    list.push(22);
    list.push(23);

    // Check with items 
    assert_eq!(list.peek(), Some(&23));
    assert_eq!(list.peek_mut(), Some(&mut 23));

    // Check mutability 
    list.peek_mut().map(|value| {
        *value = 69
    });
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut current_link = self.head.take();

        while let Link::Some(mut boxed_node) = current_link {
            current_link = mem::replace(&mut boxed_node.next, None)
        }
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

#[test]
fn into_iter() {
    let mut list: List<i32> = List::new();
    list.push(1);
    list.push(2);
    list.push(3);

    let mut iter = list.into_iter();

    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), None);
}

