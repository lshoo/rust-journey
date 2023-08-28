use atomics::chapter05::channel_atomic;
use atomics::chapter05::channel_state;
use atomics::chapter05::typesaferef_block;

fn main() {
    channel_atomic::run();
    channel_state::run();
    typesaferef_block::run();
}
