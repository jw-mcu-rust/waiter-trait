//! Traits used to wait and timeout.
//!
//! ## Features
//!
//!- `std`: Disabled by default.
//!
//! # Examples
//!
//! ```
//! use waiter_trait::{Waiter, WaiterInstance, StdWaiter};
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
//!         {
//!             t.restart();
//!         }
//!
//!         if t.timeout() {
//!             break;
//!         }
//!     }
//! }
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

mod impls;
pub use impls::*;
#[cfg(feature = "std")]
mod std_impls;
#[cfg(feature = "std")]
pub use std_impls::*;

pub trait Waiter {
    /// Start waiting.
    fn start(&self) -> impl WaiterInstance;
}

pub trait WaiterInstance {
    /// Check if the time limit expires. This function may sleeps for a while,
    /// depends on the implementation.
    fn timeout(&mut self) -> bool;
    /// Reset the timeout condition.
    fn restart(&mut self);
}
