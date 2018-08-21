// inside type
use std::io::Write;    
use concrete::HasArea;


/// Type Convert
impl HasArea for i32 {
    fn area(&self) -> f64 {
        *self as f64
    }
}

/// Orphan Rule
pub fn write_file() {
    
    let res = std::fs::File::create("foo.txt");
    println!("{:?}", res);
    
    let mut f = std::fs::File::open("foo.txt")
        .expect("Counld not write file");
        
    let buf = b"whatever";
    let result = f.write(buf);
    
    println!("{:?}", result);
}

pub fn test_inside(n: i32) {
    print!("{:?}", n.area());
}

#[cfg(test)]
mod test{
    use inside::test_inside;
    use inside::write_file;
    #[test]    
    fn it_works() {
        test_inside(5);
        write_file();
    }
}
