use crate::PartialArray;
use core::mem;

#[test]
fn size_of() {
    assert_eq!(
        mem::size_of::<[u8; 32]>() + mem::size_of::<usize>(),
        mem::size_of::<PartialArray<u8, 32>>()
    );
    assert_eq!(
        mem::size_of::<[String; 12]>() + mem::size_of::<usize>(),
        mem::size_of::<PartialArray<String, 12>>()
    );
}
