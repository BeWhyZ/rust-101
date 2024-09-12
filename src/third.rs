use std::rc::Rc;

pub struct List<T>{
    // opt by option
    head:Link<T>,
}

type Link<T> = Option<Rc<Node<T>>> ;


// def Node
struct Node<T>{
    elem:T,
    next:Link<T>,
}

impl<T> List<T>{
    fn new()->Self{
        List { head: None }
    }
}

