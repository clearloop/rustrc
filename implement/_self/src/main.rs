#[derive(Debug)]
struct Foo { dis: String } // Here is no `;`, not a sentence.

impl Foo {
    fn main (&self) {
        let bar:&Foo = self;
        println!("{:?}", bar.dis);
        println!{"{:?}", bar};
        println!{"{:?}", bar};
    }
}

fn main() {
    let foo: Foo = Foo { dis: "David".to_string() };
    foo.main()
}

