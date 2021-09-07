use crate::PartialArray;

#[test]
fn full() {
    let partial_array: PartialArray<u8, 4> = [0, 12, 24, 42].iter().copied().collect();
    assert_eq!(partial_array[0], 0);
    assert_eq!(partial_array[1], 12);
    assert_eq!(partial_array[2], 24);
    assert_eq!(partial_array[3], 42);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 4 but the index is 5")]
fn full_out_of_bounds() {
    let partial_array: PartialArray<u8, 4> = [0, 12, 24, 42].iter().copied().collect();
    partial_array[5];
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
fn empty_out_of_bounds() {
    let partial_array: PartialArray<u8, 4> = [].iter().copied().collect();
    partial_array[0];
}

#[test]
fn partial() {
    let partial_array: PartialArray<u8, 4> = [225, 1, 4].iter().copied().collect();
    assert_eq!(partial_array[0], 225);
    assert_eq!(partial_array[1], 1);
    assert_eq!(partial_array[2], 4);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 3 but the index is 3")]
fn partial_out_of_bounds() {
    let partial_array: PartialArray<u8, 4> = [225, 1, 4].iter().copied().collect();
    partial_array[3];
}

// TODO: test deref_mut
