use crate::PartialArray;

#[test]
fn full() {
    let partial_array: PartialArray<u8, 4> = [0, 1, 2, 3].iter().copied().collect();
    assert_eq!(format!("{:?}", partial_array), "[0, 1, 2, 3]");
}

#[test]
fn empty() {
    let partial_array: PartialArray<u8, 4> = [].iter().copied().collect();
    assert_eq!(format!("{:?}", partial_array), "[]");
}

#[test]
fn partial() {
    let partial_array: PartialArray<u8, 42> = [0, 1, 2].iter().copied().collect();
    assert_eq!(format!("{:?}", partial_array), "[0, 1, 2]");
}
