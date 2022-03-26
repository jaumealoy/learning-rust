# Introduction
In previous chapters we have created some basic data structures and multiple threads! Now, let's take a step back to fully understand the core's of Rust.

The goal of this chapter is to understand ownership, references and smart pointers.

# Ownership
Ownership is the concept that allows Rust compiler to create memory safe programs. The compiler enforces a set of rules, which are:

1. A value has a variable which is its owner.
2. There is only one owner at a time (there cannot be multiple owners of the same value).
3. The value is dropped once the owner goes out of scope.

# References
Programming languages have two ways of passing arguments: by value or by reference.

- By value means that all the data is copied to the stack.
- By reference means that the value being pushed to the stack is a pointer to the actual data.

```
int add(int a, int b) {
    // a and b are passed by value
    return a + b;
}

add(1, 3) -> 4
``` 

``` 
void add(int *a, int *b, int *c) {
    // a, b and c are references to integers
    *c = *a + *b;
}

int a = 1;
int b = 3;
int c;

add(&a, &b, &c) -> c = 4
```

Basic types such as integers, characters and booleans are usually passed by value as they are only a few bytes long. However, large structs or objects must be passed by reference to avoid copying those big structures.

In Rust, basic types (integers, floats, booleans and characters) are passed by value by default. 

When working with references we must remember two rules:

- There can only be one mutable reference o any number of immutable references to a variable at a time.
- All references must be valid.

# Smart pointers
Smart pointers are just like pointers to memory locations with metadata that enables extra features.

This structures usually implement the Deref and Drop traits:

- The Deref only has only one method, which is the `deref`. This method returns a reference to the underlying data. For example, if the smart pointer holds a T data type, the deref function should return a &T type. There is also a DerefMut trait that returns a mutable reference.
- The Drop trait has a method called `drop`. This method is called automatically when a variable goes out of scope. We can manually drop a variable before going out of scope by calling the `drop(variable)` function.

## Box
A Box is a smart pointer which *just* points to a memory address on the heap. A box can only have one owner and its value can be mutated by declaring the variable as mutable.

```
let mut my_box = Box::new(value);
*my_box = new_value;
println!("{0} = {1}", my_value, *my_value);
```

## Rc
A Reference Counter is a smart pointer that keeps track of how many references there are to the data. This allows to have *multiple owners* of the same data. 

This smart pointer does not allow to change the underlying data once created. You only can get immutable references to its data.

## RefCell
We have said that there can only be either a mutable reference to a value or any amount of immutable reference to a value at any time. Usually, this restrictions are checked on compile time. However, there are some scenarios where it is necessary to perform those checks on runtime.

Unlike previous smart pointers, this one does not implement the Deref trait. We must use the method `.borrow()` or `.borrow_mut()` instead. Calling these method may result in a panic if reference constraints are violated.