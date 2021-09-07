//! Test, that this crate can be used in `#![no_std]` environments by default.
// This has to be an integration test, as we need to compile a separate crate
// with the `#![no_std]` attribute, which is not possible with plain unit tests
// (note, that the root crate has `#![cfg_attr(not(test), no_std)]`, i.e. it
// requires `std` for tests, just not for the normal API).

#![no_std]
use partial_array::PartialArray;

#[test]
fn partial_array() {
    let _a: PartialArray<u32, 17> = (0..42).map(|x| x + 112).take(12).collect();
}

#[test]
fn into_iter() {
    let array = PartialArray::<u32, 17>::from([1; 17]);
    assert_eq!(array.into_iter().count(), 17);
}
