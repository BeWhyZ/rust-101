// shared ownership by writing a persistent immutable singly-linked list
use std::rc::Rc;

pub struct List<T>{
    // opt by option
    head:Link<T>,
}

type Link<T> = Option<Rc<Node<T>>>;


// def Node
struct Node<T>{
    elem:T,
    next:Link<T>,
}

impl<T> List<T>{
    pub fn new()->Self{
        List { head: None }
    }

    // prepend multi value has ownership, &value
    // Let's start with prepending. It takes a list and an element, and returns a List. 
    pub fn prepend(& self, elem:T)->List<T>{
        List { head: Some(Rc::new(
            Node { elem: elem, next: self.head.clone() }
        )) }
    }


    // takes a list and return whole list with the first element removed
    pub fn tail(&self) -> List<T> {
        List { head: self.head.as_ref().and_then(|node|{
           node.next.clone()
        }) }
    }

    //we should probably provide head, which returns a reference to the first element.
    pub fn head(&self)->Option<&T>{
        self.head.as_ref().map(|node| &node.elem)
    }

}


// iter
pub struct Iter<'a, T>{
    next:Option<&'a Node<T>>,
}

impl<T> List<T> {
    pub fn iter(&self) -> Iter<T>{
        Iter{
            next:self.head.as_deref(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T>{
    type Item = &'a T;

    fn next(&mut self)->Option<Self::Item>{
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

// drop
impl<T> Drop for List<T>{
    fn drop(&mut self){
        let mut head = self.head.take();
        // will break when head is None
        while let Some(node) = head{
            if let Ok(mut node) = Rc::try_unwrap(node){
                // node
                head = node.next.take()
            }else{
                break
            }
        }
    }
}



#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn basic(){
        let list = List::new();

        assert_eq!(list.head(), None);
        let list = list.prepend(1).prepend(2).prepend(3);
        assert_eq!(list.head(), Some(&3));

        let list = list.tail();
        assert_eq!(list.head(), Some(&2));

        let list = list.tail();
        assert_eq!(list.head(), Some(&1));


        let list = list.tail();
        assert_eq!(list.head(), None);
        let list = list.tail();
        assert_eq!(list.head(), None);
    }

    #[test]
    fn iter(){
        let list = List::new();
        let list = list.prepend(1).prepend(2).prepend(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

}




