use std::{sync::Mutex, thread, time::Duration};

pub fn locks() {
    let n = Mutex::new(0);

    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = n.lock().unwrap();
                for _ in 0..100 {
                    *guard += 1;
                }

                drop(guard); // Drop the guard before sleep in this thread, then other thread can get lock and do somethings
                thread::sleep(Duration::from_secs(1));
            });
        }
    });

    let nn = n.into_inner().unwrap();
    println!("Mutex is {nn} now.");

    assert_eq!(nn, 1000);
}
