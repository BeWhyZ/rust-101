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


    pub fn peek(&self) -> Option<&T>{
        unsafe {
            self.head.as_ref().map(|node|{
                &node.elem
            })
        }
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        unsafe {
            self.head.as_mut().take().map(|node|{
                &mut node.elem
            })
        }
    }
}

impl<T> Drop for List<T>{
    fn drop(&mut self){
        while let Some(_) = self.pop(){}
    }
}


//iter Only once the iterator is gone can you access the list and call things like push and pop which need to mess with the tail pointer and Boxes. Now, during the iteration we are going to be dereferencing a bunch of raw pointers, so there is a kind of mixing there, but we should be able to think of those references as reborrows of the unsafe pointers.
// into iter

pub struct IntoIter<T>(List<T>);


impl<T> Iterator for IntoIter<T>{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

// iter
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.map(|node|{
                self.next = node.next.as_ref();
                &node.elem
            })
        }
    }
}

// iterMut
pub struct IterMut<'a, T>{
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            self.next.take().map(|node|{
                self.next = node.next.as_mut();
                &mut node.elem
            })
        }
    }
}

impl<T> List<T>{
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn iter(&self) -> Iter<T>{
        unsafe {Iter{next:self.head.as_ref()}}
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        unsafe {IterMut{next:self.head.as_mut()}}
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

    #[test]
    fn miri_food() {
        let mut list = List::new();

        list.push(1);
        list.push(2);
        list.push(3);

        assert!(list.pop() == Some(1));
        list.push(4);
        assert!(list.pop() == Some(2));
        list.push(5);

        assert!(list.peek() == Some(&3));
        list.push(6);
        list.peek_mut().map(|x| *x *= 10);
        assert!(list.peek() == Some(&30));
        assert!(list.pop() == Some(30));

        for elem in list.iter_mut() {
            *elem *= 100;
        }

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&400));
        assert_eq!(iter.next(), Some(&500));
        assert_eq!(iter.next(), Some(&600));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

        assert!(list.pop() == Some(400));
        list.peek_mut().map(|x| *x *= 10);
        assert!(list.peek() == Some(&5000));
        list.push(7);

        // Drop it on the ground and let the dtor exercise itself
    }
}

