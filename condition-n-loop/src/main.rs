fn main() {

    let x = 5;

    let y = if x % 5 == 0 {
        10
    } else {
        15
    }; // the semi-colon is due to declaration statement/expression

    let z = if x + y % 5 == 0 {12} else {14}; // better way

    if x + y + z > 2 {
        println!("Cool");
    } // no semicolon req

    /*  Syntax

        if <expression> {
            do something;
        }

    */

    println!("x = {0}, y = {1} z = {2}", x, y, z);

    for j in 1..11 {
        println!("j = {:?}", j);
    }

    /* Syntax:

        for <var> in <sequence> {
            do something;
        }

    */


}
