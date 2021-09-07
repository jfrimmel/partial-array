//! Types for external iteration.
//!
//! This module provides the [`iter::IntoIter`] type, which is an by-value
//! iterator over a [`PartialArray`]. You most likely do not need to interact
//! with this module directly. One exception is, when you explicitly need to
//! store the iterator. In the following example a local variable is explicitly
//! annotated, but normally one would use just type-inference.
//! ```
//! # use partial_array::partial_array;
//! let array = partial_array![2, 4, 8, 16, 32, 64];
//! let iter: partial_array::iter::IntoIter<_, 6> = array.into_iter();
//! for (i, value) in iter.enumerate() {
//!     println!("Item #{}: {}", i, value);
//! }
//! ```
//!
//! [`iter::IntoIter`]: IntoIter
use crate::PartialArray;
use core::iter::FusedIterator;
use core::mem::{self, MaybeUninit};

/// An iterator that moves out of a [`PartialArray`], therefore an owning
/// by-value iterator.
///
/// This struct is created by the [`into_iter`] method on Vec (provided by
/// the [`IntoIterator`] trait).
///
/// # Example
/// ```
/// # use partial_array::PartialArray;
/// let v = PartialArray::<u8, 3>::from([0, 1, 2]);
/// let iter: partial_array::iter::IntoIter<_, 3> = v.into_iter();
/// ```
///
/// [`into_iter`]: IntoIterator::into_iter
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub struct IntoIter<T, const N: usize> {
    // invariant: `read..filled` has to be initialized
    array: [MaybeUninit<T>; N],
    filled: usize,
    read: usize,
}
impl<T, const N: usize> IntoIter<T, N> {
    /// Create a new [`IntoIter<T, N>`] from a [`PartialArray<T, N>`].
    pub(crate) fn new(array: PartialArray<T, N>) -> Self {
        Self {
            array: array.array,
            filled: array.filled,
            read: 0,
        }
    }
}
impl<T, const N: usize> Iterator for IntoIter<T, N> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.read != self.filled {
            let value = mem::replace(&mut self.array[self.read], PartialArray::<_, N>::UNINIT);
            self.read += 1;
            Some(unsafe { value.assume_init() })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.filled - self.read;
        (len, Some(len))
    }
}
impl<T, const N: usize> DoubleEndedIterator for IntoIter<T, N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.read != self.filled && self.filled > 0 {
            self.filled -= 1;
            let value = mem::replace(&mut self.array[self.filled], PartialArray::<_, N>::UNINIT);
            Some(unsafe { value.assume_init() })
        } else {
            None
        }
    }
}
impl<T, const N: usize> FusedIterator for IntoIter<T, N> {}
impl<T, const N: usize> ExactSizeIterator for IntoIter<T, N> {}
