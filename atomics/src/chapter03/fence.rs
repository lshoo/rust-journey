use std::sync::atomic::fence;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::*;
use std::thread;
use std::time::Duration;

static mut DATA: [u64; 10] = [0; 10];

#[allow(clippy::declare_interior_mutable_const)]
const ATOMIC_FALSE: AtomicBool = AtomicBool::new(false);
static READY: [AtomicBool; 10] = [ATOMIC_FALSE; 10];

pub fn atomic_fence() {
    for i in 0..10 {
        thread::spawn(move || {
            let data = calculate(i);
            unsafe {
                DATA[i] = data;
            };
            READY[i].store(true, Release);
        });
    }

    thread::sleep(Duration::from_millis(500));

    let ready: [bool; 10] = std::array::from_fn(|i| READY[i].load(Relaxed));

    if ready.contains(&true) {
        fence(Acquire);
        for i in 0..10 {
            if ready[i] {
                println!("data{i} = {}", unsafe { DATA[i] });
            }
        }
    }
}

fn calculate(i: usize) -> u64 {
    thread::sleep(Duration::from_millis(300 + i as u64 % 3 * 100));
    123
}
