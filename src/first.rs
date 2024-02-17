use std::mem;


struct Node {
    elem: i32,
    next: Link,
}

enum Link {
    Empty,
    Item(Box<Node>),
}

pub struct List {
    head: Link
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


impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push (&mut self, elem: i32) {
        let new_node = Box::new( Node { 
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty), });

        self.head = Link::Item(new_node);
    }

    pub fn pop (&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::Item(node) => {
                    self.head = node.next;
                    Some(node.elem)
                },
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut current_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::Item(mut boxed_node) = current_link {
            current_link = mem::replace(&mut boxed_node.next, Link::Empty)
        }
    }
}
