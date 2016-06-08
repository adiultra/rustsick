struct Todo {
    todo: &'static str,
    index: i32,
}

impl Todo {
    fn print_my_content(&self) {
        println!("{0} . {1}", self.index, self.todo);
    }
}


fn main() {
    let my_todo = Todo {
        todo: "Awesome goes amazing",
        index: 0
    };

    println!("The Todo at {0} contains : {1}", my_todo.index, my_todo.todo);

    my_todo.print_my_content();

}
