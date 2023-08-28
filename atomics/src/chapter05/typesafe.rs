use std::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
};

#[derive(Debug)]
struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

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
pub struct Sender<T> {
    channel: Arc<Channel<T>>,
}

#[derive(Debug)]
pub struct Receiver<T> {
    channel: Arc<Channel<T>>,
}

impl<T> Sender<T> {
    pub fn send(self, message: T) {
        unsafe {
            (*self.channel.message.get()).write(message);
        }

        self.channel.ready.store(true, Ordering::Release);
    }
}

impl<T> Receiver<T> {
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

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let a = Arc::new(Channel {
        message: UnsafeCell::new(MaybeUninit::uninit()),
        ready: AtomicBool::new(false),
    });

    (Sender { channel: a.clone() }, Receiver { channel: a })
}

pub fn run() {
    thread::scope(|s| {
        let (sender, receiver) = channel();
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
        // println!("After received: {:?}:{:?}", sender, receiver);
    });
}
