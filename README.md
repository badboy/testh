# #[tesŧ] - A beŧŧer ŧesŧ annoŧaŧion

[![Build Status](https://travis-ci.org/badboy/testh.svg?branch=master)](https://travis-ci.org/badboy/testh)

Ever hit the following error?

> src/lib.rs:3:5: 3:12 error: The attribute `tesŧ` is currently unknown to the compiler and may have meaning added to it in the future (see issue #29642)

You don't have to! Just use `tesŧ` … I mean `testh`.

*Because compiler plugins are unstable, you need to use Rust Nightly.*

## Use it

Add it as a dev-dependency to your `Cargo.toml`:
```
[dev-dependencies]
testh = { git = "https://github.com/badboy/testh" }
```

Then enable the plugin and tesŧ away!

```
#![feature(plugin, custom_attribute)]
#![plugin(testh)]

#[tesŧ]
fn it_really_works() {
    assert!(true);
}
```

It even works in tesŧ modules:

```
#![feature(non_ascii_idents)]

#[cfg(test)]
mod tesŧ {
    #[tesŧ]
    fn it_works() {
    }
}
```
