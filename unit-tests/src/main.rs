fn is_prime(number: i32) -> i32 {
    let mut div = 2;
    let mut prime = 1;
    loop {
        if div < number {
            if number % div == 0 {
                prime = 0;
            }
        } else {
            break;
        }
        div += 1;
    }
    prime
}

#[test]   // tests start with this
fn test_is_prime() {
    assert_eq!(is_prime(2 * 5), 0);     // assert_eq!(func(), return_Val)
    assert_eq!(is_prime(7), 1);
}

fn main() {
    println!("{0} is prime = {1}", 34, is_prime(34));
}
