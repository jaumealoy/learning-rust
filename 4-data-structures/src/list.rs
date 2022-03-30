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
    It's impossible to remove the last element using two pointers as we would
    end with two mutable references: previous and current, and we could get
    a mutable reference to current from previous (and then we would have two 
    mutable references to the same data)

    pub fn remove_last(&mut self) {
        match &mut self.first {
            Some(x) => {
                let mut current = x;
                let mut previous = ...;
                while let Some(next) = current.next.as_mut() {
                    
                    current = next;
                }
            },
            None => panic!("Trying to remove and element from an empty list")
        }
    }
    */

    pub fn remove_last(&mut self) {
        self.remove(self.count() - 1);
    }

    pub fn remove(&mut self, index: usize) {
        let mut current = &mut self.first;
        let mut remaining = index;
        
        while remaining > 0 {
            remaining -= 1;

            current = match current.as_mut() {
                Some(x) => &mut x.next,
                None => panic!("Index out of range (I)")
            };
        }

        // take the ownership of the previous node
        match current.take() {
            Some(x) => {
                *current = x.next;
            },
            None => panic!("Index out of range (II)")
        }
    }

    pub fn count(&self) -> usize {
        let mut current = &self.first;
        
        let mut length = 0;
        while current.is_some() {
            current = &current.as_ref()
                .unwrap()
                .next;

            length += 1;
        }

        length
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