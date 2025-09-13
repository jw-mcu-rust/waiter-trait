//! Traits used to wait and timeout in a `no-std` embedded system.
//!
//! ## Features
//!
//!- `std`: Disabled by default.
//!
//! # Examples
//!
//! ```
//! use waiter_trait::{Waiter, WaiterStatus, StdWaiter};
//! use std::time::Duration;
//!
//! // Initialize limit time and interval time.
//! let waiter = StdWaiter::new(Duration::from_millis(80), Some(Duration::from_millis(50)));
//!
//! fn foo(waiter: impl Waiter) {
//!     let mut t = waiter.start();
//!     loop {
//!         // Wait for something.
//!
//!         // Reset if it's necessary.
//!         t.restart();
//!
//!         if t.timeout() {
//!             break;
//!         }
//!     }
//! }
//! ```
//!
//! # Implementations
//!
//! For developers, you can choose one of the following options.
//! - Implement [`Waiter`], [`TimedWaiter`], and [`WaiterStatus`] then use them.
//! - Implement [`TickInstant`] and [`Interval`] then use [`TickWaiter`] or [`TimedTickWaiter`].
//!     - If you want to do nothing in the `interval()`, just give it [`NonInterval`],
//!       and in this way you can use `DelayNs` separately.
//! - Using [`Counter`], if you don't have any tick source.
//!
//! It also provides a implementation of `DelayNs` named [`TickDelay`]

#![cfg_attr(not(feature = "std"), no_std)]

mod counter;
pub use counter::*;
mod non_interval;
pub use non_interval::*;
mod tick_waiter;
pub use tick_waiter::*;
mod tick_delay;
pub use tick_delay::*;
mod timed_tick_waiter;
pub use timed_tick_waiter::*;

#[cfg(feature = "std")]
mod std_impls;
#[cfg(feature = "std")]
pub use std_impls::*;

pub use embedded_hal::delay::DelayNs;
pub use fugit::{self, MicrosDurationU32};

pub mod prelude;

pub trait Waiter {
    /// Start waiting.
    fn start(&self) -> impl WaiterStatus;
}

pub trait TimedWaiter {
    /// Set timeout and start waiting.
    fn start(&self, timeout: MicrosDurationU32) -> impl WaiterStatus;
}

pub trait WaiterStatus {
    /// Check if the time limit expires. This function may sleeps for a while,
    /// depends on the implementation.
    fn timeout(&mut self) -> bool;
    /// Reset the timeout condition.
    fn restart(&mut self);
}

pub trait TickInstant: Copy {
    fn now() -> Self;
    /// Returns the amount of ticks elapsed from another instant to this one.
    fn tick_since(self, earlier: Self) -> u32;
    /// Returns the amount of ticks elapsed since this instant.
    fn tick_elapsed(self) -> u32 {
        Self::now().tick_since(self)
    }
}

/// Can be implement for `yield`, `sleep` or do nothing.
pub trait Interval: Clone {
    fn interval(&self);
}
