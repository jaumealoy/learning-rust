use std::rc::Rc;

pub trait Stack<T> {
    fn new() -> Self;
    fn push(&mut self, element: T);
    fn pop(&mut self);
    fn peek(&self) -> &T;
    fn is_empty(&self) -> bool;
}

pub struct StackNode<T> {
    element: T,
    next: Option<Rc<StackNode<T>>>
}

pub struct UnlimitedStack<T> {
    top: Option<Rc<StackNode<T>>>
}

impl<T> Stack<T> for UnlimitedStack<T> {
    fn new() -> Self {
        UnlimitedStack {
            top: None
        }
    }

    fn push(&mut self, element: T) {
        let node = StackNode::<T> {
            element: element,
            next: self.top.clone()
        };

        let element = Rc::<StackNode<T>>::new(node);
        self.top = Some(element);
    }

    fn pop(&mut self) {
        match &self.top {
            Some(x) => {
                let next = x.next.clone();
                match next {
                    None => {
                        self.top = None;
                    },

                    Some(y) => {
                        self.top = Some(y.clone());
                    }
                }
            },
            None => ()
        }
    }

    fn peek(&self) -> &T {
        match &self.top {
            Some(x) => &x.element,
            None => panic!("Stack error")
        }
    }

    fn is_empty(&self) -> bool {
        match &self.top {
            None => true,
            Some(_x) => false
        }
    }
}