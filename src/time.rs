use core::ops::AddAssign;
use core::ops::SubAssign;
use core::time::Duration;

/// A trait to abstract a temporal instant
///
/// Instants are monotonically increasing.
pub trait Instant: Clone + Copy + SubAssign<Duration> + AddAssign<Duration> {
    /// Return the current instant
    fn now() -> Self;

    /// Compute the duration since this instant
    fn elapsed(&self) -> core::time::Duration;
}

/// A trait to abstract a sleep function
pub trait Sleeper {
    /// Sleep for the specified duration
    fn sleep(&self, duration: core::time::Duration);
}

/// A trait to abstract an asynchronous sleep function
pub trait AsyncSleeper {
    /// Sleep for the specified duration
    fn sleep(
        &self,
        duration: core::time::Duration,
    ) -> impl core::future::Future<Output = ()> + Send;
}

#[cfg(feature = "std")]
impl Instant for std::time::Instant {
    fn now() -> Self {
        std::time::Instant::now()
    }

    fn elapsed(&self) -> core::time::Duration {
        self.elapsed()
    }
}

/// A sleeper that calls `std::thread::sleep()`
#[cfg(feature = "std")]
#[derive(Default)]
pub struct StdSleeper;

#[cfg(feature = "std")]
impl Sleeper for StdSleeper {
    fn sleep(&self, duration: core::time::Duration) {
        std::thread::sleep(duration)
    }
}

/// A sleeper that calls `spin_sleep::sleep()`
#[cfg(feature = "spin_sleep")]
#[derive(Default)]
pub struct SpinSleeper;

#[cfg(feature = "spin_sleep")]
impl Sleeper for SpinSleeper {
    fn sleep(&self, duration: core::time::Duration) {
        spin_sleep::sleep(duration)
    }
}
