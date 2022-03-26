use std::rc::Rc;
use std::boxed::Box;
use std::cell::RefCell;

type Link<T> = Option<Rc<RefCell<TreeNode<T>>>>;

pub struct BinaryTree<T: PartialOrd + std::fmt::Display> {
    root: Link<T>
}

struct TreeNode<T: PartialOrd + std::fmt::Display> {
    value: T,
    left: Link<T>,
    right: Link<T>
}

impl<T: PartialOrd + std::fmt::Display> TreeNode<T> {
    fn new(value: T) -> Self {
        TreeNode {
            value: value,
            left: None,
            right: None
        }
    }
}

pub trait Tree<T: PartialOrd> {
    fn new() -> Self;
    fn add(&mut self, value: T) -> ();
}

impl<T: PartialOrd + std::fmt::Display> Tree<T> for BinaryTree<T> {
    fn new() -> Self {
        BinaryTree {
            root: None
        }
    }

    fn add(&mut self, value: T) {
        match &self.root {
            None => {
                self.root = Some(
                    Rc::new(RefCell::new(TreeNode::new(value)))
                )
            },

            Some(root) => {
                let mut parent: Rc<RefCell::<TreeNode<T>>> = root.clone();
                let mut current: Option<Rc<RefCell::<TreeNode<T>>>> = Some(root.clone());
                let mut is_left_child: bool = false;
                
                while let Some(x) = current.clone() {
                    let node = (*x).borrow();

                    parent = x.clone();
                    if value > node.value {
                        is_left_child = false;
                        current = node.right.clone();
                    } else {
                        is_left_child = true;
                        current = node.left.clone();
                    }
                }

                let mut parent = (*parent).borrow_mut();
                if is_left_child {
                    parent.left = Some(Rc::new(RefCell::new(TreeNode::new(value))));
                } else {
                    parent.right = Some(Rc::new(RefCell::new(TreeNode::new(value))));
                }
            }
        }
    }
} 