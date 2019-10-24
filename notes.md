googled some tutorials
one mentioned the [sample](https://docs.rs/sample/0.10.0/sample/) library 
how to write out little-endian? [byteorder](https://docs.rs/byteorder/1.3.2/byteorder/)
how to play sounds? [portaudio](https://docs.rs/portaudio/0.7.0/portaudio/)

using ?

error: `core::str::<impl str>::as_bytes` is not yet stable as a const fn
had to add `#![feature(const_str_as_bytes)]`
found the `b""` notation by looking at the docs for as_bytes - feature no longer needed

`into()` function to cast nums - discovered via an error message

```
error[E0308]: mismatched types
  --> src/main.rs:31:9
   |
31 |         i16::MAX,
   |         ^^^^^^^^ expected f64, found i16
help: you can convert an `i16` to `f64`, producing the floating point representation of the integer
   |
31 |         i16::MAX.into(),
   |         ^^^^^^^^^^^^^^^
```

the trait `std::convert::From<f64>` is not implemented for `i16` - probably because lossless conversion is not always possible
`<i16 as std::convert::From<bool>>`
`<i16 as std::convert::From<i8>>`
`<i16 as std::convert::From<std::num::NonZeroI16>>`
`<i16 as std::convert::From<u8>>`

`try_into()` and `into()`

switched to doing a dumb iteration with a counter to using `take` and directly iterating

explain how the chaining works

Why aren't functions like `sqrt` and `powf` const? - https://github.com/rust-lang/rust/issues/57563

is `extern crate` required?

add BuffWriter to speed things up quite a bit

turns out `byteorder` wasn't necessary! instead, use `to_le_bytes`/`to_be_bytes`/`to_ne_bytes`

it's required to declare the type of `const`s, i.e. `const FMT_CHUNK_SIZE: u32 = 16;`

clippy:

```
error: handle written amount returned or use `Write::write_all` instead
  --> src/main.rs:68:5
   |
68 |     wav_output_file.write(RIFF_LABEL)?;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
```

`unreadable_literal` lint

`fn foo(file: &mut dyn Write)` vs `fun foo<T: Write>(file: &mut T)`?
