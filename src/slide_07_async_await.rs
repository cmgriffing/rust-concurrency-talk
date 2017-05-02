// //@ # Async/Await (cpupool)
// //@
// //@ ##


// //copied in, dirty and hacky

// use futures::*;
// use std::thread;
// use futures_cpupool::CpuPool;

// #[macro_export]
// macro_rules! async {
//     ($e: expr) => ({
//         let (tx, rx) = oneshot();
//         thread::spawn(move || {
//             tx.complete($e);
//         });
//         rx
//     });
//     ($block: block) => ({
//         let (tx, rx) = oneshot();
//         thread::spawn(move || {
//             tx.complete($block);
//         });
//         rx
//     });
//     // ($e: expr, $pool: ident) => ({
//     //     let (tx, rx) = oneshot();
//     //     $pool.spawn({
//     //         tx.complete($e);
//     //         rx
//     //     })
//     // });
//     ($block: block, $pool: ident) => ({
//         let (tx, rx) = oneshot();
//         $pool.spawn(move || {
//             tx.complete($block);
//             rx
//         });
//     });
// }

// #[macro_export]
// macro_rules! await {
//     ($f: expr) => {
//         $f.wait().unwrap()
//     };
//     ($f: expr, $d: expr) => {
//         match $f.wait() {
//             Ok(e) => e,
//             Err(_) => $d
//         }
//     }
// }


// use time::PreciseTime;
// use time::Duration;
// use slide_01_setup::the_operation;
// use test::Bencher;

// #[test]
// fn testing_async_await_cpupool() {
//     let start = PreciseTime::now();

//     let pool = CpuPool::new(4);

//     let value1 = async!({
//         the_operation("Hello", 3);
//     }, pool);
//     let value2 = async!({
//         the_operation("World", 2);
//     }, pool);

//     let result = format!("{} {}!", await!(value1), await!(value2));
//     assert_eq!("Hello World!", result);

//     let end = PreciseTime::now();
//     assert!(start.to(end) < Duration::seconds(5));
// }

// #[bench]
// fn benchmark_async_await_cpupool(b: &mut Bencher) -> () {
//     b.iter(|| {
//         return testing_async_await_cpupool();
//     })
// }
