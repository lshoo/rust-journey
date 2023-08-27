use std::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    sync::atomic::{AtomicU8, Ordering},
    thread,
};

const EMTPY: u8 = 0;
const WRITING: u8 = 1;
const READY: u8 = 2;
const READING: u8 = 3;

#[derive(Debug)]
pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    state: AtomicU8,
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Channel<T> {
    pub const fn new() -> Channel<T> {
        Channel {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            state: AtomicU8::new(EMTPY),
        }
    }

    pub fn send(&self, message: T) {
        if self
            .state
            .compare_exchange(EMTPY, WRITING, Ordering::Release, Ordering::Relaxed)
            .is_err()
        {
            panic!("can't send more than one message!");
        }

        unsafe { (*self.message.get()).write(message) };
        self.state.store(READY, Ordering::Release);
    }

    pub fn is_ready(&self) -> bool {
        self.state.load(Ordering::Relaxed) == READY
    }

    pub fn receive(&self) -> T {
        if self
            .state
            .compare_exchange(READY, READING, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            panic!("no message in channel");
        }

        unsafe { (*self.message.get()).assume_init_read() }
    }
}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.state.get_mut() == READY {
            unsafe { self.message.get_mut().assume_init_drop() }
        }
    }
}

pub fn run() {
    let channel = Channel::new();
    let t = thread::current();

    println!("Before send: {:?}", channel);

    thread::scope(|s| {
        s.spawn(|| {
            channel.send("hello world!");
            // channel.send("hello world!"); // will panics
            t.unpark();
        });

        while !channel.is_ready() {
            thread::park();
        }
    });

    println!("After sent: {:?}", channel);
    assert_eq!(channel.receive(), "hello world!");
    println!("After received: {:?}", channel);
}
