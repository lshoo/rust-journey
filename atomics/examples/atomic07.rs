use std::{
    hint::black_box,
    sync::atomic::{AtomicU64, Ordering},
    thread,
    time::Instant,
};

static A: AtomicU64 = AtomicU64::new(0);

fn main() {
    let start = Instant::now();
    black_box(&A);

    // Add backend thread load
    thread::spawn(|| {
        loop {
            // black_box(A.load(Ordering::Relaxed));
            // A.store(0, Ordering::Relaxed);
            black_box(
                A.compare_exchange(10, 20, Ordering::Relaxed, Ordering::Relaxed)
                    .is_ok(),
            );
        }
    });

    for _ in 0..1_000_000_000 {
        black_box(A.load(Ordering::Relaxed));
    }

    println!("{:?}", start.elapsed());

    black_box(&AA);

    thread::spawn(|| loop {
        AA[0].0.store(1, Ordering::Relaxed);
        AA[2].0.store(1, Ordering::Relaxed);
    });

    let start2 = Instant::now();
    for _ in 0..1_000_000_000 {
        black_box(AA[1].0.load(Ordering::Relaxed));
    }

    println!("{:?}", start2.elapsed());
}

#[repr(align(64))]
struct Aligned(AtomicU64);

static AA: [Aligned; 3] = [
    Aligned(AtomicU64::new(0)),
    Aligned(AtomicU64::new(0)),
    Aligned(AtomicU64::new(0)),
];
