This repo is a demo for broken type inference of rustc when using serde_json crate.
```rust
// this line brakes type inference:
use serde_json;

#[cfg(test)]
mod test {
    #[test]
    fn into_iter() {
        assert_eq!(10, vec![1, 2, 3, 4].iter().sum());
    }
}
```
Compiling this code produces the following error:
```
error[E0283]: type annotations needed
  --> src\lib.rs:8:48
   |
 8 |         assert_eq!(10, vec![1, 2, 3, 4].iter().sum());
   |                                                ^^^
   |                                                |
   |                                                cannot infer type for type parameter `S` declared on the method `sum`
   |                                                help: consider specifying the type argument in the method call: `sum::<S>`
   |
   = note: cannot resolve `_: std::iter::Sum<&i32>`

```
Removing the `use serde_json;` makes this code compile without errors.
The toolchain used to compile this code is `stable-x86_64-pc-windows-msvc` and `rustc 1.42.0 (b8cedc004 2020-03-09)`.
Not only this brakes type inference for vector's iterator, but for any other iterator (see the src/lib.rs).