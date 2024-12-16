use crate::random::random_in_range;
use crate::types::Bits96;
use std::sync::atomic::{AtomicU64, Ordering};

pub struct IvCounter {
    counter: AtomicU64,
}

impl IvCounter {
    pub fn new() -> Self {
        let mut random_start = random_in_range(0, 1_u64 << 62);
        if random_start % 2 != 0 {
            random_start += 1
        }
        Self {
            counter: AtomicU64::new(random_start),
        }
    }

    // Agent always uses an "even" IV and frontend always uses an "odd" IV,
    // guaranteeing that IVs from Agent & Frontend will never overlap.
    pub fn next(&self) -> Bits96 {
        self.counter.fetch_add(2, Ordering::SeqCst);
        self.to_bits96()
    }

    fn to_bits96(&self) -> Bits96 {
        let counter_value = self.counter.load(Ordering::SeqCst);
        Bits96::from(counter_value)
    }
}
