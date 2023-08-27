use std::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    sync::atomic::{AtomicBool, Ordering},
    thread,
};

#[derive(Debug)]
pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
    is_use: AtomicBool,
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Channel<T> {
    pub const fn new() -> Channel<T> {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
            is_use: AtomicBool::new(false),
        }
    }

    /// # Safety
    /// Only call this once
    /// Panics when trying to send more than one message
    pub fn send(&self, message: T) {
        // (*self.message.get()).write(message);
        // self.ready.store(true, Ordering::Release);

        if self.is_use.swap(true, Ordering::Relaxed) {
            panic!("can't send more than one message!");
        }

        unsafe {
            (*self.message.get()).write(message);
        }
        self.ready.store(true, Ordering::Release);
    }

    pub fn is_ready(&self) -> bool {
        // self.ready.load(Ordering::Acquire)
        self.ready.load(Ordering::Relaxed)
    }

    /// Panics if no message is available yet.
    /// or if the message was already consumed
    ///
    /// Tip: use `is_ready` to check first
    ///
    pub fn receive(&self) -> T {
        // if !self.is_ready() {
        //     panic!("no message in channel");
        // }

        // (*self.message.get()).assume_init_read()
        if !self.ready.swap(false, Ordering::Acquire) {
            panic!("no message in channel");
        }

        // # Safety
        // Only after is_ready returns true, and consume once
        // We've check just check (and reset) the ready flag.
        unsafe { (*self.message.get()).assume_init_read() }
    }
}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe { self.message.get_mut().assume_init_drop() };
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
