# Rust Log

A log for remembering stuff I learn in Rust

## Types

There are many many types, those that are important to me are `i32` `i64` `f64`.

```rust
let apple = 45; // immutable(i.e. fixed or unchangable) x // type = i32 default
apple = 34; // Error

let mut mango = 90;  // mutable(i.e. can be changed)
mango = 8; // Correct
```

## I/O

```rust
println!("Hello");  // prints Hello and ends line automatically
print!("Hello");    // prints Hello

let banana = 26;
println!("Hi, {} Bananas are left", banana); // Replaces {} with value of `banana` i.e. 26
```

## Conditional

### `if`

```rust
if 3 > 2 {
    print!("I am feeling giddy");
}
```

### `if ... else ...`

If condition of if is false then else is executed

```rust
if 3 < 5 {
    println!("How do you do");
} else {
    println!("You might be doing fine");
}
```


## Looping

### `loop {}` for infinite looping

```rust
loop {
    print!("Infinite");
}
```

you can break out of it using `break;`

```rust
let mut x = 0;
loop {
    x += 1;
    if x > 2 {
        break;
    }
}
```

## Functions

```rust
fn main() {
    // Main function
    // code here is executed first
}
```

for example:

```rust
fn main() {
    println!("Hello world");
}
```

```rust
fn Any_Fxn(/* takes this */) -> /* returns this */ {
    // doing this
}
```
