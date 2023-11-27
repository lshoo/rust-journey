
pub mod condvar;

use std::{sync::atomic::{AtomicU32, Ordering}, cell::UnsafeCell, ops::{Deref, DerefMut}};

use atomic_wait::{wait, wake_one};


pub struct Mutex<T> {
    /// 0: unlocked
    /// 1: locked, no other threads waiting
    /// 2: locked, other threads waiting
    state: AtomicU32,
    value: UnsafeCell<T>,
}

/// For share between threads
unsafe impl<T> Sync for Mutex<T> where T: Send { }

impl<T> Mutex<T> {
    pub const fn new(value: T) -> Self {
        Self {
            state: AtomicU32::new(0),   // unlocked state
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> MutexGuard<T> {
        if self.state.compare_exchange(0, 1, Ordering::Acquire, Ordering::Relaxed).is_err() {
            // Set the state to 2: locked and waiting
            // while self.state.swap(2, Ordering::Acquire) != 0 {
            //     wait(&self.state, 2);
            // }    

            lock_contended(&self.state);
        }
        
        MutexGuard { mutex: self }
    }

    
}

fn lock_contended(state: &AtomicU32) {
    let mut spin_count = 0;

    while state.load(Ordering::Relaxed) == 1 && spin_count < 100 {
        spin_count += 1;
        std::hint::spin_loop();
    }

    if state.compare_exchange(0, 1, Ordering::Acquire, Ordering::Relaxed).is_ok() {
        return;
    }

    while state.swap(2, Ordering::Acquire) != 0 {
        wait(state, 2)
    }
}

/// For guard
pub struct MutexGuard<'a, T> {
    mutex: &'a Mutex<T>,
}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.mutex.value.get() }
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.mutex.value.get() }
    }
}

impl<T> Drop for MutexGuard<'_, T> {
    fn drop(&mut self) {
        // Set state back to 0: unlocked
        // self.mutex.state.store(0, Ordering::Release);

        // Wake up one of the waiting threads, if any
        // wake_one(&self.mutex.state)

        // if other threads are waiting, wake it and set state to 0
        if self.mutex.state.swap(0, Ordering::Release) == 2 {
            wake_one(&self.mutex.state);
        }
    }
}