# Asynchronous programming
Some tasks take some time to be completed and there is nothing that the processor can do while waiting. Asynchronous programming allows to perform other tasks while a blocking task is being executed.

In Rust, we have the Future trait that represents a value that might have not finished computing yet. Rust only sets how asynchrounous operations are created and its type, but it doesn't set how tasks are executed. This details are left to community libraries such as Tokio.

Unlike JavaScript, tasks (Promises in JS) are not executed after its creation. They must be polled in orde to be scheduled.

```
new Promise(() => console.log("Hello world"))
```

This previous JS code would print "Hello world" but its equivalent in Rust wouldn't do it, as the Promise (or the Future) has not been polled.

## async / await keywords
`async` keyword is used to indicate that a function is a task. `await` keyword is used to wait for a task completion.