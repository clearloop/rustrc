#[derive(Debug)]
struct David { bowie: String } // Here is no `;`, not a sentence.

impl Foo {
    fn main (self) {
        println!{"{:?}", self};
        println!{"{:?}", &self};
    }
}

fn main() {
    let foo: Go = Go { dis: "David".to_string() };
    foo.main()
}
