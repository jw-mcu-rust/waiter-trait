use super::*;

/// The timeout condition is independent of time
/// and is determined solely by the number of times `timeout()` is called.
pub struct Counter {
    retry_times: usize,
}

impl Counter {
    /// - `retry_times`: The number of calls to `timeout()` before it returns `true`.
    ///     - If set to `0`, `timeout()` always returns `true`
    ///     - If set to `usize::MAX`, `timeout()` always returns `false`
    pub fn new(retry_times: usize) -> Self {
        Self { retry_times }
    }
}

impl Waiter for Counter {
    #[inline]
    fn start(&self) -> impl WaiterInstance {
        CounterInstance {
            count: 0,
            waiter: self,
        }
    }
}

pub struct CounterInstance<'a> {
    count: usize,
    waiter: &'a Counter,
}

impl<'a> WaiterInstance for CounterInstance<'a> {
    #[inline]
    fn timeout(&mut self) -> bool {
        if self.waiter.retry_times == usize::MAX {
            return false;
        }

        if self.count < self.waiter.retry_times {
            self.count = self.count.wrapping_add(1);
            false
        } else {
            true
        }
    }

    #[inline(always)]
    fn restart(&mut self) {
        self.count = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter() {
        let c = Counter::new(0);
        let mut t = c.start();
        assert!(t.timeout());
        assert!(t.timeout());

        let c = Counter::new(usize::MAX);
        let mut t = c.start();
        assert!(!t.timeout());
        assert!(!t.timeout());

        let c = Counter::new(2);
        let mut t = c.start();
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
