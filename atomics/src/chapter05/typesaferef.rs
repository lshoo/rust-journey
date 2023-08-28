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
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Channel<T> {
    pub const fn new() -> Channel<T> {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
        }
    }

    // Regerenate the Channel, give up origin channel
    pub fn split(&mut self) -> (Sender<T>, Receiver<T>) {
        *self = Self::new();
        (Sender { channel: self }, Receiver { channel: self })
    }
}
impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe {
                (*self.message.get_mut()).assume_init_drop();
            }
        }
    }
}

#[derive(Debug)]
pub struct Sender<'a, T> {
    channel: &'a Channel<T>,
}

#[derive(Debug)]
pub struct Receiver<'a, T> {
    channel: &'a Channel<T>,
}

impl<T> Sender<'_, T> {
    pub fn send(self, message: T) {
        unsafe {
            (*self.channel.message.get()).write(message);
        }

        self.channel.ready.store(true, Ordering::Release);
    }
}

impl<T> Receiver<'_, T> {
    pub fn receive(self) -> T {
        if !self.channel.ready.swap(false, Ordering::Acquire) {
            panic!("no message in channel");
        }

        unsafe { (*self.channel.message.get()).assume_init_read() }
    }

    pub fn is_ready(&self) -> bool {
        self.channel.ready.load(Ordering::Relaxed)
    }
}

pub fn run() {
    let mut channel = Channel::new();

    println!("Before send: {:?}", channel);
    // let (sender, receiver) = channel.split();

    thread::scope(|s| {
        let (sender, receiver) = channel.split();
        // let (sender2, receiver2) = channel.split();
        let t = thread::current();

        println!("Before send: {:?}-{:?}", sender, receiver);

        s.spawn(move || {
            sender.send("hello world!");
            t.unpark();
        });

        while !receiver.is_ready() {
            thread::park();
        }

        println!("After sent: {:?}", receiver);
        assert_eq!(receiver.receive(), "hello world!");
    });

    // let (sender, receiver) = channel.split();

    println!("After channel: {:?}", channel);
}
