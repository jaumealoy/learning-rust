use std::borrow::Borrow;
use std::ops::DerefMut;
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    // pass by value or reference?
    let a = 5;
    let simple_tuple = (false, 5, 'a');

    // simple_tuple is passed by value as all its types can be passed by value
    // therefore, we are not actually moving the ownership of the data
    test_pass_by_value(a, simple_tuple);

    // pass by reference
    let mut b: i32 = 7;
    let mut another_tuple = simple_tuple; 
    test_pass_by_reference(&b, &simple_tuple);
    println!("b = {0}", b);

    // we can now borrow as mutable (get a mutable reference) as the variable b is
    // not used further down
    let mut mutable_b = &mut b;
    let mut mutable_tuple = &mut another_tuple;
    println!("before, b = {0}", mutable_b);
    println!("before tuple = ({0}, {1}, {2})", mutable_tuple.0, mutable_tuple.1, mutable_tuple.2);
    test_pass_by_mutable_reference(mutable_b, mutable_tuple);
    println!("after, b = {0}", mutable_b);
    println!("after tuple = ({0}, {1}, {2})", mutable_tuple.0, mutable_tuple.1, mutable_tuple.2);

    // Smart Pointers
    // Box
    let mut my_box = Box::new(22);
    println!("my_box = {0}", my_box); // my_box is automatically dereferenced
    // to change its value, we must declare the variable as mutable
    *my_box += 1;
    println!("my_box = {0}", *my_box); // although we can dereference it explicitly

    let mut the_same_box = my_box.deref_mut();
    *the_same_box += 1;
    println!("the_same_box = {0}", *the_same_box);

    // two mutable references at the same time!
    //println!("the_same_box = {0} = {1}", *the_same_box, *my_box);

    let mut the_same_box_again = my_box.deref_mut();
    *the_same_box_again += 1; // this is allowed as neither my_box or the_same_box are used below

    // code won't compile if the following line is not commented:
    // there will be 3 mutable references to the same data!
    //println!("the_same_box = {0} = {1} = {2}", *the_same_box, my_box, the_same_box_again);

    // Reference counter
    let my_rc = Rc::new(22);
    println!("my_rc = {0} = {1}", my_rc, *my_rc);
    // *my_rc += 1; // not allowed as Rc does not implement the DerefMut trait
    let my_rc_clone = my_rc.clone(); // this will increase the reference count
    println!("my_rc_clone = {0} - reference count: {1}", my_rc_clone, Rc::strong_count(&my_rc_clone));
    drop(my_rc_clone);
    println!("my_rc = {0} - reference count = {1}", my_rc, Rc::strong_count(&my_rc));

    // RefCell
    let my_cell = RefCell::new(22);
    // println!("my_cell = {0}", my_cell); // it does not implement the Deref trait
    println!("my_cell = {0}", my_cell.borrow());
    // the reference borrowed above is moved into the print function and then is dropped

    {
        // we are borrowing immutable references
        let my_ref_a = my_cell.borrow();
        let my_ref_b = my_cell.borrow();

        // borrowing a mutable reference will panic as there are two immutable references
        // let my_ref_c = my_cell.borrow_mut();
    }

    let mut my_ref_d = my_cell.borrow_mut();
    *my_ref_d += 1;

    println!("my_cell = {0}", my_ref_d);

    // if we try borrowing a mutable reference, it will panic as my_ref_d is in scope
    // let mut my_ref_e = my_cell.borrow_mut();
}

fn test_pass_by_value(number: i32, simple_tuple: (bool, i32, char)) {
    println!("Number = {0}", number);
    println!("Tuple = ({0}, {1}, {2})", simple_tuple.0, simple_tuple.1, simple_tuple.2);

    // we cannot modify the value of simple_tuple as it is not mutable
    // simple_tuple.0 = true;
}

fn test_pass_by_reference(number: &i32, simple_tuple: &(bool, i32, char)) {
    // Rust automatically dereferences the references
    println!("Number = {0}", number);
    println!("Tuple = ({0}, {1}, {2})", simple_tuple.0, simple_tuple.1, simple_tuple.2);
}

fn test_pass_by_mutable_reference(number: &mut i32, simple_tuple: &mut (bool, i32, char)) {
    *number += 1;

    // we can change one field at a time
    // in this case, Rust dereferences automatically the field
    simple_tuple.0 = !simple_tuple.0;

    // or change the whole tuple
    // we must use the * operator to tell Rust to change the data instead of 
    // the reference value.
    *simple_tuple = (true, 1337, 'b')
}