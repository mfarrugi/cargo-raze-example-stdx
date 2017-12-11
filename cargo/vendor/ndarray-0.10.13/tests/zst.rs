
extern crate ndarray;

use ndarray::arr2;
use ndarray::RcArray;

#[test]
fn test_swap() {
    let mut a = arr2(&[[();3];3]);

    let b = a.clone();

    for i in 0..a.rows() {
        for j in i + 1..a.cols() {
            a.swap((i, j), (j, i));
        }
    }
    assert_eq!(a, b.t());
}

#[test]
fn test() {
    let c = RcArray::<(), _>::default((3, 4));
    let mut d = c.clone();
    for _ in d.iter_mut() {}
}
