/// # Play Ground

struct Todo {
    todo: &'static str,
    index: i32,
}

fn main() {
    print!("This is Play Ground, Edit this code and add more :)");

    let stuff = "Awesome";

    let mut cool = Todo{todo: stuff, index: 0};

    println!("{0} contains - {1}", cool.index, cool.todo);

    cool.todo = "Amazing";

    println!("{0} contains - {1}", cool.index, cool.todo);
}
