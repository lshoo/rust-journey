use std::{time::Instant, thread};

use atomics::chapter09::Mutex;


fn main() {
    let m = Mutex::new(0);
    std::hint::black_box(&m);

    let start = Instant::now();

    for _ in 0..5_000_000 {
        *m.lock() += 1;
    }

    let duration = start.elapsed();

    println!("locked {} times in {:?}", *m.lock(), duration);

    let m = Mutex::new(0);
    std::hint::black_box(&m);
    let start = Instant::now();
    thread::scope(|s| {
        for _ in 0..4 {
            s.spawn(|| {
                for _ in 0..5_000_000 {
                    *m.lock() += 1;
                }
            });
        }
    });
    let duration = start.elapsed();
    println!("locked {} times in {:?}", *m.lock(), duration);
}