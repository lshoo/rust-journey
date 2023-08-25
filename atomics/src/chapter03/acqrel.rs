use std::{
    sync::atomic::{AtomicBool, AtomicU64, Ordering::*},
    thread,
};

static X: AtomicBool = AtomicBool::new(false);
static Y: AtomicBool = AtomicBool::new(false);
static Z: AtomicU64 = AtomicU64::new(0);

pub fn acquire_release() {
    let t1 = thread::spawn(move || {
        write_x_then_y();
    });

    let t2 = thread::spawn(move || {
        write_y_then_x();
    });

    t2.join().unwrap();
    t1.join().unwrap();

    assert_eq!(Z.load(Relaxed), 1); // Z must be 1
}

fn write_x_then_y() {
    X.store(true, Relaxed);
    Y.store(true, Release);
}

fn write_y_then_x() {
    while !Y.load(Acquire) {}
    if X.load(Relaxed) {
        Z.fetch_add(1, Relaxed);
    }
}
