use std::io;
struct Todo {
    todo: String,
    index: i32,
}

impl Todo {
    fn print_my_content(&self) {
        println!("{0} . {1}", self.index, self.todo);
    }
}

struct List {
    list: &'static mut Vec<Todo>,
}

impl List {
    fn add_todo(&self) {
        let mut content = String::new();

        io::stdin().read_line(&mut content)
            .expect("Failed to read line");

        let new: Todo = Todo{todo: content, index: 0};

        self.list.push(new);
    }
}

fn main() {

}
