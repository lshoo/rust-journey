use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{AtomicBool, Ordering},
    thread,
};

pub struct SpinLockSimple {
    locked: AtomicBool,
}

impl SpinLockSimple {
    pub const fn new() -> SpinLockSimple {
        SpinLockSimple {
            locked: AtomicBool::new(false),
        }
    }

    pub fn lock(&self) {
        while self.locked.swap(true, Ordering::Acquire) {
            std::hint::spin_loop();
        }
    }

    pub fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }
}

#[derive(Debug)]
pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for SpinLock<T> where T: Send {}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> SpinLock<T> {
        SpinLock {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    // need to add lifetime bound if add unlock function
    // pub lock(&self) -> &mut T {
    pub fn lock(&self) -> Guard<T> {
        while self.locked.swap(true, Ordering::Acquire) {
            std::hint::spin_loop();
        }

        Guard { lock: self }
        // unsafe { &mut *self.value.get() }
    }

    // pub fn unlock(&self) {
    //     self.locked.store(false, Ordering::Release);
    // }
}

#[derive(Debug)]
pub struct Guard<'a, T> {
    lock: &'a SpinLock<T>,
}

impl<T> Deref for Guard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // Safety
        // The very existence of this Guard gurantees that the lock is held
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // Safety
        // The very existence of this Guard guarantees we've exclusively locked the lock.
        unsafe { &mut *self.lock.value.get() }
    }
}

impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Ordering::Release);
    }
}

pub fn spin_lock() {
    let lock = SpinLock::new(Vec::new());

    thread::scope(|s| {
        s.spawn(|| lock.lock().push(1));
        s.spawn(|| {
            let mut g = lock.lock();
            g.push(2);
            g.push(2);
        });
    });

    let g = lock.lock();

    println!("g: {:?}", g.as_slice());
    assert!(g.as_slice() == [1, 2, 2] || g.as_slice() == [2, 2, 1]);
}
