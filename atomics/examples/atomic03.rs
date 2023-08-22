use atomics::chapter03::{out_of_thin_air_value, release_acquire};

fn main() {
    out_of_thin_air_value();

    release_acquire();
}
