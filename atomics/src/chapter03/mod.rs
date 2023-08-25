pub mod acqrel;
pub mod fence;

use std::{
    sync::atomic::{
        AtomicBool, AtomicI32, AtomicI64, AtomicPtr,
        Ordering::{Acquire, Relaxed, Release, SeqCst},
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

static mut DATA2: String = String::new();
static LOCKED: AtomicBool = AtomicBool::new(false);

pub fn atomic_lock() {
    if LOCKED
        .compare_exchange(false, true, Acquire, Relaxed)
        .is_ok()
    {
        unsafe {
            DATA2.push('!');
        };
        LOCKED.store(false, Release);
        println!("atomic lock ended!");
    }
}

pub fn multi_atomic_lock(ths: u64) {
    thread::scope(|s| {
        for _ in 0..ths {
            s.spawn(atomic_lock);
        }
    })
}

#[derive(Debug)]
pub struct Data([u8; 8]);

pub fn get_data() -> &'static Data {
    static PTR: AtomicPtr<Data> = AtomicPtr::new(std::ptr::null_mut());

    let mut p = PTR.load(Acquire);

    if p.is_null() {
        p = Box::into_raw(Box::new(generate_data()));

        if let Err(e) = PTR.compare_exchange(std::ptr::null_mut(), p, Release, Acquire) {
            // Safety: p comes from Box::into_raw right above, and wasn't shared with any other thread.
            drop(unsafe { Box::from_raw(p) });
            p = e;
        }
    }

    // Safety: p is not null and points to a properly initialized value.
    unsafe { &*p }
}

pub fn get_and_print_data() {
    for i in 1..=3 {
        thread::spawn(move || {
            let d = get_data();
            println!("thread {i} data is: {:?}, {:p}", d, d);
        });
    }
}

fn generate_data() -> Data {
    Data([123; 8])
}

static A: AtomicBool = AtomicBool::new(false);
static B: AtomicBool = AtomicBool::new(false);

static mut S: String = String::new();

pub fn seq_cst() {
    let a = thread::spawn(|| {
        A.store(true, SeqCst);
        if !B.load(SeqCst) {
            unsafe { S.push('!') };
            println!("Set S in a thread");
        }
    });

    let b = thread::spawn(|| {
        B.store(true, SeqCst);
        if !A.load(SeqCst) {
            unsafe { S.push('?') };
            println!("Set S in b thread");
        }
    });

    a.join().unwrap();
    b.join().unwrap();

    println!(
        "A: {}, B: {}, S: {:?}",
        A.load(Relaxed),
        B.load(Relaxed),
        unsafe { S.as_str() }
    );
}
