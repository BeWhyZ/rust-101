// a bad safe deque
use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};

pub struct List<T>{
    // opt by option
    head:Link<T>,
    tail:Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;


// def Node
pub struct Node<T>{
    elem:T,
    prev:Link<T>,
    next:Link<T>,
}

impl<T> Node<T>{
    pub fn new(elem:T) -> Rc<RefCell<Node<T>>>{
        Rc::new(RefCell::new(Node { elem: elem, prev: None, next: None }))
    }
}


impl<T> List<T>{
    pub fn new()->Self{
        List { head: None , tail:None,}
    }

    // push elem to the head
    pub fn push_front(&mut self, elem: T){
        let new_node = Node::new(elem);
        match self.head.take(){
            Some(old_node)=> {
                old_node.borrow_mut().prev = Some(new_node.clone());
                new_node.borrow_mut().next = Some(old_node);
                self.head = Some(new_node)
            },
            None=>{
                self.head = Some(new_node.clone());
                self.tail = Some(new_node)
            },
        }
    }

    // pop an elem from the head
    pub fn pop_front(&mut self)-> Option<T>{
        self.head.take().map(|pop_node|{
            // checkout the next
            match pop_node.borrow_mut().next.take(){
                Some(next_node)=>{
                    next_node.borrow_mut().prev.take();
                    self.head = Some(next_node);

                    // self.head = Some(next_node.clone());
                    // next_node.borrow_mut().prev.take();
                },
                None=>{
                    // self.tail = None
                    self.tail.take();
                },
            }
            // pop_node.into_inner().elem
            Rc::try_unwrap(pop_node).ok().unwrap().into_inner().elem
        })
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        // Ref and RefMut implement Deref and DerefMut respectively. 
        // So for most intents and purposes they behave exactly like &T and &mut T. 
        // However, because of how those traits work, the reference that's returned is connected to the lifetime of the Ref, and not the actual RefCell. 
        // This means that the Ref has to be sitting around as long as we keep the reference around.
        // But as soon as we return the reference from peek, the function is over and the Ref goes out of scope.
        self.head.as_ref().map(|node|{
            // &node.borrow().elem
            Ref::map(node.borrow(), |node| &node.elem)
        })
    }


    pub fn push_back(&mut self, elem:T) {
        let new_node = Node::new(elem);

        match self.tail.take(){
            Some(old_node)=>{
                old_node.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(old_node);
                self.tail = Some(new_node)
            },
            None=>{
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            },
        }
    }

    pub fn pop_back(&mut self) -> Option<T>{
        self.tail.take().map(|pop_node|{
            match pop_node.borrow_mut().prev.take(){
                Some(next_node)=>{
                    next_node.borrow_mut().next.take();
                    self.tail = Some(next_node);
                },
                None=>{
                    self.head.take();
                },
            }
            Rc::try_unwrap(pop_node).ok().unwrap().into_inner().elem
        })
    }

    pub fn peek_back(&self)->Option<Ref<T>>{
        self.tail.as_ref().map(|node| {
            Ref::map(node.borrow(), |node|{
                &node.elem
            })
        })
    }

    pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail.as_ref().map(|node|{
            RefMut::map(node.borrow_mut(), |node|{
                &mut node.elem
            })
        })
    }

    pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        self.head.as_ref().map(|node|{
            RefMut::map(node.borrow_mut(), |node|{
                &mut node.elem
            })
        })
    }

}

impl<T> Drop for List<T>{
    fn drop(&mut self){
        while self.pop_front().is_some(){}
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic(){
        let mut list = List::new();
        assert_eq!(list.pop_front(), None);

        list.push_front(1);list.push_front(2);list.push_front(3);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);

        list.push_back(1);list.push_back(2);list.push_back(3);
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));

        assert_eq!(&*list.peek_back().unwrap(), &1);
        list.push_front(2);
        assert_eq!(&*list.peek_back().unwrap(), &1);
        assert_eq!(&*list.peek_front().unwrap(), &2);

    }

    #[test]
    fn peek(){
        let mut list = List::new();
        list.push_front(1);list.push_front(2);list.push_front(3);
        assert_eq!(&*list.peek_front().unwrap(), &3);

        assert_eq!(list.pop_front(), Some(3));

        assert_eq!(&*list.peek_front().unwrap(), &2);

        assert_eq!(&*list.peek_front_mut().unwrap(), &mut 2);
        assert_eq!(&*list.peek_back().unwrap(), &1);
        assert_eq!(&*list.peek_back_mut().unwrap(), &mut 1);
    }


}


