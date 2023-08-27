use std::{
    collections::VecDeque,
    sync::{Condvar, Mutex},
};

#[derive(Debug, Default)]
pub struct Channel<T> {
    queue: Mutex<VecDeque<T>>,
    item_ready: Condvar,
}

impl<T> Channel<T> {
    pub fn new() -> Channel<T> {
        Channel {
            queue: Mutex::new(VecDeque::new()),
            item_ready: Condvar::new(),
        }
    }

    pub fn send(&self, message: T) {
        self.queue.lock().unwrap().push_back(message);
        self.item_ready.notify_one();
    }

    pub fn receive(&self) -> T {
        let mut queue = self.queue.lock().unwrap();

        loop {
            if let Some(messsage) = queue.pop_front() {
                return messsage;
            }

            queue = self.item_ready.wait(queue).unwrap();
        }
    }
}
