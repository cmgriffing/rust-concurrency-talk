# Futures

## Formerly in Standard Library

Deprecated since 1.2.0
[https://doc.rust-lang.org/1.2.0/std/sync/struct.Future.html](https://doc.rust-lang.org/1.2.0/std/sync/struct.Future.html)

Maybe this helps explain why:
[https://www.reddit.com/r/rust/comments/2m64o5/stdsyncfuture_is_almost_useless_for_async/](https://www.reddit.com/r/rust/comments/2m64o5/stdsyncfuture_is_almost_useless_for_async/)

## Popular Libraries

- Eventual - [https://github.com/carllerche/eventual](https://github.com/carllerche/eventual)
- futures-rs - [https://github.com/alexcrichton/futures-rs](https://github.com/alexcrichton/futures-rs)

## Eventual

```rust
use time::PreciseTime;
use time::Duration;
use eventual::*;
use slide_01_setup::the_operation;

#[test]
fn testing_futures() {
    let start = PreciseTime::now();

    let future1 = Future::spawn(|| the_operation("Hello", 3));

    let future2 = Future::spawn(|| the_operation("World", 2));

    let res = join((future1, future2))
        .and_then(|(v1, v2)| Ok(format!("{} {}!", v1, v2)))
        .await()
        .unwrap();

    assert_eq!("Hello World!", res);

    let end = PreciseTime::now();
    assert!(start.to(end) < Duration::seconds(5));
}
```


## Promises

Similar to futures. Key Differences:

- Single-threaded

It is pretty much like working with Node.js

[https://github.com/dwrensha/gj](https://github.com/dwrensha/gj)

<div class="notes">

  For the most part, Eventual's Futures seem more useful.

  However, a single threaded event loop could be useful to isolate business logic from rendering logic if you were making something like a game engine.

</div>

## An Example

Warning: Not async, determined by ~5 second run time. Not sure why, but definitely user error.

```rust
// use time::PreciseTime;
use gj::{EventLoop, Promise, ClosedEventPort};
// use slide_01_setup::the_operation;

#[test]
fn testing_promises() {
    EventLoop::top_level(|wait_scope| -> Result<(), ()> {
        let start = PreciseTime::now();

        let mut promises = Vec::new();

        let promise1 = Promise::<(), ()>::ok(());
        promises.push(promise1.then(|_| Promise::ok(the_operation("Hello", 3))));

        let promise2 = Promise::<(), ()>::ok(());
        promises.push(promise2.then(|_| Promise::ok(the_operation("World", 2))));

        let values = Promise::all(promises.into_iter())
            .wait(wait_scope, &mut ClosedEventPort(()))
            .unwrap();

        let result = format!("{} {}!", values[0], values[1]);
        assert_eq!("Hello World!", result);

        let end = PreciseTime::now();
        assert!(start.to(end) < Duration::seconds(5));

        Ok(())
    })
    .expect("top level");
}
```
