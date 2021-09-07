//! Potentially partial-filled arrays.
//!
//! This crate provides a central new data type, similar to an [array]: the
//! [`PartialArray<N>`]. It is equivalent to an array, but the number of entries
//! might be anywhere from `0` to `N`. While this has similarities to a `Vec<T>`
//! keep in mind, that a [`PartialArray`] does not grow its memory: it always
//! takes up the memory for the fully array (with some additional counter) and
//! it cannot ever hold more than `N` elements. This means that its memory is
//! fully static and on the stack, making it usable from `#![no_std]` crates.
//!
//! ## Usages
//! This new data type is most likely to be used for collecting iterators into
//! arrays, when then length is not known, but has an upper bound, e.g.:
//! ```
//! # use partial_array::PartialArray;
//! /// Take the first 10 elements of an iterator, that match the condition.
//! ///
//! /// This can return less than 10 elements if the iterator has fewer than 10
//! /// items or there are less than 10 matching elements.
//! fn first_10_matching<T, I, F>(iter: I, check: F) -> PartialArray<T, 10>
//!     where I: IntoIterator<Item = T>,
//!           F: FnMut(&T) -> bool,
//! {
//!     iter.into_iter().filter(check).take(10).collect()
//! }
//! ```
//! Aside from this main usage, the [`PartialArray`] can be used like a normal
//! array, i.e. it can be used as a [slice], you can convert [`from`] arrays and
//! [`try_from`] slices. You can also iterate over the [`PartialArray`]s by
//! value.
//! ```
//! # use partial_array::PartialArray;
//! let array = PartialArray::from([42_u16; 4]);
//! assert_eq!(array.len(), 4);
//! assert_eq!(array[0], 42);
//! assert_eq!(array[3], 42);
//! assert_eq!(array[2..].len(), 2);
//! array.into_iter().map(|x| x + 4).for_each(|x| println!("{}", x));
//! ```
//! As [`PartialArray`] implements [`IntoIterator`], you can use it in a `for`
//! loop directly:
//! ```
//! # use partial_array::partial_array;
//! let array = partial_array![42_u16; 4];
//! for item in array {
//!     println!("{}", item);
//! }
//! ```
//! This crate also provides a [macro] to make creating partial arrays easier:
//! ```
//! # use partial_array::partial_array;
//! let array = partial_array![42, -13, 2];
//! ```
//!
//! ## Behavior on out-of-bounds accesses
//! This crate simply panics on an out-of-bound access, both if you using more
//! than `N` items or if you use a non-initialized entry:
//! ```should_panic
//! # use partial_array::PartialArray;
//! // partial array is only filled half, the last entry is uninitialized and
//! // therefore out of bounds:
//! let mut array: PartialArray<i32, 4> = (0..2).collect();
//! array[2] = 42; // panic!
//! ```
//! ```should_panic
//! # use partial_array::PartialArray;
//! // partial array has less space than the iterator has items:
//! let _array: PartialArray<i32, 4> = (0..42).collect(); // panic!
//! ```
//!
//! [array]: prim@array
//! [slice]: prim@slice
//! [`from`]: core::convert::From::from
//! [`try_from`]: core::convert::TryFrom::try_from
//! [macro]: crate::partial_array
#![cfg_attr(not(test), no_std)] // allow `std` for tests

pub mod iter;

#[cfg(test)]
mod tests;

use core::fmt::{self, Debug, Formatter};
use core::iter::{FromIterator, IntoIterator};
use core::mem::{self, MaybeUninit};
use core::ops::{Deref, DerefMut};

/// A potentially partially filled array.
///
/// This is an array, with a length of at most `N`, but any value below that is
/// possible. It is mainly used as a [`iter.collect()`][collect]
/// target via the [`FromIterator`] trait.
/// ```
/// # use partial_array::PartialArray;
/// fn first_five_authors<'a>(names: &mut [&'a str]) -> PartialArray<&'a str, 5> {
///     names.sort();
///     names.iter().copied().take(5).collect() // can be less than 5 items
/// }
///
/// // e.g. works with 5 or more items, less than 5 or even none at all
/// assert_eq!(
///     first_five_authors(&mut ["a", "c", "b", "d", "f", "e"]),
///     ["a", "b", "c", "d", "e"],
/// );
/// assert_eq!(
///     first_five_authors(&mut ["Bela Writa", "A Nauthor"]),
///     ["A Nauthor", "Bela Writa"],
/// );
/// assert_eq!(first_five_authors(&mut []), []);
/// ```
///
/// It [deref]s to a slice, so you can execute the usual slice operations on it.
///
/// See the [crate-level-documentation](crate) for more information on the
/// intended usage.
///
/// [deref]: core::ops::Deref::deref
/// [collect]: Iterator::collect
pub struct PartialArray<T, const N: usize> {
    /// The number of filled entries inside the array.
    ///
    /// Each item in `0..filled` must be initialized. Others may or may not.
    /// This must never be greater than `N`.
    filled: usize,
    /// The actual storage for the items.
    ///
    /// This is an array of [`MaybeUninit`] items to prevent initialization of
    /// non-filled elements. There is the invariant: `filled` elements must be
    /// initialized and allowed to read independently.
    array: [MaybeUninit<T>; N],
}
impl<T, const N: usize> Deref for PartialArray<T, N> {
    /// A [`PartialArray<T, _>`] dereferences to a [slice of `T`][slice].
    type Target = [T];

    /// Dereference to the slice of filled elements (potentially less than `N`).
    fn deref(&self) -> &Self::Target {
        let slice = &self.array[..self.filled];
        // SAFETY: the invariant is, that `0..self.filled` is initialized, so it
        // is no UB reading those. The transmute itself is safe, since
        // `MaybeUninit` is `#[rpr(transparent)]`.
        unsafe { mem::transmute(slice) }
    }
}
impl<T, const N: usize> DerefMut for PartialArray<T, N> {
    /// Dereference to the slice of filled elements (potentially less than `N`).
    fn deref_mut(&mut self) -> &mut Self::Target {
        let slice = &mut self.array[..self.filled];
        // SAFETY: the invariant is, that `0..self.filled` is initialized, so it
        // is no UB reading those. The transmute itself is safe, since
        // `MaybeUninit` is `#[rpr(transparent)]`.
        unsafe { mem::transmute(slice) }
    }
}
impl<T: Debug, const N: usize> Debug for PartialArray<T, N> {
    /// Debug-format the slice of filled elements (potentially less than `N`).
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        <[T] as Debug>::fmt(self, f)
    }
}
impl<T: PartialEq, const N: usize, const M: usize> PartialEq<PartialArray<T, M>>
    for PartialArray<T, N>
{
    /// Compare the filled elements of [`PartialArray`]s.
    ///
    /// Two [`PartialArray`]s can be compared even if their lengths do not
    /// match. Only the number of filled elements and their values are compared.
    ///
    /// # Example
    /// ```
    /// # use partial_array::PartialArray;
    /// let a: PartialArray<u8, 5> = (0..4).collect();
    /// let b: PartialArray<u8, 500> = (0..4).collect();
    ///
    /// assert_eq!(a, b);
    /// ```
    fn eq(&self, other: &PartialArray<T, M>) -> bool {
        self.len() == other.len() && self.deref() == other.deref()
    }
}
impl<T: PartialEq, const N: usize, const M: usize> PartialEq<[T; M]> for PartialArray<T, N> {
    /// Compare a [`PartialArray`] with a normal array.
    ///
    /// This compares the filled elements (potentially less than `N`).
    ///
    /// # Example
    /// ```
    /// # use partial_array::PartialArray;
    /// let a: PartialArray<u8, 5> = (10..15).collect();
    /// let b = [10, 11, 12, 13, 14];
    ///
    /// assert_eq!(a, b);
    ///
    /// // the other way round is also possible.
    /// assert_eq!(b, a);
    /// ```
    fn eq(&self, other: &[T; M]) -> bool {
        self.len() == other.len() && self.deref() == other.deref()
    }
}
impl<T: PartialEq, const N: usize, const M: usize> PartialEq<PartialArray<T, M>> for [T; N] {
    /// Compare a normal array with a [`PartialArray`].
    ///
    /// This compares the filled elements (potentially less than `N`).
    ///
    /// # Example
    /// ```
    /// # use partial_array::PartialArray;
    /// let a = [10, 11, 12, 13, 14];
    /// let b: PartialArray<u8, 5> = (10..15).collect();
    ///
    /// assert_eq!(a, b);
    ///
    /// // the other way round is also possible.
    /// assert_eq!(b, a);
    /// ```
    fn eq(&self, other: &PartialArray<T, M>) -> bool {
        self.len() == other.len() && self.deref() == other.deref()
    }
}
impl<T: PartialEq, const N: usize> PartialEq<&[T]> for PartialArray<T, N> {
    /// Compare the slice of filled elements (potentially less than `N`).
    ///
    /// # Example
    /// ```
    /// # use partial_array::PartialArray;
    /// let a: PartialArray<u8, 5> = (10..15).collect();
    /// let b = &[10, 11, 12, 13, 14][..];
    ///
    /// assert_eq!(a, b);
    /// ```
    fn eq(&self, other: &&[T]) -> bool {
        self.len() == other.len() && self.deref() == other.deref()
    }
}
impl<T: PartialEq, const N: usize> PartialEq<PartialArray<T, N>> for &[T] {
    /// Compare a slice with a [`PartialArray`].
    ///
    /// This compares the filled elements (potentially less than `N`).
    ///
    /// # Example
    /// ```
    /// # use partial_array::PartialArray;
    /// let a: &[u8] = &[10, 11, 12, 13, 14];
    /// let b: PartialArray<u8, 5> = (10..15).collect();
    ///
    /// assert_eq!(a, b);
    ///
    /// // the other way round is also possible.
    /// assert_eq!(b, a);
    /// ```
    fn eq(&self, other: &PartialArray<T, N>) -> bool {
        self.len() == other.len() && self.deref() == other.deref()
    }
}
impl<T: Eq, const N: usize> Eq for PartialArray<T, N> {}
impl<T, const N: usize> Default for PartialArray<T, N> {
    /// Initialize an empty [`PartialArray`].
    fn default() -> Self {
        Self {
            array: [Self::UNINIT; N],
            filled: 0,
        }
    }
}
impl<T, const N: usize> PartialArray<T, N> {
    /// Required for `MaybeUninit::uninit()` in array initializers
    const UNINIT: MaybeUninit<T> = MaybeUninit::uninit();
}
impl<T, const N: usize> FromIterator<T> for PartialArray<T, N> {
    /// Build up a [`PartialArray`] from an iterator with potentially less than
    /// `N` elements.
    ///
    /// # Example
    /// ```
    /// # use partial_array::PartialArray;
    /// // a set of channels set to different values
    /// let mut channels = [12, 13, 8, 12, 255, 8, 8, 8];
    ///
    /// // we want to only have the distinct channel values
    /// channels.sort_unstable();
    /// let distinct_channels: PartialArray<_, 8> = channels
    ///     .windows(2)
    ///     .chain(Some(&[channels[7], 0][..]))
    ///     .filter(|window| window[0] != window[1])
    ///     .map(|window| window[0])
    ///     .collect();
    ///
    /// assert_eq!(distinct_channels.len(), 4);
    /// assert_eq!(distinct_channels, [8, 12, 13, 255]);
    /// ```
    ///
    /// # Panics
    /// Panics, if the length of the iterator os greater than the maximum length
    /// of the array (`N`).
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut result = Self::default();
        result.extend(iter);
        result
    }
}
impl<T, const N: usize> Extend<T> for PartialArray<T, N> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        let remaining = (self.filled..N).len();
        let mut iter = iter.into_iter();

        iter.by_ref()
            .take(remaining)
            .enumerate()
            .for_each(|(i, element)| {
                self.array[i] = MaybeUninit::new(element);
                self.filled += 1;
            });

        // check, that there are no more elements left
        let remaining = iter.count();
        assert_eq!(remaining, 0, "Iterator has {} elements to much", remaining);
    }
}
impl<T, const N: usize> IntoIterator for PartialArray<T, N> {
    type Item = T;
    type IntoIter = iter::IntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        iter::IntoIter::new(self)
    }
}
// TODO: generalize to From<[T; M]> for PartialArray<T, N> where M <= N
impl<T, const N: usize> From<[T; N]> for PartialArray<T, N> {
    fn from(array: [T; N]) -> Self {
        // TODO: is there a more performant way? Maybe with unsafe
        core::array::IntoIter::new(array).collect()
    }
}

/// Create a partial array from a given set of values (similar to `vec![]`).
///
/// # Example
/// ```
/// use partial_array::{partial_array, PartialArray};
///
/// assert_eq!(partial_array![0, 1, 2], PartialArray::from([0, 1, 2]));
/// assert_eq!(partial_array![17, 12, 2, ], PartialArray::from([17, 12, 2]));
/// assert_eq!(partial_array![42; 5], PartialArray::from([42; 5]));
/// ```
#[macro_export]
macro_rules! partial_array {
    ($($element:expr),*$(,)?) => {
        $crate::PartialArray::from([$($element),*])
    };
    ($element:expr; $n: literal) => {
        $crate::PartialArray::from([$element; $n])
    };
}
