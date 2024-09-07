// in first.rs

use std::mem;

pub struct List {
    head: Link,
}

// pub says we want people outside this module to be able to use List
enum Link {
    Empty,
    // Box is a smart pointer that points to heap-allocated data
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    // Self is an alias for the type we're defining in impl (List)
    pub fn new() -> Self {
        // variant of enum is specified by :: syntax, namespace operator
        List { head: Link::Empty }
        // last expression in a function is returned by default
    }

    // pushing new node to front of list
    pub fn push(&mut self, elem: i32) {
        let new_node = Node {
            elem: elem,
            // next: self.head 
            // the above will move out of borrowed content and head will be partially initialized which is not permitted
            // temp. replace the current head with empty, and return the old head
            // this ensures head not partially initialized 
            next: mem::replace(&mut self.head, Link::Empty),
        };

        // make new node the head
        self.head = Link::More(Box::new(new_node));
    }

    // Option is an enum that represents either Some or None
    pub fn pop(&mut self) -> Option<i32> {
        // match self.head will give error
        // becasue while matching data is moved which cannot be done
        // instead borrow using &
        // match &self.head
        // making self mut so that we can change head after pop
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next; // cannot move out of borrowed content if self not mut
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        // `while let` == "do this thing until this pattern doesn't match"
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
        }
    }
}


// the following says compile test module only for tests
#[cfg(test)]
// using mod to create new file inline
mod test {
    // since this is inline new module we have to explicitly pull List
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
