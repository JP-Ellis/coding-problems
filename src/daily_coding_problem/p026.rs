use crate::{Error, Problem};
use std::{io::prelude::*, mem};

pub struct P;

const STATEMENT: &str = r#"Given a singly linked list and an integer k, remove
the kth last element from the list. k is guaranteed to be smaller than the
length of the list.

The list is very long, so making more than one pass is prohibitively expensive.

Do this in constant space and in one pass."#;

struct Node<T> {
    val: Option<T>,
    next: Option<*mut Node<T>>,
}

impl<T> Drop for Node<T> {
    fn drop(&mut self) {
        unsafe {
            if let Some(next) = self.next {
                self.next = None;
                let _to_be_dropped = Box::from_raw(next);
            }
        }
    }
}

struct LinkedList<T> {
    head: Node<T>,
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Node {
            val: Some(val),
            next: None,
        }
    }

    fn into_inner(mut self) -> T {
        if self.next.is_some() {
            panic!("Cannot call into_inner() if node owns the next node.");
        }
        mem::replace(&mut self.val, None).unwrap()
    }
}

impl<T> LinkedList<T> {
    fn new(val: T) -> Self {
        Self {
            head: Node::new(val),
        }
    }

    fn push(&mut self, val: T) {
        let mut node: *mut Node<T> = &mut self.head;
        unsafe {
            while let Some(next) = (*node).next {
                node = next;
            }

            (*node).next = Some(Box::into_raw(Box::new(Node::new(val))));
        }
    }

    fn remove(&mut self, index: usize) -> T {
        if index == 0 {
            unsafe {
                // Move the new head from the heap back to the stack
                let mut new_head = *Box::from_raw(
                    self.head
                        .next
                        .expect("Cannot remove element if it results in an empty list."),
                );
                mem::swap(&mut new_head, &mut self.head);
                new_head.next = None;
                new_head.into_inner()
            }
        } else {
            let mut prev: *mut Node<T> = &mut self.head;
            let mut node: *mut Node<T> = &mut self.head;
            let mut i = 0;
            unsafe {
                while i < index {
                    i += 1;
                    if let Some(next) = (*node).next {
                        prev = node;
                        node = next;
                    } else {
                        panic!("Cannot insert beyond the length of the linked list.");
                    }
                }

                let mut removed_node = *Box::from_raw((*prev).next.unwrap());
                removed_node.next = None;
                (*prev).next = (*node).next;

                removed_node.into_inner()
            }
        }
    }
}

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 26"
    }

    fn statement(&self) -> &str {
        STATEMENT
    }

    fn solve(&self, _out: &mut dyn Write) -> Result<(), Error> {
        let mut list = LinkedList::new(0);
        for i in 1..=10 {
            list.push(i);
        }
        for i in 0..10 {
            let li = list.remove(0);
            if i != li {
                Err(format!("Expected {} but got {}.", i, li))?
            }
        }

        let mut list = LinkedList::new(0);
        for i in 1..=10 {
            list.push(i);
        }
        for i in (0..10).rev() {
            let li = list.remove(i);
            if i != li {
                Err(format!("Expected {} but got {}.", i, li))?
            }
        }

        Ok(())
    }
}

#[cfg(all(test, feature = "nightly"))]
mod benches {
    use test::Bencher;
}
