use super::*;
use std::{
    thread::{sleep, yield_now},
    time::{Duration, Instant},
};

pub struct StdWaiter {
    timeout: Duration,
    interval: Option<Duration>,
}

impl StdWaiter {
    /// - `timeout`
    /// - `interval`: Before the time limit expires, this action will execute each time `timeout()` is called.
    ///     - `None`: do nothing
    ///     - `Some(Duration::ZERO)`: call `yield_now()`
    ///     - `Some(Duration)`: call `sleep(Duration)`
    pub fn new(timeout: Duration, interval: Option<Duration>) -> Self {
        Self { timeout, interval }
    }
}

impl Waiter for StdWaiter {
    #[inline]
    fn start(&self) -> impl WaiterInstance {
        StdWaiterInstance {
            start_time: Instant::now(),
            waiter: self,
        }
    }
}

pub struct StdWaiterInstance<'a> {
    start_time: Instant,
    waiter: &'a StdWaiter,
}

impl<'a> WaiterInstance for StdWaiterInstance<'a> {
    #[inline]
    fn timeout(&mut self) -> bool {
        if Instant::now() - self.start_time >= self.waiter.timeout {
            true
        } else {
            match self.waiter.interval {
                None => (),
                Some(Duration::ZERO) => yield_now(),
                Some(dur) => sleep(dur),
            }
            false
        }
    }

    #[inline(always)]
    fn restart(&mut self) {
        self.start_time = Instant::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn std_waiter() {
        let w = StdWaiter::new(Duration::from_millis(10), None);
        let mut t = w.start();
        assert!(!t.timeout());
        sleep(Duration::from_millis(1));
        assert!(!t.timeout());
        sleep(Duration::from_millis(9));
        assert!(t.timeout());
        assert!(t.timeout());

        let w = StdWaiter::new(Duration::from_millis(80), Some(Duration::from_millis(50)));
        let mut t = w.start();
        assert!(!t.timeout());
        assert!(!t.timeout());
        assert!(t.timeout());
        assert!(t.timeout());

        t.restart();
        assert!(!t.timeout());
        assert!(!t.timeout());
        assert!(t.timeout());
        assert!(t.timeout());
    }
}
