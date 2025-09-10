use super::*;
use core::marker::PhantomData;
use embedded_hal::delay::DelayNs;

pub struct TickDelay<T, INS, IUS> {
    frequency: u32,
    interval_ns: INS,
    interval_us: IUS,
    _t: PhantomData<T>,
}

impl<T, INS, IUS> TickDelay<T, INS, IUS>
where
    INS: Interval,
    IUS: Interval,
    T: TickInstant,
{
    pub fn new(frequency: u32, interval_ns: INS, interval_us: IUS) -> Self {
        Self {
            frequency,
            interval_ns,
            interval_us,
            _t: PhantomData,
        }
    }
}

impl<T, INS, IUS> DelayNs for TickDelay<T, INS, IUS>
where
    INS: Interval,
    IUS: Interval,
    T: TickInstant,
{
    fn delay_ns(&mut self, ns: u32) {
        let w = TickWaiter::<T, _, _>::ns(ns as u64, self.interval_ns.clone(), self.frequency);
        let mut t = w.start();
        while !t.timeout() {}
    }

    fn delay_us(&mut self, us: u32) {
        let w = TickWaiter::<T, _, _>::us(us, self.interval_us.clone(), self.frequency);
        let mut t = w.start();
        while !t.timeout() {}
    }
}

#[cfg(feature = "std")]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::std_impls::*;
    use std::time::{Duration, Instant};

    #[test]
    fn tick_delay() {
        let mut d = TickDelay::<Instant, _, _>::new(
            Duration::from_secs(1).as_nanos() as u32,
            NonInterval::new(),
            StdInterval::new(Duration::ZERO),
        );

        let t = Instant::now();
        d.delay_ns(10_000);
        let elapsed = t.elapsed();
        assert!(elapsed - Duration::from_nanos(10_000) < Duration::from_nanos(500));

        let t = Instant::now();
        d.delay_us(1000);
        let elapsed = t.elapsed();
        assert!(elapsed - Duration::from_micros(1000) < Duration::from_micros(500));
    }
}
