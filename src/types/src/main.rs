fn main() {
    let x: bool = true;
    // bool, usual in 'if' sentences

    let y: char = 'y';
    // char, 4 char in rust

    let z: i8 = 1;
    // number

    let arr: [&str;3] = ["1", "2", "3"];
    // names: [&str; 3]

    let slice = &arr[..];
    // ref of array

    let (x, y, z) = (1, 2, 3);
    // tumples

    
    println!("{:?}", arr)
}
