use std::sync::atomic::{AtomicUsize, Ordering};

use crate::PartialArray;

#[derive(Debug, Clone)]
struct DropCounter<'a>(&'a AtomicUsize);
impl Drop for DropCounter<'_> {
    fn drop(&mut self) {
        self.0.fetch_add(1, Ordering::Relaxed);
    }
}

#[test]
fn full() {
    let count = AtomicUsize::new(0);

    let partial_array: PartialArray<_, 4> = vec![
        DropCounter(&count),
        DropCounter(&count),
        DropCounter(&count),
        DropCounter(&count),
    ]
    .into_iter()
    .collect();
    drop(partial_array);

    assert_eq!(count.load(Ordering::Relaxed), 4);
}

#[test]
fn empty() {
    let count = AtomicUsize::new(0);

    let partial_array: PartialArray<i32, 4> = vec![].into_iter().collect();
    drop(partial_array);

    assert_eq!(count.load(Ordering::Relaxed), 0);
}

#[test]
fn partial() {
    let count = AtomicUsize::new(0);

    let partial_array: PartialArray<_, 4> = vec![DropCounter(&count), DropCounter(&count)]
        .into_iter()
        .collect();
    drop(partial_array);

    assert_eq!(count.load(Ordering::Relaxed), 2);
}

#[test]
fn iter_full() {
    let count = AtomicUsize::new(0);

    let partial_array: PartialArray<_, 4> = vec![
        DropCounter(&count),
        DropCounter(&count),
        DropCounter(&count),
        DropCounter(&count),
    ]
    .into_iter()
    .collect();
    drop(partial_array.into_iter());

    assert_eq!(count.load(Ordering::Relaxed), 4);
}

#[test]
fn iter_empty() {
    let count = AtomicUsize::new(0);

    let partial_array: PartialArray<i32, 4> = vec![].into_iter().collect();
    drop(partial_array.into_iter());

    assert_eq!(count.load(Ordering::Relaxed), 0);
}

#[test]
fn iter_partial() {
    let count = AtomicUsize::new(0);

    let partial_array: PartialArray<_, 4> = vec![DropCounter(&count), DropCounter(&count)]
        .into_iter()
        .collect();
    drop(partial_array.into_iter());

    assert_eq!(count.load(Ordering::Relaxed), 2);
}
