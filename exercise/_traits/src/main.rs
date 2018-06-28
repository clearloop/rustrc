struct Rectange<T> {
    x: T,
    y: T,
    width: T,
    height: T,
}

//impl<T: PartialEq> Rectange<T> {
//    fn is_square(&self) -> bool {
//        self.width == self.height
//    }
//}

impl<T: PartialEq> Rectange<T> {
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

fn main() {
    let mut r = Rectange {
        x: 0, y: 0, width: 47, height: 47,
    };
    assert!(r.is_square());
    r.height = 42;
    assert!(!r.is_square());
}
