use concrete::HasArea;

pub struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}


impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

pub fn test_concrete() {
    let c = Circle {
        x: 0.0_f64,
        y: 0.0_f64,
        radius: 1.0_f64,
    };

    println!("circle c has an area of {}", c.area());
}


mod test{
    use::_struct::test_concrete;
    #[test]
    fn it_works() {
        test_concrete();
    }
}
