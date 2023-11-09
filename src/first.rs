pub struct List {
    head: Link,
}

struct Node {
    elem: i32,
    next: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut x = List { head: Link::Empty };
        assert!(matches!(x.head, Link::Empty));

        x.head = Link::More(Box::new(Node {
            elem: 3,
            next: Link::Empty,
        }));
        let y = if let Link::More(b) = x.head {
            Some(b.elem)
        } else {
            None
        };
        assert_eq!(y.unwrap(), 3);
    }
}
