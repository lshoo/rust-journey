use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::{AtomicU64, AtomicUsize};
use std::time::{Duration, Instant};
use std::{sync::atomic::AtomicBool, thread};

pub fn atomics() {
    static STOP: AtomicBool = AtomicBool::new(false);

    // Spawn a thread to do the work.
    let background_thread = thread::spawn(|| {
        while !STOP.load(Relaxed) {
            // println!("STOP is true!");
            do_somethiong();
        }
    });

    // Use the main thread to listen for user input.
    for line in std::io::stdin().lines() {
        match line.unwrap().as_ref() {
            "help" => println!("command: help, stop"),
            "stop" => break,
            cmd => println!("unknown command: {:?}", cmd),
        }
    }

    STOP.store(true, Relaxed);

    background_thread.join().unwrap();
}

pub fn process_report() {
    let num_done = AtomicUsize::new(0);

    let main_thread = thread::current();

    thread::scope(|s| {
        // A background thread to process all 100 items
        s.spawn(|| {
            for i in 0..100 {
                process_item(i);
                num_done.store(i + 1, Relaxed);
                main_thread.unpark();
            }
        });

        loop {
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }

            println!("Working .. {n} / 100 done");
            // thread::sleep(Duration::from_secs(1));
            thread::park_timeout(Duration::from_secs(1));
        }
    });

    println!("Done!");
}

pub fn fetch_update() {
    let num_done = &AtomicUsize::new(0);
    let total_time = &AtomicU64::new(0);
    let max_time = &AtomicU64::new(0);

    thread::scope(|s| {
        // A background thread to process all 100 items, 25 each
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
                    let start = Instant::now();
                    process_item(t * 25 + i);
                    let time_taken = start.elapsed().as_micros() as u64;
                    num_done.fetch_add(1, Relaxed);
                    total_time.fetch_add(time_taken, Relaxed);
                    max_time.fetch_max(time_taken, Relaxed);
                }
            });
        }

        loop {
            let total_time = Duration::from_micros(total_time.load(Relaxed));
            let max_time = Duration::from_micros(max_time.load(Relaxed));
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }

            if n == 0 {
                println!("Working .. nothing done yet.");
            } else {
                println!(
                    "Working .. {n} / 100 done, {:?} average, {:?} peak",
                    total_time / n as u32,
                    max_time
                );
                thread::sleep(Duration::from_secs(1));
            }
        }
    });

    println!("Done!");
}

pub fn allocate_new_id() -> u64 {
    static NEXT_ID: AtomicU64 = AtomicU64::new(0);
    // let mut id = NEXT_ID.load(Relaxed);
    // loop {
    //     assert!(id < 1000, "too many IDs");
    //     match NEXT_ID.compare_exchange_weak(id, id + 1, Relaxed, Relaxed) {
    //         Ok(_) => return  id,
    //         Err(v) => id = v,
    //     }
    // }
    // equals to
    NEXT_ID
        .fetch_update(Relaxed, Relaxed, |n| n.checked_add(1))
        .expect("too many IDs")
}

pub fn get_key() -> u64 {
    static KEY: AtomicU64 = AtomicU64::new(0);
    let key = KEY.load(Relaxed);
    if key == 0 {
        let new_key = generate_random_key();
        match KEY.compare_exchange(0, new_key, Relaxed, Relaxed) {
            Ok(_) => new_key,
            Err(k) => k,
        }
    } else {
        key
    }
}

fn generate_random_key() -> u64 {
    0
}

fn process_item(i: usize) {
    println!("{i}");
    thread::sleep(Duration::from_micros(i as u64 * 1000))
}

fn do_somethiong() {
    // println!("doing somethiong");
}
