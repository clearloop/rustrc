fn main() {
    let v1:Vec<i32> = vec![1, 2, 3];
    let v2:Vec<i32> = vec![1, 2, 3];
    let (v1, v2, answer) = foo(v1, v2);
    println!("Hello, world!");
}

fn foo(v1: Vec<i32>, v2: Vec<i32>) -> (Vec<i32>, Vec<i32>, i32) {
    (v1, v2, 45)
}

// 这不是理想的 rust 代码，它没有利用'借用'这个编程语言的特点..... what the fuck of 'borrowed'.
