mod stack;
mod tree;

use stack::Stack;
use tree::Tree;

fn print_stack_status(stack: &stack::UnlimitedStack::<i32>) {
    if stack.is_empty() {
        println!("stack is empty")
    } else {
        println!("stack is not empty")
    }
}

fn main() {
    let mut stack = stack::UnlimitedStack::<i32>::new();

    stack.push(0);
    stack.push(1);
    stack.push(2);
    
    print_stack_status(&stack);

    while !stack.is_empty() {
        println!("Element at the top {}", stack.peek());
        stack.pop();
    }

    print_stack_status(&stack);


    let mut tree = tree::BinaryTree::<i64>::new();
}