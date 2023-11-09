use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut list = List { head: Link::Empty };
        assert!(matches!(list.head, Link::Empty));

        list.head = Link::More(Box::new(Node {
            elem: 3,
            next: Link::Empty,
        }));
        let y = if let Link::More(b) = list.head {
            Some(b.elem)
        } else {
            None
        };
        assert_eq!(y.unwrap(), 3);
    }

    #[test]
    fn push_pop() {
        let mut list = List::new();
        list.push(16);
        list.push(17);
        list.push(42);

        let mut elements = Vec::new();
        while let Some(el) = list.pop() {
            elements.push(el);
        }
        assert_eq!(vec![42, 17, 16], elements);
        assert!(matches!(list.head, Link::Empty));
    }
}
