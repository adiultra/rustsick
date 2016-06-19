use std::io;

// All operations happen on i64

/// # input
/// takes input and gives a i64
fn input() -> i64 {
    let mut val = String::new();
    io::stdin().read_line(&mut val).expect("");
    let val: i64 = match val.trim().parse() {
        Ok(num) => num,     // Match num if everything is OK
        Err(_) => 1,        // Set num = 1 if anything != OK happens
                            // This exits the check process and displays invalid
    };
    val
}

fn factor(x: i64) -> Vec<i64> {
    let mut arr = vec![];
    let mut i: f64 = 2.0;
    let mut y = x as f64;
    while y.sqrt().ceil() >= i {
        if y % i == 0.0 {
            y = y/i;
            arr.push(i as i64);
        }
        i += 1.0;
    }

    arr.push(y as i64);
    arr.sort();
    arr
}

fn factorial(x: i64) -> i64 {
    if x > 0 && x < 21 {
        x * factorial(x - 1)
    } else if x > 21 {
        println!("Number too large");
        1
    } else {
        1
    }
}

fn npr(n: i64, r: i64) -> i64 {
    if n * r > 0 &&
        n > r &&
        n < 21 {
        factorial(n)/factorial(r)
    } else if n > 21 {
        println!("Number too large");
        1
    } else {
        1
    }
}

fn ncr(n: i64, r: i64) -> i64 {
    if  n * r > 0 &&
        n > r &&
        n < 21 {
        factorial(n)/(factorial(r)*factorial(n-r))
    } else if n > 21 {
        println!("Number too large");
        1
    } else {
        1
    }
}

#[test]
fn factorials_test() {
    // test functions related to factorials
    assert_eq!(factorial(4), 24);
    assert_eq!(factorial(0), 1);

    assert_eq!(npr(3, 2), 3);
    assert_eq!(npr(1, 0), 1);

    assert_eq!(ncr(4, 2), 6);
    assert_eq!(ncr(6, 2), ncr(6, 4));
}

#[test]
fn factor_test() {
    assert_eq!(factor(12), vec![2, 2, 3]);
}

fn main() {
    let integ = input();
    let vect = factor(integ);

    for fact in vect {
        print!("{ } ", fact);
    }
    print!("\n");

    println!("{0}! \t= {1}", integ, factorial(integ));
    println!("{0}C{1} \t= {2}", integ, 3, ncr(integ, 3));
    println!("{0}P{1} \t= {2}", integ, 3, npr(integ, 3));

}
