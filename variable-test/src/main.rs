fn main() {
    println!("Testing variables 123 \n");

    let x = 34;

    printit(x);
    printit(0);

    let x = x + 90;

    println!("{}", x + 78);

    let q = sum_all(3, 9);
    printit(q);
}

fn printit(x: i32) {
    println!("{}", x);
}

fn sum_all(x: i32, y: i32) -> i32 {
    x + y // returns sum of two numbers passed
}
