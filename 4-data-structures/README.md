# Data structures (again)
In chapter 1, we created a simple stack and a binary tree. And it worked. However, I am not satisfied with the result as we didn't care much about pointers nor implemented some traits.

This time we will try to create a list and implement the iterator trait. Then, we will try to implement a binary tree using an alternative insertion algorithm.

## List
A list is a linear data structure in which each element can have a successor element. The list is made up of nodes, which have the element that they are storing and a pointer to its successor.

If we are building a simple one-link list, we can use the Box smart pointer, as there will only be one owner. However, if we were to create a double-linked list, we would have to use another smart pointer.

### Iterator trait
The `Iterator<T>` trait allows to use our type in a loop expression. To implement this trait we only have to implement a method, which is `next()`. We must create a struct (for example `ListIterator`) which holds the state of the iteration. The `next` method will update this struct until there are no more elements.

# Learn more
Learn more Rust by creating more lists: https://rust-unofficial.github.io/too-many-lists/