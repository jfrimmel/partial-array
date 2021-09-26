use crate::PartialArray;

#[test]
fn empty() {
    let mut partial_array: PartialArray<u8, 3> = Default::default();
    partial_array.extend([1,2].iter().copied());
    assert_eq!(partial_array.len(), 2);
    assert_eq!(partial_array[0], 1);
    assert_eq!(partial_array[1], 2);
}

#[test]
fn nonempty() {
    let mut partial_array: PartialArray<u8, 3> = Default::default();
    partial_array.extend([1,2].iter().copied());
    partial_array.extend(Some(3));
    assert_eq!(partial_array.len(), 3);
    assert_eq!(partial_array[0], 1);
    assert_eq!(partial_array[2], 3);
}

#[test]
#[should_panic(expected = "Iterator has 1 elements to much")]
fn full() {
    let mut partial_array: PartialArray<u8, 3> = [1, 2, 3].iter().copied().collect();
    partial_array.extend(Some(4));
}
