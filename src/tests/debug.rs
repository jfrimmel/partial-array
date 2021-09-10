use crate::{partial_array, PartialArray};

#[test]
fn full() {
    let partial_array: PartialArray<u8, 4> = [0, 1, 2, 3].iter().copied().collect();
    assert_eq!(format!("{:?}", partial_array), "[0, 1, 2, 3]");
    assert_eq!(format!("{:?}", partial_array.into_iter()), "[0, 1, 2, 3]");
}

#[test]
fn empty() {
    let partial_array: PartialArray<u8, 4> = [].iter().copied().collect();
    assert_eq!(format!("{:?}", partial_array), "[]");
    assert_eq!(format!("{:?}", partial_array.into_iter()), "[]");
}

#[test]
fn partial() {
    let partial_array: PartialArray<u8, 42> = [0, 1, 2].iter().copied().collect();
    assert_eq!(format!("{:?}", partial_array), "[0, 1, 2]");
    assert_eq!(format!("{:?}", partial_array.into_iter()), "[0, 1, 2]");
}

#[test]
fn iter() {
    let mut iter = PartialArray::from([1, 2, 3, 4, 5]).into_iter();
    assert_eq!(format!("{:?}", iter), "[1, 2, 3, 4, 5]");
    iter.next();
    assert_eq!(format!("{:?}", iter), "[2, 3, 4, 5]");
    iter.next_back();
    assert_eq!(format!("{:?}", iter), "[2, 3, 4]");
    iter.by_ref().for_each(drop);
    assert_eq!(format!("{:?}", iter), "[]");
}

#[test]
fn debugability() {
    fn assert<T: core::fmt::Debug>(_: T) {}

    assert(partial_array![" "]);
    assert(partial_array![1, 2, 7]);
    assert(partial_array![0.425, -0.0]);
    assert(partial_array![1, 2, 7]);
    assert(partial_array![" "].into_iter());
    assert(partial_array![4.07].into_iter());
}
