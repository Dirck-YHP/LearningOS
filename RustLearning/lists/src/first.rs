// pub struct List {
//     head: Link,
// }

// enum Link {
//     Empty,
//     More(Box<Node>),
// }

// struct Node {
//     elem: i32,
//     next: Link,
// }


pub struct List {
    head: NodePtr,
}

struct Node {
    elem: i32,
    next: NodePtr,
}

type NodePtr = Option<Box<Node>>;

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Node {
            elem: elem,
            next: self.head.clone(),
        };
    }
}