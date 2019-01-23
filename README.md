## Rustrc Questions

+ Show u8 slice in hex representation - [stackoverflow][s-1]
> convert each two bytes into [hex][r-1].
```rust
fn main() {
    // Template
    let bytes = [
        177, 147, 134, 198, 163, 82, 51, 22,
        41, 118, 161, 105, 142, 86, 191, 146,
        250, 11, 152, 10, 58, 75, 238, 24,
        21, 46, 68, 18, 3, 112, 162, 50,
    ];

    // String to extend.
    let mut hex = String::new();
    hex.extend(bytes.iter().map(|byte| format!("{:02x}", byte)));

    // Print it.
    println!("{:}", hex);
}
```

+ Is it possible for one struct to extend an existing struct, keeping all the fields? - [stackoverflow][s-2]
> Implementing [Deref][r-2] for smart pointers that we can access the data behind, kind like inherit.
```
// Existing struct.
struct Person {
  gender: &'static str,
  age:    &'static i32
}

// Some struct new.
struct Child(Person);

// Implementing Deref.
impl std::ops::Deref for Child {
  type Target = Person;
  fn deref(&self) -> &Self -> &Self::Target {
    &self.0
  }
}

fn main() {
  let child = Child(Person{
    gender: 'male',
    age:     6_i32
  });

  println!("your child is {}, {} years old.", child.gender, child.age);
}
```


<!-- rust docs -->
[r-1]: https://doc.rust-lang.org/std/fmt/index.html#width
[r-2]: https://doc.rust-lang.org/std/ops/trait.Deref.html

<!-- question links -->
[s-1]: https://stackoverflow.com/questions/27650312/show-u8-slice-in-hex-representation/54302798?noredirect=1#comment95439998_54302798
[s-2]: https://stackoverflow.com/questions/32552593/is-it-possible-for-one-struct-to-extend-an-existing-struct-keeping-all-the-fiel/54301034#54301034
