use concrete::HasArea;
use default::{Foo, FooBar};


pub struct Circle {
    radius: f64,
}


impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
}

impl Foo for Circle {
    fn is_valid(&self) -> bool {
        true
    }
}

impl FooBar for Circle {
    fn foobar(&self) {
        println!("foobar...")
    }
}

pub fn test_concrete() {
    let c = Circle {
        radius: 1.0_f64,
    };
    c.foobar();
    
    println!("valid {:?}", c.is_valid());
    println!("valid {:?}", c.is_invalid());        
    println!("circle c has an area of {}", c.area());

}

#[cfg(test)]
mod test{
    use _struct::test_concrete;
    #[test]
    fn it_works() {
        test_concrete();
    }
}
