# Some Setup

## The Operation

Here is an arbitrary function we will be using to demonstrate how to use the various techniques.

```rust
use std::time::Duration;
use std::thread::sleep;

pub fn the_operation(handle: &str, duration: u64) -> &str {
    //println!("Starting {}", handle);
    sleep(Duration::from_millis(duration));
    //println!("Finishing {}", handle);
    return handle;
}
```

<div class="notes">

It is not very complicated but it is something that we can execute with a varied duration to demonstrate concurrent operations resolving.

Just imagine that this is making an http request.

</div>

## A Quick Test

```rust
use test::Bencher;

#[test]
fn testing_the_operation() {
    let v1 = the_operation("Hello", 3);
    let v2 = the_operation("World", 2);
    let result = format!("{} {}!", v1, v2);
    assert_eq!("Hello World!", result);
}

// #[bench]
// fn benchmark_the_operation(b: &mut Bencher) -> () {
//     b.iter(|| {
//         return testing_the_operation();
//     })
// }
```
