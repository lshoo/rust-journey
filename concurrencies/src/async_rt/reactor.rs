use std::os::fd::{BorrowedFd, RawFd};
use std::sync::{atomic::AtomicBool, Arc, Mutex};

use futures::task::AtomicWaker;
use polling::Poller;
use slab::Slab;

pub struct Reactor {
    // Poller instance
    poller: Poller,
    // repository
    repo: Mutex<Slab<Arc<IoEvent>>>,
}

struct IoEvent {
    fd: RawFd,
    key: usize,
    is_ready: AtomicBool,
    waker: AtomicWaker,
}

impl Reactor {
    // Io 循环事件, 当fd就绪时，调用注册的waker
    pub fn event_loop(&self) -> std::io::Result<()> {
        todo!()
    }

    // 注册一个可读事件，当fd可读时返回
    pub async fn register_readable(&self, fd: BorrowedFd<'_>) -> std::io::Result<()> {
        todo!()
    }
}
