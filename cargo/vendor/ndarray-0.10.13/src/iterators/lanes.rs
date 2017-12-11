
use imp_prelude::*;
use {NdProducer, Layout};
use super::LanesIter;
use super::LanesIterMut;

impl_ndproducer! {
    ['a, A, D: Dimension]
    [Clone => 'a, A, D: Clone ]
    Lanes {
        base,
        inner_len,
        inner_stride,
    }
    Lanes<'a, A, D> {
        type Item = ArrayView<'a, A, Ix1>;
        type Dim = D;

        unsafe fn item(&self, ptr) {
            ArrayView::new_(ptr, Ix1(self.inner_len), Ix1(self.inner_stride as Ix))
        }
    }
}

/// See [`.lanes()`](../struct.ArrayBase.html#method.lanes)
/// for more information.
pub struct Lanes<'a, A: 'a, D> {
    base: ArrayView<'a, A, D>,
    inner_len: Ix,
    inner_stride: Ixs,
}


pub fn new_lanes<A, D>(v: ArrayView<A, D>, axis: Axis)
    -> Lanes<A, D::Smaller>
    where D: Dimension
{
    let ndim = v.ndim();
    let len;
    let stride;
    let iter_v;
    if ndim == 0 {
        len = 1;
        stride = 1;
        iter_v = v.try_remove_axis(Axis(0))
    } else {
        let i = axis.index();
        len = v.dim[i];
        stride = v.strides[i] as isize;
        iter_v = v.try_remove_axis(axis)
    }
    Lanes {
        inner_len: len,
        inner_stride: stride,
        base: iter_v,
    }
}

impl_ndproducer! {
    ['a, A, D: Dimension]
    [Clone =>]
    LanesMut {
        base,
        inner_len,
        inner_stride,
    }
    LanesMut<'a, A, D> {
        type Item = ArrayViewMut<'a, A, Ix1>;
        type Dim = D;

        unsafe fn item(&self, ptr) {
            ArrayViewMut::new_(ptr, Ix1(self.inner_len), Ix1(self.inner_stride as Ix))
        }
    }
}

impl<'a, A, D> IntoIterator for Lanes<'a, A, D>
    where D: Dimension,
{
    type Item = <Self::IntoIter as Iterator>::Item;
    type IntoIter = LanesIter<'a, A, D>;
    fn into_iter(self) -> Self::IntoIter {
        LanesIter {
            iter: self.base.into_base_iter(),
            inner_len: self.inner_len,
            inner_stride: self.inner_stride,
        }
    }
}

/// See [`.lanes_mut()`](../struct.ArrayBase.html#method.lanes_mut)
/// for more information.
pub struct LanesMut<'a, A: 'a, D> {
    base: ArrayViewMut<'a, A, D>,
    inner_len: Ix,
    inner_stride: Ixs,
}


pub fn new_lanes_mut<A, D>(v: ArrayViewMut<A, D>, axis: Axis)
    -> LanesMut<A, D::Smaller>
    where D: Dimension
{
    let ndim = v.ndim();
    let len;
    let stride;
    let iter_v;
    if ndim == 0 {
        len = 1;
        stride = 1;
        iter_v = v.try_remove_axis(Axis(0))
    } else {
        let i = axis.index();
        len = v.dim[i];
        stride = v.strides[i] as isize;
        iter_v = v.try_remove_axis(axis)
    }
    LanesMut {
        inner_len: len,
        inner_stride: stride,
        base: iter_v,
    }
}

impl<'a, A, D> IntoIterator for LanesMut<'a, A, D>
    where D: Dimension,
{
    type Item = <Self::IntoIter as Iterator>::Item;
    type IntoIter = LanesIterMut<'a, A, D>;
    fn into_iter(self) -> Self::IntoIter {
        LanesIterMut {
            iter: self.base.into_base_iter(),
            inner_len: self.inner_len,
            inner_stride: self.inner_stride,
        }
    }
}
