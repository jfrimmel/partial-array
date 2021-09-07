use crate::PartialArray;

#[test]
fn partial_array_is_partial_eq_if_t_is() {
    fn assert<T: PartialEq>(_: T) {}

    assert(PartialArray::<u8, 5>::default());
    assert(PartialArray::<f32, 1>::default());
    assert(PartialArray::<String, 4>::default());
}

#[test]
fn partial_array_is_eq_if_t_is() {
    fn assert<T: Eq>(_: T) {}

    assert(PartialArray::<u8, 5>::default());
    assert(PartialArray::<String, 4>::default());
}

#[test]
fn slice_comparison() {
    let a = PartialArray::<f32, 3>::from([0.0, 0.5, 0.75]);
    let b: &[f32] = &[0.0, 0.5, 0.75];
    let c: &[f32] = &[0.1, 0.5, 0.75];

    assert_eq!(a, b);
    assert_eq!(b, a);

    assert_ne!(a, c);
    assert_ne!(c, a);
    assert_ne!(b, c);
    assert_ne!(c, b);
}

#[test]
fn array_comparison() {
    let a = PartialArray::<f32, 3>::from([0.0, 0.5, 0.75]);
    let b: [f32; 3] = [0.0, 0.5, 0.75];
    let c: [f32; 3] = [0.1, 0.5, 0.75];

    assert_eq!(a, b);
    assert_eq!(b, a);

    assert_ne!(a, c);
    assert_ne!(c, a);
    assert_ne!(b, c);
    assert_ne!(c, b);
}

#[test]
fn partial_array_comparison() {
    let a = PartialArray::<f32, 3>::from([0.0, 0.5, 0.75]);
    let b = PartialArray::<f32, 3>::from([0.0, 0.5, 0.75]);
    let c = PartialArray::<f32, 3>::from([0.1, 0.5, 0.75]);

    assert_eq!(a, b);
    assert_eq!(b, a);

    assert_ne!(a, c);
    assert_ne!(c, a);
    assert_ne!(b, c);
    assert_ne!(c, b);
}
