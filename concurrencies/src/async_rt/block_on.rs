//! https://toetoe55.github.io/async-rt-book/block_on.html

use std::{
    future::{Future, IntoFuture},
    pin::pin,
    task::{Context, Poll},
};

use waker_fn::waker_fn;

pub fn block_on<T>(fut: impl IntoFuture<Output = T>) -> T {
    // 当前线程的 `parker` and `unpacker`
    let (parker, unpacker) = parking::pair();

    // waker 在调用 `.wake()` 时 unpark 当前线程
    let waker = waker_fn(move || {
        unpacker.unpark();
    });

    let ctx = &mut Context::from_waker(&waker);

    // 轮询 `Future`
    let mut fut = pin!(fut.into_future());

    loop {
        if let Poll::Ready(t) = fut.as_mut().poll(ctx) {
            return t;
        }

        // 当返回 `Pending` 时，休眠当前线程，等待 `waker` 被调用
        // note： 如果waker已经被调用过了，这里不会阻塞
        parker.park();
    }
}
