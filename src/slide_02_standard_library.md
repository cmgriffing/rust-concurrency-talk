# Standard Library

<div class="notes">

  The first place to start is the standard library.

</div>

## Send/Sync

These traits are the fundamental building blocks of pretty much everything else I will talk about today.

[https://doc.rust-lang.org/nomicon/send-and-sync.html](https://doc.rust-lang.org/nomicon/send-and-sync.html)

- "A type is Send if it is safe to send it to another thread."

- "A type is Sync if it is safe to share between threads (&T is Send)."

- "... unlike every other trait, if a type is composed entirely of Send or Sync types, then it is Send or Sync."

<div class="notes">

</div>

## Threads

Kernel/OS level threads. Limited by the threading capabilities of the underlying hardware.

## Sharing Data - Arc and Mutex

Arc (Atomic Reference Counter) allows us to share data between threads. However, the data is inherently immutable.

A Mutex (Mutual Exclusion) allows us to lock the data as it is being written.

We can wrap our value with a Mutex and then wrap that with an Arc.  Mutexes become tricky if a thread panics after locking.

<div class="notes">

- The result of the lock method is a MutexGuard that cannot cross thread boundaries since it doesnt implement Send. The lock is therefore implicitly released when the result of the lock goes out of scope.

- To make sure the Arc/Mutex is not torn down before we are done with it, we have to add a timer to the function to allow the actions to complete. This means the timer is usually too long or not long enough.

- We can also use the method join. This blocks the parent function/thread until the child thread finishes. This means waiting on each thread explicitly.

</div>

## Joining Threads

<div class="notes">
In my opinions, this is the simplest way of handling threads in the standard library. Works especially well if your threads will have return values.
</div>

```rust
use time::PreciseTime;
use time::Duration;
use std::thread;
use slide_01_setup::the_operation;
use test::Bencher;

#[test]
fn testing_join() {
    let start = PreciseTime::now();

    let thread1 = thread::spawn(move || the_operation("Hello", 3));
    let thread2 = thread::spawn(move || the_operation("World", 2));

    let mut values = Vec::new();

    values.push(thread1.join().unwrap());
    values.push(thread2.join().unwrap());

    let result = format!("{} {}!", values[0], values[1]);

    assert_eq!("Hello World!", result);

    let end = PreciseTime::now();
    assert!(start.to(end) < Duration::seconds(5));
}

#[bench]
fn benchmark_join(b: &mut Bencher) -> () {
    b.iter(|| {
        return testing_join();
    })
}
```


## Channels

A unidirectional data flow from sender to receiver. Basically, message passing.

```rust
// use time::PreciseTime;
use std::sync::mpsc::channel;
// use std::thread;
// use slide_01_setup::the_operation;

#[test]
fn testing_channels() {
    let start = PreciseTime::now();

    let (tx, rx) = channel();
    let tx2 = tx.clone();

    thread::spawn(move || {
        tx.send(the_operation("Hello", 3)).unwrap();
    });
    thread::spawn(move || {
        tx2.send(the_operation("World", 2)).unwrap();
    });

    let mut values = Vec::new();

    values.push(rx.recv().unwrap());
    values.push(rx.recv().unwrap());

    // Also cheating by knowing the order of the responses ahead of time. Could have the function return a struct that has an index value for sorting the results.
    let result = format!("{} {}!", values[1], values[0]);

    assert_eq!("Hello World!", result);

    let end = PreciseTime::now();
    assert!(start.to(end) < Duration::seconds(5));
}

#[bench]
fn benchmark_channels(b: &mut Bencher) -> () {
    b.iter(|| {
        return testing_channels();
    })
}
```


# Utility Libraries - Quick Mentions

<div class="notes">

I did not go into too much depth on these next couple utility libraries but they are worth mentioning.

</div>

## Crossbeam

[https://github.com/aturon/crossbeam](https://github.com/aturon/crossbeam)

- Non-blocking data structures - stacks, queues, dequeues, bags, sets and maps

- Scoped threads - Can share stack data among child threads

<div class="notes">

The scope thread is really cool, but very similar standard threading.

Do not disregard this library. I just wanted to cover some more diverse libraries.

</div>

## Some others

- Simple_parallel - "this is not recommended for general use" - quoted form the docs

[https://github.com/huonw/simple_parallel](https://github.com/huonw/simple_parallel)

- Syncbox - Queues and Thread Pools (Used to contain Futures but that was abstracted away)

[https://github.com/carllerche/syncbox](https://github.com/carllerche/syncbox)
