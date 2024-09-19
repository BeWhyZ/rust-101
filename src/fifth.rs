// an ok unsafe queue

use std::mem;

pub struct List<T> {
    head:Link<T>,
    tail: *mut Node<T>,
}


type Link<T> = Option<Box<Node<T>>>;


struct Node<T> {
    elem:T,
    next:Link<T>,
}

impl<T> List<T>{
    pub fn new()-> Self {
        List { head: None, tail: None }
    }

    pub fn push(&'a mut self, elem:T) {
        let new_tail = Box::new(Node{
            elem:elem,
            next:None
        });
        // mem swap
        // tail push new node
        let new_tail = match self.tail.take(){
            Some(old_tail)=>{
                old_tail.next = Some(new_tail);
                old_tail.next.as_deref_mut()
            },
            None=>{
                self.head = Some(new_tail);
                self.head.as_deref_mut()
            },
        };

        self.tail = new_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head|{
            let head = *head;
            self.head = head.next;

            if self.head.is_none(){
                self.tail = None;
            }

            head.elem
        })

    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn basic(){
        let mut list = List::new();
        
        list.push(1);list.push(2);list.push(3);
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), None);
    }
}

