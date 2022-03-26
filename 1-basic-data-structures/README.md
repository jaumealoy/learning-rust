# Data structures
The goal is to implement the following data structures:
- Stack
- Queue
- Binary tree

And a main program that makes use of them.

# Getting started
This is my first time using Rust. To create a new Rust app we can use the `cargo` package manager using the command `cargo new <folder>` or `cargo init <folder>` if the folder already exists.

This will create an empty project with a `main.rs` file. There we can write our "Hello world" program to check that everything works as expected.

The `main.rs` file should look something like this:
```
fn main() {
    println!("Hello world!")
}
```
Then we can run this program using the command `cargo run` or compile it using `cargo build`.

Now let's get started with the real problem and suppose that we already know the basics of the language (we don't).
If you already have a background in computer science, you would want to create an interface that defines the methods of each data structure.

Creating an interface makes sense as there multiple ways to implement the same data structure. For example, we can create a stack using a linked list or using a static array. Both implementations must have the same behaviour. The implementation of the data structure must remain *transparent* to the programmer who is using it.

Rust is not an object oriented programming language but there are other tools that are similar to interfaces, these are called *traits*. In an OOP language we would create a class to create heterogenic data structures, in Rust, just like C, we can create *structs*.

Another concept that you must know is generics. We don't want to implement a stack for each possible data type. We implement a data structure that must work with every type. We can do it by using a generic.

## Stack
The stack is one of the most common data structures and have multiple uses. The main operation over a stack are:

- *push* which adds an element to the stack
- *pop* which removes the element on the top of the stack

Whether the *pop* operations returns or not the element depends on the implementation, but that does not change much at all. We just can add a *peek* operation to look at the element on the top of the stack.

### Dynamic memory
As we want our stack to hold any amount of element, we cannot use a static array to hold our elements. We must use dynamic memory, the heap.

We will create a stack struct which will be just a pointer to the first stack node. As rust does not have a null pointer we must use the Option type to indicate that it might be a None value. 
If the option has a None value will mean that the stack is empty, otherwise it will Some(x) value where x will be the value.

## Binary tree
Once we have created a dynamic stack, creating new data structures is almost straight forward as we already know the basics.

A binary tree is a data structure made up of nodes, which can have up to 2 children: the left and the right node. However, there is an order relation between the these nodes. Each node must satisfy the following contraints:

- If a node has a left child, it must be lower than thje node.
- If a node has a right child, it must be higher than the node.

```
        5
       / \
      3   7
     / \   \
    1   4   10
```

Example of a valid binary tree.

Now, we can create a new trait and implement it. Let's start by creating a module called `tree` by creating a file called `tree.rs`. 

Again, we will create a generic struct called `BinaryTree`, which will only have one field. This field is the root of the tree and it might empty or not.
Therefore we must use the Option<T> type. 

Wait! Before going on with the implementation of our tree node we must solve one problem. We have said that the elements of the tree create an order relation. Therefore our generic type cannot be *any* generic. The generic type must implement some trait to create this order relation.

Luckily, the standard Rust provides an interface called `PartialOrd` that is used to overload operators such as `<` and `>`. That's enough for us! But... how do we say that the type `T` implements the `PartialOrd` trait? 
Instead of declaring our tree as `BinaryTree<T>` we can declare as `BinaryTree<T: PartialOrd>`. We can add more traits using the following syntax: `BinaryTree<T: PartialOrd + Trait 2 + ... + Trait N>`.

Let's continue creating our tree node. This struct will have 3 fields:

- The node's value
- The left child
- The right child

The left child and the right child must be empty or not. In addition, the can change over time, as new nodes can be added and removed. Therefore, its type should be something like `Option<Rc<RefCell<TreeNode<T>>>>`.

Let's breakdown this type:

- The option enum allows us to have nodes without any child.
- The Rc (reference count) allows us to have multiple owners of the nodes, which help us to implement the operations
- The RefCell allow us to mutate that its data, in this case, the TreeNode.

There might be a simpler way to implement a binary tree, but for now this works.

For now we will just implement the insertion to the tree. There are differents ways to add a node to a tree. To keep things simple, we will use two pointers to our nodes: the parent node and the current node. We will update those variables until we find the right spot for the new value.

In short, we will update the `current` variable until its value is None. When that happens, we will create a new node to parent node. If parent node is None, the new node is the root of the tree.