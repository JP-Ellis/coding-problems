use crate::{Error, Problem};
use std::io::prelude::*;
use std::ptr::{null, null_mut};

pub struct P;

const STATEMENT: &str = r#"An XOR linked list is a more memory efficient doubly
linked list. Instead of each node holding next and prev fields, it holds a field
named both, which is an XOR of the next node and the previous node. Implement an
XOR linked list; it has an add(element) which adds the element to the end, and a
get(index) which returns the node at index.

If using a language that has no pointers (such as Python), you can assume you
have access to get_pointer and dereference_pointer functions that converts
between nodes and memory addresses."#;

/// Xor two pointers
fn xor<T>(a: *const T, b: *const T) -> *mut T {
    ((a as usize) ^ (b as usize)) as *mut T
}

#[derive(Debug)]
struct XorNode<T> {
    val: T,
    both: *mut XorNode<T>,
}

impl<T> XorNode<T> {
    /// Create a new head-node that is not linked to anything
    fn new(val: T) -> Self {
        XorNode {
            val,
            both: null_mut(),
        }
    }

    /// Go to the next node
    fn next(&self, prev: *const XorNode<T>) -> *const XorNode<T> {
        xor(prev, self.both)
    }

    /// Go to the next node
    fn next_mut(&mut self, prev: *const XorNode<T>) -> *mut XorNode<T> {
        xor(prev, self.both)
    }

    fn get(&self, i: usize) -> Option<&XorNode<T>> {
        let mut prev = null();
        let mut current: *const XorNode<T> = self;
        let mut next = unsafe { (*current).next(prev) };
        let mut idx = 0;
        while !next.is_null() && idx < i {
            idx += 1;
            unsafe {
                next = (*current).next(prev);
                prev = current;
                current = next;
            }
        }

        if idx == i {
            unsafe { current.as_ref() }
        } else {
            None
        }
    }

    fn get_mut(&mut self, i: usize) -> Option<&mut XorNode<T>> {
        let mut prev = null_mut();
        let mut current: *mut XorNode<T> = self;
        let mut next = unsafe { (*current).next_mut(prev) };
        let mut idx = 0;
        while !next.is_null() && idx < i {
            idx += 1;
            unsafe {
                next = (*current).next_mut(prev);
                prev = current;
                current = next;
            }
        }

        if idx == i {
            unsafe { current.as_mut() }
        } else {
            None
        }
    }

    fn traverse(&mut self) -> (*mut XorNode<T>, *mut XorNode<T>) {
        let mut prev = null_mut();
        let mut current: *mut XorNode<T> = self;
        let mut next = unsafe { (*current).next_mut(prev) };
        while !next.is_null() {
            unsafe {
                prev = current;
                current = next;
                next = (*current).next_mut(prev);
            }
        }

        (prev, current)
    }

    fn push(&mut self, val: T) {
        let (prev, current) = self.traverse();

        let next = Box::into_raw(Box::new(XorNode::new(val)));
        unsafe {
            current.as_mut().unwrap().both = xor(prev, next);
            next.as_mut().unwrap().both = xor(current, null());
        }
    }
}

impl<T> Drop for XorNode<T> {
    fn drop(&mut self) {
        unsafe {
            let prev = null_mut();
            let current: *mut XorNode<T> = self;
            let next = (*current).next_mut(prev);
            if !next.is_null() {
                // Drop the next node as well, but set it's prev node to null as
                // it is the new head.
                (*next).both = xor((*next).both, current);
                let _to_be_dropped = Box::from_raw(next);
            }
        }
    }
}

impl Problem for P {
    fn name(&self) -> &str {
        "Daily Coding Problem 6"
    }

    fn statement(&self) -> &str {
        STATEMENT
    }

    fn solve(&self, _out: &mut dyn Write) -> Result<(), Error> {
        let mut head = XorNode::new(0);

        let max = 1000;
        for i in 1..max {
            head.push(i);
        }

        for i in 0..max {
            if let Some(node) = head.get(i) {
                if node.val != i {
                    return Err(format!("Error in getting node {}, got {}.", i, node.val))?;
                }
            } else {
                return Err(format!("Unable to get node {}.", i))?;
            }
        }

        for i in 0..max {
            if let Some(node) = head.get_mut(i) {
                node.val *= 2;
            } else {
                return Err(format!("Unable to get_mut node {}.", i))?;
            }
        }

        for i in 0..max {
            if let Some(node) = head.get(i) {
                if node.val != 2 * i {
                    return Err(format!("Error in getting node {}, got {}.", i, node.val))?;
                }
            } else {
                return Err(format!("Unable to get node {}.", i))?;
            }
        }

        Ok(())
    }
}

#[cfg(all(test, feature = "nightly"))]
mod benches {
    use test::Bencher;

    macro_rules! push {
        ($f:ident, $n:expr) => {
            #[bench]
            fn $f(b: &mut Bencher) {
                b.iter(|| {
                    let mut head = super::XorNode::new(0);
                    for i in 0..2usize.pow($n) {
                        head.push(i);
                    }
                })
            }
        };
    }

    macro_rules! get {
        ($f:ident, $n:expr) => {
            #[bench]
            fn $f(b: &mut Bencher) {
                let mut head = super::XorNode::new(0);
                for i in 0..2usize.pow($n) {
                    head.push(i);
                }

                b.iter(|| {
                    for i in 0..2usize.pow($n) {
                        test::black_box(head.get(i));
                    }
                })
            }
        };
    }

    push!(push_01, 1);
    push!(push_02, 2);
    push!(push_03, 3);
    push!(push_04, 4);
    push!(push_05, 5);
    push!(push_06, 6);
    push!(push_07, 7);
    push!(push_08, 8);
    push!(push_09, 9);
    push!(push_10, 10);

    get!(get_01, 1);
    get!(get_02, 2);
    get!(get_03, 3);
    get!(get_04, 4);
    get!(get_05, 5);
    get!(get_06, 6);
    get!(get_07, 7);
    get!(get_08, 8);
    get!(get_09, 9);
    get!(get_10, 10);
}
