use std::ops::Deref;

struct Actor;

impl Actor {
    fn name(&self) -> &'static str {
        "Leonard"
    }
}

struct DerefExapmle {
    a: Actor, 
}

impl Deref for DerefExapmle {
    type Target = Actor;

    fn deref(&self) -> &Self::Target {
        &self.a
    }
}

fn main() {
    let b = DerefExapmle { a: Actor };
    println!("{}", b.name());
}
