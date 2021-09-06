mod forward {
    use crate::PartialArray;

    #[test]
    fn full() {
        let partial_array: PartialArray<u8, 4> = [0, 1, 2, 3].iter().copied().collect();
        let mut iter = partial_array.into_iter();
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn empty() {
        let partial_array: PartialArray<u8, 4> = [].iter().copied().collect();
        let mut iter = partial_array.into_iter();
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn partial() {
        let partial_array: PartialArray<u8, 42> = [0, 1, 2].iter().copied().collect();
        let mut iter = partial_array.into_iter();
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), None);
    }
}

mod reverse {
    use crate::PartialArray;

    #[test]
    fn full() {
        let partial_array: PartialArray<u8, 4> = [0, 1, 2, 3].iter().copied().collect();
        let mut iter = partial_array.into_iter();
        assert_eq!(iter.next_back(), Some(3));
        assert_eq!(iter.next_back(), Some(2));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next_back(), Some(0));
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn empty() {
        let partial_array: PartialArray<u8, 4> = [].iter().copied().collect();
        let mut iter = partial_array.into_iter();
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn partial() {
        let partial_array: PartialArray<u8, 42> = [0, 1, 2].iter().copied().collect();
        let mut iter = partial_array.into_iter();
        assert_eq!(iter.next_back(), Some(2));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next_back(), Some(0));
        assert_eq!(iter.next_back(), None);
    }
}

mod forward_reverse {
    use crate::PartialArray;

    #[test]
    fn full() {
        let partial_array: PartialArray<u8, 4> = [0, 1, 2, 3].iter().copied().collect();
        let mut iter = partial_array.into_iter();
        assert_eq!(iter.next_back(), Some(3));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next_back(), Some(2));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn partial() {
        let partial_array: PartialArray<u8, 42> = [0, 1, 2].iter().copied().collect();
        let mut iter = partial_array.into_iter();
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next_back(), Some(2));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }
}
