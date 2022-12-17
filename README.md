 

# Hyperoperation

This crate allows to calculate and format [hyperoperation](https://en.wikipedia.org/wiki/Hyperoperation), sometimes known as **Knuth's up-arrow notation**, which is a way to define very large numbers, such as famous 3↑↑↑3.

### Features

- Calculate value of hyperoperations ([more](hyperoperation) and [more](Hyperoperation::evaluate))
- Format hyperoperations with Knuth's up-arrow notation ([more](struct.Hyperoperation.html#method.fmt))
- Use any unsigned numeric type for calculation fitting [some conditions](NumForKnuth), such as [BigUint](https://docs.rs/num-bigint/latest/num_bigint/struct.BigUint.html)



### Examples

Simple calculation:

```rust
use hyperoperation::*; 
assert_eq!(
    hyperoperation::<u64>(&3, 3, 2), // 3 ↑↑ 3
    7625597484987
);
```



Using [BigUint](https://docs.rs/num-bigint/latest/num_bigint/struct.BigUint.html) to handle big results without overflowing (don't forget to add [`num_bigint`](https://lib.rs/crates/num-bigint) as your dependency) :

```rust
use hyperoperation::*; 
use num_bigint::BigUint;

let result = hyperoperation::<BigUint>(&5u8.into(), 3u8.into(), 2); // 5 ↑↑ 3
println!("Result:\n{result}");
assert_eq!(
    result % BigUint::from(100_000_000u32),
    8203125u32.into()
);
```



Using [Hyperoperation struct ](Hyperoperation)and formatting it with [Knuth's up-arrow notation](https://en.wikipedia.org/wiki/Knuth%27s_up-arrow_notation):

```rust
use hyperoperation::*;
let expr = Hyperoperation::<u64>::new(3, 3, 2); // Represents 3 ↑↑ 3
let result = expr.clone().evaluate(); // Calculate the value of 3 ↑↑ 3

println!("{expr} = {result}");
assert_eq!(result, 7625597484987);
assert_eq!(format!("{expr}"), "3 ↑↑ 3");
```

