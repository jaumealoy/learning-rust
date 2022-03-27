struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Node {
            value: value,
            next: None
        }
    }
}

pub struct List<T> {
    first: Option<Box<Node<T>>>
}

impl<T> List<T> {
    pub fn new() -> Self {
        List {
            first: None
        }
    }

    pub fn add(self: &mut Self, value: T) {
        match &mut self.first {
            Some(first) => {
                // list is not empty, add the element at the end
                // we do not have a pointer to the last element (and using types we cannot!)
                let mut current = first;
                while current.next.is_some() {
                    // we can use safely unwrap as we have checked that current.next is Some
                    current = current.next.as_mut().unwrap();
                }
                current.next = Some(Box::new(Node::new(value)));

                println!("Adding element at the end");
            },
            
            None => {
                // list is empty
                self.first = Some(Box::new(Node::new(value)));
                println!("Adding first element");
            }
        }
    }

    pub fn iter(self: &Self) -> ListIterator<T> {
        ListIterator { 
            current: self.first.as_deref()
        }
    }
}

pub struct ListIterator<'a, T> {
    current: Option<&'a Node<T>>
}

impl<'a, T> std::iter::Iterator for ListIterator<'a, T> {
    type Item = &'a T; 

    fn next(&mut self) -> Option<Self::Item> {
        match self.current {
            Some(x) => {
                self.current = x.next.as_deref();
                Some(&x.value)
            },
            None => None
        }
    }
}

/*
impl<T> std::iter::IntoIterator for List<T> {
    type Item = &T;

    type IntoIter = ListIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIterator {
            current: self.first.as_ref()
        }
    }
}
*/