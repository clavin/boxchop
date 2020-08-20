# boxchop

[Documentation][docs] | [View on `libs.rs`][crate]

A tiny library for creating boxed slices `Box<[T]>`.

```rust
let nums = boxchop::new_with(5, |x| x + 1);

assert_eq!(
    nums,
    Box::from([1, 2, 3, 4, 5]),
);
```

⚠️ This was created before I found the `new_uninit` nightly feature. The fundamental itch this crate
scratches will eventually be part of the standard library: see
[`Box::new_uninit_slice`][box_new_uninit_slice] and [`Box::assume_init`][box_assume_init].

[box_new_uninit_slice]: https://doc.rust-lang.org/std/boxed/struct.Box.html#method.new_uninit_slice
[box_assume_init]: https://doc.rust-lang.org/std/boxed/struct.Box.html#method.assume_init-1
[crate]: https://libs.rs/crates/boxchop/
[docs]: https://docs.rs/boxchop/
