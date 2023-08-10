use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::time::Duration;
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

fn process_item(i: usize) {
    println!("{i}");
}

fn do_somethiong() {
    // println!("doing somethiong");
}
