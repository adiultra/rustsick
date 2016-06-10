fn display(array: &[[i32; 7]; 7]) {
    println!("=============");
    for row in array {
        for element in row {
            print!("{} ", element);
        }
        print!("\n");
    }
    println!("=============");
}

fn main() {
    let mut surface = [
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0]
    ];

    display(&surface);

    surface[0][1] = 9;

    display(&surface);

    loop {
        break;
    }

}
