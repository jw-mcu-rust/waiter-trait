use super::*;

/// Do nothing for interval
#[derive(Default)]
pub struct NonInterval {}

impl NonInterval {
    pub fn new() -> Self {
        Self {}
    }
}

impl Interval for NonInterval {
    #[inline(always)]
    fn interval(&self) {}
}
