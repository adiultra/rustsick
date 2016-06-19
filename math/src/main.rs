use std::io;

/// # input
/// takes input and gives a usize integer
fn input() -> f64 {
    // integer is a mutable empty string
    let mut integer = String::new();

    // Get input from user in integer
    io::stdin().read_line(&mut integer)
        .expect("Failed to read line");

    // Convert and shadow(displace) integer into a usize integer
    let integer: f64 = match integer.trim().parse() {
        Ok(num) => num,     // Match num if everything is OK
        Err(_) => 1.0,        // Set num > 7 if anything != OK happens
                            // This exits the check process and displays invalid
    };

    integer
}

fn factor(mut x: f64) -> Vec<f64> {
    let mut arr = vec![];
    let mut i: f64 = 2.0;

    while x.sqrt().ceil() >= i {
        if x % i == 0.0 {
            x = x/i;
            arr.push(i);
        }
        i += 1.0;
    }

    arr.push(x);

    arr
}

fn main() {
    let integ = input();
    let vect = factor(integ);

    for fact in vect {
        print!("{ } ", fact);
    }
    print!("\n");
}
