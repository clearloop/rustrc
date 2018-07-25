# Bytes

## What is Bytes

### u8 and Bytes and Slices and Vec.

### std::str::Bytes

#### String.bytes()
`pub struct Bytes<'a>(_)`
An iterator over the bytes of a string slice.

```rust

let b1 = "str".bytes();                                                                                                                                                                          
println!("{:?}", b1);
// Bytes(Cloned { it: Iter([115, 116, 114]) })

```

#### String.into_bytes
`pub fn into_bytes(self) -> Vec<u8>`
Converts a String into a byte vector.

```rust
Converts a String into a byte vector.
```

#### String.as_bytes
`pub fn as_bytes(&self) -> &[u8]`
Converts a string slice to a byte slice. 

```rust

let b2 = "u8".as_bytes();
println!("{:?}", b2);
// [115, 116, 114]

```

### bytes::Bytes
`pub struct Bytes {/* fields omitted */}`
A reference counted contiguous slice of memory.

```rust

let b2 = Bytes::from_static("str".as_bytes());                                                                                                                                                         
println!("{:?}", b2);
// b"str"

```
