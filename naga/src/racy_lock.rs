#[cfg(not(feature = "std"))]
use alloc::boxed::Box;
#[cfg(not(feature = "std"))]
use once_cell::race::OnceBox;
#[cfg(feature = "std")]
use std::sync::LazyLock;

#[cfg(feature = "std")]
type Inner<T> = LazyLock<T, fn() -> T>;
#[cfg(not(feature = "std"))]
type Inner<T> = OnceBox<T>;

/// Lazy static helper that uses [`LazyLock`] with `std` and [`OnceBox`] otherwise.
///
/// [`LazyLock`]: https://doc.rust-lang.org/stable/std/sync/struct.LazyLock.html
pub struct RacyLock<T: 'static> {
    inner: Inner<T>,
    #[cfg(not(feature = "std"))]
    init: fn() -> T,
}

impl<T: 'static> RacyLock<T> {
    #[cfg(feature = "std")]
    /// Creates a new [`RacyLock`], which will initialize using the provided `init` function.
    pub const fn new(init: fn() -> T) -> Self {
        Self {
            inner: LazyLock::new(init),
        }
    }

    #[cfg(not(feature = "std"))]
    /// Creates a new [`RacyLock`], which will initialize using the provided `init` function.
    pub const fn new(init: fn() -> T) -> Self {
        Self {
            inner: OnceBox::new(),
            init,
        }
    }
}

#[cfg(feature = "std")]
impl<T: 'static> core::ops::Deref for RacyLock<T> {
    type Target = T;

    /// Loads the internal value, initializing it if required.
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[cfg(not(feature = "std"))]
impl<T: 'static> core::ops::Deref for RacyLock<T> {
    type Target = T;

    /// Loads the internal value, initializing it if required.
    fn deref(&self) -> &Self::Target {
        self.inner.get_or_init(|| Box::new((self.init)()))
    }
}
