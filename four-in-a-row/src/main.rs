use std::io;

/// # display
/// displays the surface of the game
fn display(array: &[[i32; 7]; 7]) {
    println!("0 1 2 3 4 5 6");

    for row in array {
        for element in row {
            match element {
                &1 => print!("x "),
                &2 => print!("o "),
                _ => print!("  ")
            }
        }

        print!("\n");
    }

    println!("=============");
}

fn input() -> [usize; 2] {
    // integer is a mutable empty string
    let mut integer = String::new();
    let mut integer2 = String::new();

    // Get input from user in integer
    io::stdin().read_line(&mut integer)
        .expect("Failed to read line");

    io::stdin().read_line(&mut integer2)
        .expect("Failed to read line");

    // Convert and shadow(displace) integer into a 32bit integer
    let integer: usize = match integer.trim().parse() {
        Ok(num) => num,         // Match num if everything is OK
        Err(_) => 0,     // Continue even if anything != OK happens
    };
    let integer2: usize = match integer2.trim().parse() {
        Ok(num) => num,         // Match num if everything is OK
        Err(_) => 0,     // Continue even if anything != OK happens
    };

    [integer, integer2]
}

fn check(indices: &[usize; 2]) -> bool {
    let mut condition = false;
    if 7 > indices[0] {
        if 7 > indices[0] {
            condition = true;
        }
    }

    condition
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

    let mut chance = 0;


    for _ in 1..10 {
        // Display the game surface
        display(&surface);

        // Ask for coloumn v and row >
        println!("Enter the row > and coloumn v");
        let index: [usize; 2] = input();

        if check(&index) {
            // Change element to 1 if chance is 0
            if chance == 0 {
                surface[index[0]][index[1]] = 1;
                chance = 1;
            }

            // otherwise change element to 2
            else {
                surface[index[0]][index[1]] = 2;
                chance = 0;
            }
        }

        else {
            println!("Wrong Indexes, Try again");
        }
    }

}
