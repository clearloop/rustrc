fn main() {
    print_number(5);
}

fn print_number(x: i32) {
    println!("x is: {}", x);
}

// expression and sentence;
fn add_one(x: i32) -> i32 {
    return x + 1;
}

// early returns;
fn foo(x: i32) -> i32 {
    return x + 1;
}

// Diverging functions
fn diverges() -> ! {
    panic!("This function never returns!")
}

// Point
let f: fn(i32) -> i32 = plus_one;
let f = plus_one;
