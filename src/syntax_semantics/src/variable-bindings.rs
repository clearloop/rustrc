fn main() {
    // variable-bindings
    // define
    let x = 5;
    println!("Hello, world!");

    // Patterns
    let (x, y) = (1, 2);

    // Type annotations
    let x: i32 = 5;

    // Mutability
    let mut x = 5;
    x = 10;

    // Initializing bindings
    let x: i32;
    println!("The valueof x is :{}", x);

    // Scope and Shadowing
    // Scope
    let x: i32 = 17;
    {
        let y: i32 = 3;
        println!("The value of x is {} and value of y is {}", x, y);
    }
    println!("The value of x is {} and value of y is {}", x, y);
    // Shadowing
    let x: i32 = 8;
    {
        println!("{}", x);
        let x = 12;
        println!("{}", x);
    }
    println!("{}", x);
    let x = 42;
    println!("{}", x);
}
