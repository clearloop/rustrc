# Guessing game

### Dependencies
`[dependencies]` in `Cargo.toml` just like `[package]`: all the contents will be part of the label.

`[dependencies]` content dependencies and their versions. when we build our project, it will download the dependencies our project need.

### extern crate rand and rand:: Rng

```rust
extern crate rand;

let secret_number = rand::thread_rng().gen_range(1, 101);

```
> error[E0599]: no method named "gen_range" found for type `rand::ThreadRng` in the current scope  


```rust
extern crate rand;

use rand::Rng;

let secret_number = randn::thread_rng().gen_range(1, 101);
```
> passed  
