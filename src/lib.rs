//! Traits used to wait and timeout in a `no-std` embedded system.
//!
//! ## Features
//!
//!- `std`: Disabled by default.
//!
//! # Examples
//!
//! ```
//! use waiter_trait::{Waiter, WaiterTime, StdWaiter};
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
//! ## Implementations
//!
//! For developers, you can choose one of the following options.
//! - Implement [`Waiter`] and [`WaiterTime`] then use them.
//! - Implement [`TickInstant`] and [`Interval`] then use [`TickWaiter`].
//!     - If you want to do nothing in `interval()`, just use [`NonInterval`].
//! - If you can't use a timer, you can consider to use [`Counter`]

#![cfg_attr(not(feature = "std"), no_std)]

mod counter;
pub use counter::*;
mod non_interval;
pub use non_interval::*;
mod tick_waiter;
pub use tick_waiter::*;
mod tick_delay;
pub use tick_delay::*;
#[cfg(feature = "std")]
mod std_impls;
#[cfg(feature = "std")]
pub use std_impls::*;

pub trait Waiter {
    /// Start waiting.
    fn start(&self) -> impl WaiterTime;
}

pub trait WaiterTime {
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
}

/// Can be implement for `yield`, `sleep` or do nothing.
pub trait Interval: Clone {
    fn interval(&self);
}
