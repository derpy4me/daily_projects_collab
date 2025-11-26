
#[derive(Debug)]
struct MyItem<T> {
    item: T,
    logs: Vec<String>
}


fn my_option<T>(mut my_item: MyItem<T>) -> MyItem<T>{
    my_item.logs.push("made it to my option".to_string());
    my_item
}

fn my_monad<T>(my_item: &mut MyItem<T>) {
    my_item.logs.push("made it to my_monad".to_string())
}


fn my_result<T>(my_item: &mut MyItem<T>) -> Result<&mut MyItem<T>, &'static str> {
    my_item.logs.push("made it to my_result".to_string());

    Ok(my_item)
}

fn my_func() -> Option<i32> {
    None
}

fn main() {
    let mut my_item = MyItem{
        item: 1,
        logs: vec![]
    };
    my_monad(&mut my_item);
    println!("{:?}", my_item);
    let mut my_new_item = my_option(my_item);
    println!("{:?}", my_new_item);
    let the_result = my_result(&mut my_new_item).unwrap();
    println!("{:?}", the_result);
}
