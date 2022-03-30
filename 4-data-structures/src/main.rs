mod list;
use list::List;
use std::fmt::Display;

fn print_list<T>(list: &List<T>) 
where T: Display
{
    println!("There are {0} elements", list.count());

    for element in list.iter() {
        println!("Element: {0}", element);
    }

    println!("==========");
}


fn main() {
    // start by creating a new list
    let mut my_list = List::<i32>::new();
    my_list.add(0);
    my_list.add(1);
    my_list.add(2);
    my_list.add(3);
    my_list.add(5);

    print_list(&my_list);

    my_list.remove_last();
    my_list.remove(1);

    print_list(&my_list);

    my_list.remove_first();
    my_list.remove_first();

    print_list(&my_list);
}
