use std::fmt::Debug;

// Traits: [<Debug>, <Clone, Debug>] join
pub fn foo<T: Debug, K: Clone + Debug>(x: T, y: K) {
    println!("{:?}", x);
    println!("{:?}", y);    
}

// implement foo by where
pub fn bar<T, K>(x: T, y: K) where T: Debug, K: Clone + Debug {
    println!("{:?}", x);
    println!("{:?}", y);
}

#[cfg(test)]
mod test{
    use blur::*;
    #[test]
    fn it_works() {
        let para_1 = "hello".to_string();
        let para_2 = "world".to_string();
        foo(para_1.clone(), para_2.clone());
        bar(para_1, para_2);        
    }
}
