use std::{
    sync::atomic::{
        AtomicBool, AtomicI32, AtomicI64,
        Ordering::{Acquire, Relaxed, Release},
    },
    thread,
    time::Duration,
};

static X: AtomicI32 = AtomicI32::new(0);
static Y: AtomicI32 = AtomicI32::new(0);

pub fn out_of_thin_air_value() {
    let a = thread::spawn(|| {
        let x = X.load(Relaxed);
        Y.store(x, Relaxed);
    });

    let b = thread::spawn(|| {
        let y = Y.load(Relaxed);
        X.store(y, Relaxed);
    });

    a.join().unwrap();
    b.join().unwrap();

    println!("X: {}, Y: {}", X.load(Relaxed), Y.load(Relaxed));

    assert_eq!(X.load(Relaxed), 0); // Might fail?
    assert_eq!(Y.load(Relaxed), 0); // Might fail?
}

static DATA: AtomicI64 = AtomicI64::new(0);
static READY: AtomicBool = AtomicBool::new(false);

pub fn release_acquire() {
    thread::spawn(|| {
        DATA.store(123, Relaxed);
        READY.store(true, Release);
    });

    while !READY.load(Acquire) {
        thread::sleep(Duration::from_millis(100));
        println!("waiting ... ");
    }

    println!("{}", DATA.load(Relaxed));
}
