
fn main() {
    println!("Data Structures 1");
    println!("Stack: $cargo run --example stack");
    println!("Queue: $cargo run --example queue");
    println!("Queue: $cargo run --example queue-rc");

    println!("Data Structures 2");
    println!("Linked list: $cargo run --example linked-list");
    println!("Linked list: $cargo run --example linked-list-enum");
    println!("Linked list: $cargo run --example linked-list-generic");

    // Переписать реализации циклических структур используя Rc<RefCell<T>> и Weak
    // https://ricardomartins.cc/2016/06/08/interior-mutability
}

// бинарный поиск https://shane-o.dev/blog/binary-search-rust-part-2