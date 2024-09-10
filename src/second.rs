use std::mem;

pub struct List<T>{
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem:T,
    next:Link<T>,
}

// T using in all these impls
impl<T> List<T> {
    pub fn new()->Self{
        List { head: None }
    }

    pub fn push(&mut self, elem: T){
        let new_node = Box::new(Node {
            elem,
            next:self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self)->Option<T>{
        self.head.take().map(|node|{
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self)->Option<&T>{
        self.head.as_ref().map(|node|{
            &node.elem
        })
    }

    pub fn peek_mut(&mut self)->Option<&mut T>{
        self.head.as_mut().map(|node|{
            &mut node.elem
        })
    }
}

// implement Drop
impl<T> Drop for List<T>{
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link{
            cur_link = boxed_node.next.take();
        }
    }
}


// implement IntoIter T
pub struct IntoIter<T>(List<T>);

impl<T> List<T>{
    pub fn into_iter(self)->IntoIter<T>{
        IntoIter(self)
    }
}

// implement iter for struct
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self)->Option<Self::Item>{
        self.0.pop()
    }
}


// implement iter &T

pub struct Iter<T>{
    next:Option<&Node<T>>,
}

impl<T> List<T>{
    pub fn iter(&self)->Iter<T>{
        Iter{ next:self.head.map(|node|{&node}) }
    }
}

impl<T> Iterator for Iter<T>{
    type Item = T;
    fn next(&mut self)->Option<Self::Item>{
        self.next.map(|node|{
            self.next = node.next.map(|node|{&node});
            &node.elem
        })
    }
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn basis(){
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek(){
        let mut list = List::new();
        list.push(1);list.push(2);list.push(3);


        assert_eq!(list.peek(), Some(&3));
        list.pop();

        list.peek_mut().map(|node|{
            *node = 42
        });
        assert_eq!(list.peek_mut(), Some(&mut 42));
        assert_eq!(list.peek(), Some(&42));
    }


    #[test]
    fn into_iter(){
        let mut list = List::new();
        list.push(1);list.push(2);list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);

    }
}

