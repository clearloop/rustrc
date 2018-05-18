# Guessing game

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
