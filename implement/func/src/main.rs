fn main() {
    print_sum(5, 6);
    println!("5 -> add_one return {}", add_one(5));
    // diverges();
    println!("{}", )
}

fn print_sum(x: i32, y: i32) {
    println!("sum is: {}", x + y);
}

fn add_one(x: i32) -> i32 {
    x + 1
    // this is pure expression, no ';'?
}

fn diverges() -> ! {
    panic!("This function never returns!")
}

