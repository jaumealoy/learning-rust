mod list;
use list::List;

fn main() {
    // start by creating a new list
    let mut my_list = List::<i32>::new();
    my_list.add(0);
    my_list.add(1);
    my_list.add(2);

    for element in my_list.iter() {
        println!("Element: {0}", element);
    }
}
