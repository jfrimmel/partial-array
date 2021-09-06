use crate::PartialArray;

#[test]
fn full() {
    let partial_array: PartialArray<u8, 4> = [0, 1, 2, 3].iter().copied().collect();
    assert_eq!(partial_array.len(), 4);
    assert_eq!(partial_array, [0, 1, 2, 3]);
}

#[test]
fn empty() {
    let partial_array: PartialArray<u8, 4> = [].iter().copied().collect();
    assert_eq!(partial_array.len(), 0);
    assert_eq!(partial_array, []);
}

#[test]
fn partial() {
    let partial_array: PartialArray<u8, 4> = [0, 1, 2].iter().copied().collect();
    assert_eq!(partial_array.len(), 3);
    assert_eq!(partial_array, [0, 1, 2]);
}

#[test]
#[should_panic(expected = "Iterator has 2 elements to much")]
fn to_many() {
    let _partial_array: PartialArray<u8, 2> = [0, 1, 2, 3].iter().copied().collect();
}
