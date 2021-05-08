Awaitable promises in Rust. Similar to promises in Javascript, except you can only await them, no then/catch business.

I made this for fun. I claim neither completeness nor correctness. 

To use it, just add this to your cargo.toml:
```toml
[dependencies]
no-promises = { git = "https://github.com/wolfgang/no-promises" }

```

Anyway, here are some examples.

- Resolve or Reject, that is the question:

```rust

use no_promises::{Promise, Error};

#[async_std::test]
async fn resolve_or_fail() {
    assert_eq!(promise_div(10.0, 0.0).await, Err(Error::from("division by zero")));
    assert_eq!(promise_div(10.0, 2.0).await, Ok(5.0));
}

fn promise_div(a: f32, b: f32) -> Promise<f32> {
    Promise::new(move |resolve, reject| {
        if b == 0.0 { reject(promise::Error::from("division by zero")) } else { resolve(a / b) }
    })
}

```

- Threadless sleeping:

```rust
use std::time;
use no_promises::{Promise, Error};

#[async_std::main]
async fn main() {
    println!("Wait for it ...");
    silly_sleep(1000).await.unwrap();
    println!("Hello!");
}

fn silly_sleep(millis: u128) -> Promise<()> {
    Promise::new(move |resolve, _| {
        let mut waiting = 0;
        let mut last = time::SystemTime::now();
        while waiting <= millis*1000 {
            let now = time::SystemTime::now();
            let duration = now.duration_since(last);
            last = now;
            waiting += duration.unwrap().as_micros();
        }
        resolve(())

    })
}
```
