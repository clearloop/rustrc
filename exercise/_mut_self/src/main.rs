
use std::borrow::ToOwned;
#[derive(Debug)]
struct H {

    a: String,
}
impl H {
    fn c(mut self) -> H {
        self.a = "2".to_string();
        return self;
    }
}


fn main() {

    let mut _h = H {a: "1".to_string()};
    _h = _h.c();
    println!("{:?}", _h);
}
