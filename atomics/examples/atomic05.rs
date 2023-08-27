use atomics::chapter05::channel_atomic;
use atomics::chapter05::channel_state;

fn main() {
    channel_atomic::run();
    channel_state::run();
}
