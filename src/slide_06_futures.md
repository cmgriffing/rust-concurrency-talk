# Futures.rs

## Futures.rs

A zero-allocation futures implementation

```rust
use futures::oneshot;
use std::thread;
use futures::Future;
use time::PreciseTime;
use time::Duration;
use slide_01_setup::the_operation;
use test::Bencher;

#[test]
fn testing_futures_rs() {

    let start = PreciseTime::now();

    let (tx, rx) = oneshot();
    let (tx2, rx2) = oneshot();

    thread::spawn(move || {
        tx.complete(the_operation("Hello", 3));
    });

    thread::spawn(move || {
        tx2.complete(the_operation("World", 2));
    });

    let res = rx.join(rx2).map(|(v1, v2)| {
        format!("{} {}!", v1, v2)
    }).wait().unwrap();

    assert_eq!("Hello World!", res);

    let end = PreciseTime::now();
    assert!(start.to(end) < Duration::seconds(5));
}

#[bench]
fn benchmark_futures_rs(b: &mut Bencher) -> () {
    b.iter(|| {
        return testing_futures_rs();
    })
}
```
