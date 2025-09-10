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
    fn start(&self) -> impl WaiterTime {
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

impl<'a> WaiterTime for StdWaiterInstance<'a> {
    #[inline]
    fn timeout(&mut self) -> bool {
        if self.start_time.elapsed() >= self.waiter.timeout {
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

impl TickInstant for Instant {
    #[inline(always)]
    fn now() -> Self {
        Instant::now()
    }

    #[inline(always)]
    fn tick_since(self, earlier: Self) -> u32 {
        self.duration_since(earlier).as_nanos() as u32
    }
}

pub struct StdInterval {
    dur: Duration,
}

impl StdInterval {
    /// - `dur`: the action in `interval()`.
    ///     - `Duration::ZERO`: call `yield_now()`
    ///     - `Duration`: call `sleep(Duration)`
    pub fn new(dur: Duration) -> Self {
        Self { dur }
    }
}

impl Interval for StdInterval {
    fn interval(&self) {
        match self.dur {
            Duration::ZERO => yield_now(),
            dur => sleep(dur),
        }
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

        let w = StdWaiter::new(Duration::from_millis(10), Some(Duration::from_millis(8)));
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
