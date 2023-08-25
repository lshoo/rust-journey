use std::{thread, time::Duration};

use atomics::chapter03::{
    acqrel::acquire_release, fence::atomic_fence, get_and_print_data, multi_atomic_lock,
    out_of_thin_air_value, release_acquire, seq_cst,
};

fn main() {
    out_of_thin_air_value();

    release_acquire();

    multi_atomic_lock(2);

    get_and_print_data();

    seq_cst();

    atomic_fence();

    acquire_release();

    thread::sleep(Duration::from_secs(1));
}
