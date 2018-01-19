observer-rs
===

[![Build Status](https://travis-ci.org/49nord/observer-rs.svg?branch=master)](https://travis-ci.org/49nord/observer-rs)
[![Crates.io version](https://img.shields.io/crates/v/observer.svg)](https://crates.io/crates/observer)

Implementation of the [observer](https://en.wikipedia.org/wiki/Observer_pattern) software design
pattern (also known as event emitter) for rust.

See the [documentation](https://docs.rs/observer) for details.

```rust
struct Model {
    counter: usize,
}

impl Model {
        fn increment(&mut self) {
            self.counter += 1;
        }
}

let model = Model { counter: 42 };

// create a subject that can be observed
let mut subject = Subject::new(model);
// add a function that is informed about changes
subject.register(|v: &mut Model| println!("new counter value: {}", v.counter));

subject.get_mut_notify().increment();
```
