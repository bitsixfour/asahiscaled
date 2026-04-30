use std::collections::VecDeque;
use std::time::Instant;

struct TimedBuffer<T> {
    window: VecDeque<(Instant, T)>,
    duration: Duration,
}
// later
