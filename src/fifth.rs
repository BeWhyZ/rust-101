// an ok unsafe queue

// using raw pointer for the type, and don't mixed the safe pointer and raw pointer
// The clean way to implement this by using a smart pointer function to convert itself into a raw pointer.
// This transfers the responsibility for the previously managed memory. To release the memory, convert the raw pointer back into a smart pointer,
// and the default drop will correctly handle memory release.
// smart pointer -> raw pointer -> smart pointer
// start with safe stuff, turn into into raw pointers, and then only convert back to safe stuff at the end (when we want to Drop it).

use std::{mem, ptr};

pub struct List<T> {
    head:Link<T>,
    tail: *mut Node<T>,
}


type Link<T> = *mut Node<T>;


struct Node<T> {
    elem:T,
    next:Link<T>,
}

impl<T> List<T>{
    pub fn new()-> Self {
        List { head: ptr::null_mut(), tail: ptr::null_mut() }
    }

    pub fn push(&mut self, elem:T) {
        unsafe {
            let new_tail = Box::into_raw(Box::new(Node{
                elem:elem,
                next:ptr::null_mut(),
            }));

            if !self.tail.is_null(){
                (*self.tail).next = new_tail;
            } else {
                self.head = new_tail;
            }

            self.tail = new_tail
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        unsafe{
            if !self.head.is_null(){
                let head = Box::from_raw(self.head);
                self.head = head.next;

                // also modify the tail
                if self.head.is_null(){
                    self.tail = ptr::null_mut();
                }

                Some(head.elem)

            } else {
                None
            }
        }
    }
}

impl<T> Drop for List<T>{
    fn drop(&mut self){
        while let Some(_) = self.pop(){}
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn basic(){
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);

        // Check the exhaustion case fixed the pointer right
        list.push(6);
        list.push(7);

        // Check normal removal
        assert_eq!(list.pop(), Some(6));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.pop(), None);
    }
}

