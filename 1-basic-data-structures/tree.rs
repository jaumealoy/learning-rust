use std::rc::Rc;

use crate::stack::UnlimitedStack;
use crate::stack::Stack;

pub struct BinaryTree<T: PartialOrd> {
    root: Option<Rc<TreeNode<T>>>
}

struct TreeNode<T: PartialOrd> {
    value: T,
    left: Option<Rc<TreeNode<T>>>,
    right: Option<Rc<TreeNode<T>>>
}

pub trait Tree<T: PartialOrd> {
    fn new() -> Self;
    fn add(&mut self, value: T) -> ();
}

impl<T: PartialOrd> Tree<T> for BinaryTree<T> {
    fn new() -> Self {
        BinaryTree {
            root: None
        }
    }

    fn add(&mut self, value: T) {
        match &self.root {
            None => {
                self.root = Some(
                    Rc::new(
                        TreeNode {
                            value: value,
                            left: None,
                            right: None
                        }
                    )
                )
            },

            Some(root) => {
                let mut stack = UnlimitedStack::<Rc<TreeNode<T>>>::new();
                stack.push(root.clone());

                let mut parent: Option<Rc<TreeNode<T>>> = None;
                
                while !stack.is_empty() {
                    let current = stack.peek();
                    
                    if value > current.value {
                        match &current.right {
                            Some(x) => {
                                parent = Some(current.clone());
                                
                                let r = x.clone();
                                stack.push(r);
                            }
                            _ => ()
                        } 
                    } else {
                        // <= 
                        match &current.left {
                            Some(x) => {
                                parent = Some(current.clone());
                                
                                let r = x.clone();
                                stack.push(r);
                            }
                            _ => ()
                        } 
                    }
                }

                match parent {
                    None => {
                        // new node is the root
                        self.root = Some(
                            Rc::new(
                                TreeNode {
                                    left: None,
                                    right: None,
                                    value: value
                                }
                            )
                        )
                    },

                    Some(x) => {
                        // is this new node the left child or the right child?
                    }
                }
            }
        }
    }
}