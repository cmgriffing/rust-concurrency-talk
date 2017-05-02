# Coroutines

## Libraries

- mioco - [https://github.com/dpc/mioco](https://github.com/dpc/mioco)
- coio-rs - [https://github.com/zonyitoo/coio-rs](https://github.com/zonyitoo/coio-rs)
- coroutine-rs - [https://github.com/rustcc/coroutine-rs](https://github.com/rustcc/coroutine-rs)

## Mioco

Built on top of Mio. Mio stands for Metal I/O. It just received $30k for api enhancements from the Mozilla Foundation.

[http://hermanradtke.com/2015/07/12/my-basic-understanding-of-mio-and-async-io.html](http://hermanradtke.com/2015/07/12/my-basic-understanding-of-mio-and-async-io.html)

<div class="notes">

- Designed to have a similar api to Standard Library channels and threads for easy adoption.

- This code snippet probably looks a little familiar. It is very similar to the way we implemented a channel using the standard library.

</div>

```rust
use time::PreciseTime;
use time::Duration;
use mioco;
use mioco::Mioco;
use mioco::sync::mpsc::channel;
use slide_01_setup::the_operation;
use test::Bencher;

#[test]
fn testing_mioco_coroutines() {
    Mioco::new()
        .start(move || {
            let start = PreciseTime::now();

            let (tx, rx) = channel();
            let tx2 = tx.clone();

            mioco::spawn(move || {
                tx.send(the_operation("Hello", 3)).unwrap();
            });

            mioco::spawn(move || {
                tx2.send(the_operation("World", 2)).unwrap();
            });

            let mut values = Vec::new();

            values.push(rx.recv().unwrap());
            values.push(rx.recv().unwrap());

            let result = format!("{} {}!", values[1], values[0]);
            assert_eq!("Hello World!", result);

            let end = PreciseTime::now();
            assert!(start.to(end) < Duration::seconds(5));

        })
        .unwrap();
}

// #[bench]
// fn benchmark_mioco_coroutines(b: &mut Bencher) -> () {
//     b.iter(|| {
//         return testing_mioco_coroutines();
//     })
// }
```
