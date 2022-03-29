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

    /// can we remove the n-th element from a list using a Box?
    /// if this was allowed, would we have multiple owners of the same box?
    /// let'ts try by just removing adding a remove_first and remove_last operation
    /*
    pub fn remove(&mut self, index: usize) {
        let mut current_index = index - 1;
        let mut current = self.first.as_ref();
        
        while current_index > 0 && current.is_some() {
            current = current.unwrap().next.as_ref();
            current_index -= 1;
        }

        if current_index > 0 {
            panic!("Removing invalid element");
        }

        let node = current.as_deref().unwrap();
        let following = node.next.unwrap().next;
        node.next = following;
    }*/

    pub fn remove_first(&mut self) {
        match &mut self.first {
            None => panic!("Trying to remove an element from an empty list"),
            Some(first) => {
                let value = first.next.take();
                self.first = value;
            }
        }
    }

    /*
    pub fn remove_last(&mut self) {
        match &mut self.first {
            Some(x) => {
                let mut previous: Option<&mut Node<T>> = None;
                let mut current = x.as_mut();
                while current.next.is_some() {
                    previous = Some(current);
                    current = current.next
                        .as_mut()
                        .unwrap()
                        .as_mut();
                }

                drop(current);

                if previous.is_none() {
                    self.first = None;
                } else {
                    previous.as_mut().unwrap().next = None;
                }
            },
            None => ()
        }
    }
    */

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