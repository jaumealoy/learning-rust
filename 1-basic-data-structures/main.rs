mod stack;
mod tree;

use stack::Stack;
use tree::Tree;

use std::boxed::Box;
use std::rc::Rc;
use std::cell::RefCell;

fn print_stack_status(stack: &stack::UnlimitedStack::<i32>) {
    if stack.is_empty() {
        println!("stack is empty")
    } else {
        println!("stack is not empty")
    }
}

fn main() {
    let mut stack = stack::UnlimitedStack::<i32>::new();

    // stack
    stack.push(0);
    stack.push(1);
    stack.push(2);
    
    print_stack_status(&stack);

    while !stack.is_empty() {
        println!("Element at the top {}", stack.peek());
        stack.pop();
    }

    print_stack_status(&stack);

    // boxes
    let mut my_box = Box::new(5); // create a pointer to an integer allocated on the heap
    println!("my_box value = {0}", *my_box);

    let value = my_box.as_mut(); // a reference to the value as mutable (we can edit the box value)
    *value = 7;
    println!("my_box value = {0}", *value);

    println!("my_box value = {0}", *my_box);

    // reference counters (value is immutable)
    let my_reference = Rc::new(5);
    println!("my_reference = {0}", *my_reference);

    // reference cell (value is mutable)
    let my_reference_cell = RefCell::new(5);

    // tree
    let mut tree = tree::BinaryTree::<i64>::new();
    tree.add(100);
    tree.add(150);
    tree.add(75);
    tree.add(25);
    tree.add(10);

    println!("Tree 1 is completed");
}