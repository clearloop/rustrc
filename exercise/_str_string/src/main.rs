fn main() {
    // all of them are &str    
    let a = "Hello";
    let b = "World";

    // &str => String
    let c = a.to_string();
    let d = String::from(b);

    // String => &str
    let e = &String::from("hello, world");
    let _e = String::from("hello, world");
    let _e_str = _e.as_str();
        

    // String + &str = String;
    
}

fn foo(s: &mut String) {

}
