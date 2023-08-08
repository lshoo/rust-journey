use std::{sync::Arc, thread};

fn main() {
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);

    println!("This main thread.");

    let _ = t1.join();
    let _ = t2.join();

    let numbers = vec![1, 3, 9];
    thread::spawn(move || {
        for n in numbers {
            println!("{}!", n);
        }
    })
    .join()
    .unwrap();

    let numbers = Vec::from_iter(1..=1000);
    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum: usize = numbers.iter().sum();

        sum / len
    });

    let aver = t.join().unwrap();
    println!("average is: {aver}");

    // scope thread
    let numbers = vec![1, 3, 5, 7, 9];
    thread::scope(|s| {
        s.spawn(|| {
            println!("length: {}", numbers.len());
        });
        s.spawn(|| {
            for n in &numbers {
                println!("this is {n}");
            }
        });
    });

    static X: [i32; 5] = [1, 2, 3, 5, 7];
    thread::spawn(|| dbg!(&X));
    thread::spawn(|| dbg!(&X));

    // leaking
    let x: &'static [i32; 5] = Box::leak(Box::new([2, 4, 6, 8, 10]));
    thread::spawn(move || dbg!(x)).join().unwrap();
    thread::spawn(move || dbg!(x)).join().unwrap();

    println!("Printed leaking");

    // Arc
    let a = Arc::new([1, 2, 3]);
    let b = a.clone();

    thread::spawn(move || dbg!(a)).join().unwrap();
    thread::spawn(move || dbg!(b)).join().unwrap();

    println!("Printed Arc");

    // Naming clones
    let a = Arc::new([1, 2, 3]);
    let b = a.clone();

    thread::spawn(move || {
        dbg!(b);
    })
    .join()
    .unwrap();

    dbg!(a);

    let a = Arc::new([2, 4, 6]);
    thread::spawn({
        let a = a.clone();
        move || {
            dbg!(a);
        }
    })
    .join()
    .unwrap();

    dbg!(a);

    println!("Printed Naming clones");
}

fn f() {
    let id = thread::current().id();
    println!("This is from thread: {id:?}");
}
