# rust dictionary type

## impl [indexmap](https://crates.io/crates/indexmap) to deal with the key-value type

## example

add this line to Cargo.toml

```toml
dictionary = "0.1.0"
```

from_iter_main.rs

```rust
use dictionary::Dictionary;

fn main() {
    let tuples: Vec<(u8, u8)> = vec![(1, 3), (5, 7)];
    let _d: Dictionary<u8, u8> = Dictionary::from_iter(tuples.iter());
    let tuples_array: Vec<(u8, [u8; 5])> = vec![(2, [3, 4, 5, 6, 7]),(8,[9,10,11,12,13])];
    let _d_array: Dictionary<u8, [u8; 5]> = Dictionary::from_iter(tuples_array.iter());

    println!("{}",*_d.get(&5).unwrap());
    println!("{:?}",*_d_array.get(&8).unwrap());
}
```

insert.rs

```rust
use dictionary::Dictionary;
fn main() {
    let mut _d: Dictionary<u8, u8> = Dictionary::new();
    _d.insert(1, 2);
    _d.insert(3, 4);

    println!("{}", *_d.get(&3).unwrap());
}
```
