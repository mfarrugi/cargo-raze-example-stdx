// Copyright 2014-2016 bluss and ndarray developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::cmp;
use std::ptr as std_ptr;
use std::slice;

use itertools::zip;

use imp_prelude::*;

use arraytraits;
use dimension;
use iterators;
use error::{self, ShapeError, ErrorKind};
use dimension::IntoDimension;
use dimension::{axes_of, Axes, merge_axes, stride_offset};
use iterators::{
    new_lanes,
    new_lanes_mut,
    exact_chunks_of,
    exact_chunks_mut_of,
    windows
};
use zip::Zip;

use {
    NdIndex,
};
use iter::{
    AxisChunksIter,
    AxisChunksIterMut,
    Iter,
    IterMut,
    IndexedIter,
    IndexedIterMut,
    Lanes,
    LanesMut,
    AxisIter,
    AxisIterMut,
    ExactChunks,
    ExactChunksMut,
    Windows
};
use stacking::stack;

/// # Methods For All Array Types
impl<A, S, D> ArrayBase<S, D> where S: Data<Elem=A>, D: Dimension
{
    /// Return the total number of elements in the array.
    pub fn len(&self) -> usize {
        self.dim.size()
    }

    /// Return the length of `axis`.
    ///
    /// The axis should be in the range `Axis(` 0 .. *n* `)` where *n* is the
    /// number of dimensions (axes) of the array.
    ///
    /// ***Panics*** if the axis is out of bounds.
    pub fn len_of(&self, axis: Axis) -> usize {
        self.dim[axis.index()]
    }

    /// Return whether the array has any elements
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Return the number of dimensions (axes) in the array
    pub fn ndim(&self) -> usize {
        self.dim.ndim()
    }

    /// Return the shape of the array in its “pattern” form,
    /// an integer in the one-dimensional case, tuple in the n-dimensional cases
    /// and so on.
    pub fn dim(&self) -> D::Pattern {
        self.dim.clone().into_pattern()
    }

    /// Return the shape of the array as it stored in the array.
    pub fn raw_dim(&self) -> D {
        self.dim.clone()
    }

    /// Return the shape of the array as a slice.
    pub fn shape(&self) -> &[Ix] {
        self.dim.slice()
    }

    /// Return the strides of the array as a slice
    pub fn strides(&self) -> &[Ixs] {
        let s = self.strides.slice();
        // reinterpret unsigned integer as signed
        unsafe {
            slice::from_raw_parts(s.as_ptr() as *const _, s.len())
        }
    }

    /// Return a read-only view of the array
    pub fn view(&self) -> ArrayView<A, D> {
        debug_assert!(self.pointer_is_inbounds());
        unsafe {
            ArrayView::new_(self.ptr, self.dim.clone(), self.strides.clone())
        }
    }

    /// Return a read-write view of the array
    pub fn view_mut(&mut self) -> ArrayViewMut<A, D>
        where S: DataMut,
    {
        self.ensure_unique();
        unsafe {
            ArrayViewMut::new_(self.ptr, self.dim.clone(), self.strides.clone())
        }
    }

    /// Return an uniquely owned copy of the array
    pub fn to_owned(&self) -> Array<A, D>
        where A: Clone
    {
        if let Some(slc) = self.as_slice_memory_order() {
            unsafe {
                Array::from_shape_vec_unchecked(self.dim.clone()
                                                .strides(self.strides.clone()),
                                                slc.to_vec())
            }
        } else {
            self.map(|x| x.clone())
        }
    }

    /// Return a shared ownership (copy on write) array.
    pub fn to_shared(&self) -> RcArray<A, D>
        where A: Clone
    {
        // FIXME: Avoid copying if it’s already an RcArray.
        self.to_owned().into_shared()
    }

    /// Turn the array into a uniquely owned array, cloning the array elements
    /// to unshare them if necessary.
    pub fn into_owned(self) -> Array<A, D>
        where A: Clone,
              S: DataOwned,
    {
        S::into_owned(self)
    }

    /// Turn the array into a shared ownership (copy on write) array,
    /// without any copying.
    pub fn into_shared(self) -> RcArray<A, D>
        where S: DataOwned,
    {
        let data = self.data.into_shared();
        ArrayBase {
            data: data,
            ptr: self.ptr,
            dim: self.dim,
            strides: self.strides,
        }
    }

    /// Return an iterator of references to the elements of the array.
    ///
    /// Elements are visited in the *logical order* of the array, which
    /// is where the rightmost index is varying the fastest.
    ///
    /// Iterator element type is `&A`.
    pub fn iter(&self) -> Iter<A, D> {
        debug_assert!(self.pointer_is_inbounds());
        self.view().into_iter_()
    }

    /// Return an iterator of mutable references to the elements of the array.
    ///
    /// Elements are visited in the *logical order* of the array, which
    /// is where the rightmost index is varying the fastest.
    ///
    /// Iterator element type is `&mut A`.
    pub fn iter_mut(&mut self) -> IterMut<A, D>
        where S: DataMut,
    {
        self.view_mut().into_iter_()
    }

    /// Return an iterator of indexes and references to the elements of the array.
    ///
    /// Elements are visited in the *logical order* of the array, which
    /// is where the rightmost index is varying the fastest.
    ///
    /// Iterator element type is `(D::Pattern, &A)`.
    ///
    /// See also [`Zip::indexed`](struct.Zip.html)
    pub fn indexed_iter(&self) -> IndexedIter<A, D> {
        IndexedIter::new(self.view().into_elements_base())
    }

    /// Return an iterator of indexes and mutable references to the elements of the array.
    ///
    /// Elements are visited in the *logical order* of the array, which
    /// is where the rightmost index is varying the fastest.
    ///
    /// Iterator element type is `(D::Pattern, &mut A)`.
    pub fn indexed_iter_mut(&mut self) -> IndexedIterMut<A, D>
        where S: DataMut,
    {
        IndexedIterMut::new(self.view_mut().into_elements_base())
    }


    /// Return a sliced array.
    ///
    /// See [*Slicing*](#slicing) for full documentation.
    /// See also [`D::SliceArg`].
    ///
    /// [`D::SliceArg`]: trait.Dimension.html#associatedtype.SliceArg
    ///
    /// **Panics** if an index is out of bounds or stride is zero.<br>
    /// (**Panics** if `D` is `IxDyn` and `indexes` does not match the number of array axes.)
    pub fn slice(&self, indexes: &D::SliceArg) -> ArrayView<A, D> {
        let mut arr = self.view();
        arr.islice(indexes);
        arr
    }

    /// Return a sliced read-write view of the array.
    ///
    /// See also [`D::SliceArg`].
    ///
    /// [`D::SliceArg`]: trait.Dimension.html#associatedtype.SliceArg
    ///
    /// **Panics** if an index is out of bounds or stride is zero.<br>
    /// (**Panics** if `D` is `IxDyn` and `indexes` does not match the number of array axes.)
    pub fn slice_mut(&mut self, indexes: &D::SliceArg) -> ArrayViewMut<A, D>
        where S: DataMut
    {
        let mut arr = self.view_mut();
        arr.islice(indexes);
        arr
    }

    /// Slice the array’s view in place.
    ///
    /// See also [`D::SliceArg`].
    ///
    /// [`D::SliceArg`]: trait.Dimension.html#associatedtype.SliceArg
    ///
    /// **Panics** if an index is out of bounds or stride is zero.<br>
    /// (**Panics** if `D` is `IxDyn` and `indexes` does not match the number of array axes.)
    pub fn islice(&mut self, indexes: &D::SliceArg) {
        let offset = D::do_slices(&mut self.dim, &mut self.strides, indexes);
        unsafe {
            self.ptr = self.ptr.offset(offset);
        }
        debug_assert!(self.pointer_is_inbounds());
    }



    /// Return a reference to the element at `index`, or return `None`
    /// if the index is out of bounds.
    ///
    /// Arrays also support indexing syntax: `array[index]`.
    ///
    /// ```
    /// use ndarray::arr2;
    ///
    /// let a = arr2(&[[1., 2.],
    ///                [3., 4.]]);
    ///
    /// assert!(
    ///     a.get((0, 1)) == Some(&2.) &&
    ///     a.get((0, 2)) == None &&
    ///     a[(0, 1)] == 2. &&
    ///     a[[0, 1]] == 2.
    /// );
    /// ```
    pub fn get<I>(&self, index: I) -> Option<&A>
        where I: NdIndex<D>,
    {
        unsafe {
            self.get_ptr(index).map(|ptr| &*ptr)
        }
    }

    pub(crate) fn get_ptr<I>(&self, index: I) -> Option<*const A>
        where I: NdIndex<D>,
    {
        let ptr = self.ptr;
        index.index_checked(&self.dim, &self.strides)
             .map(move |offset| unsafe { ptr.offset(offset) as *const _ })
    }

    /// Return a mutable reference to the element at `index`, or return `None`
    /// if the index is out of bounds.
    pub fn get_mut<I>(&mut self, index: I) -> Option<&mut A>
        where S: DataMut,
              I: NdIndex<D>,
    {
        unsafe {
            self.get_ptr_mut(index).map(|ptr| &mut *ptr)
        }
    }

    pub(crate) fn get_ptr_mut<I>(&mut self, index: I) -> Option<*mut A>
        where S: DataMut,
              I: NdIndex<D>,
    {
        // const and mut are separate to enforce &mutness as well as the
        // extra code in as_mut_ptr
        let ptr = self.as_mut_ptr();
        index.index_checked(&self.dim, &self.strides)
             .map(move |offset| unsafe { ptr.offset(offset) })
    }

    /// Perform *unchecked* array indexing.
    ///
    /// Return a reference to the element at `index`.
    ///
    /// **Note:** only unchecked for non-debug builds of ndarray.
    #[inline]
    pub unsafe fn uget<I>(&self, index: I) -> &A
        where I: NdIndex<D>,
    {
        arraytraits::debug_bounds_check(self, &index);
        let off = index.index_unchecked(&self.strides);
        &*self.ptr.offset(off)
    }

    /// Perform *unchecked* array indexing.
    ///
    /// Return a mutable reference to the element at `index`.
    ///
    /// **Note:** Only unchecked for non-debug builds of ndarray.<br>
    /// **Note:** (For `RcArray`) The array must be uniquely held when mutating it.
    #[inline]
    pub unsafe fn uget_mut<I>(&mut self, index: I) -> &mut A
        where S: DataMut,
              I: NdIndex<D>,
    {
        debug_assert!(self.data.is_unique());
        arraytraits::debug_bounds_check(self, &index);
        let off = index.index_unchecked(&self.strides);
        &mut *self.ptr.offset(off)
    }

    /// Swap elements at indices `index1` and `index2`.
    ///
    /// Indices may be equal.
    ///
    /// ***Panics*** if an index is out of bounds.
    pub fn swap<I>(&mut self, index1: I, index2: I)
        where S: DataMut,
              I: NdIndex<D>,
    {
        let ptr1: *mut _ = &mut self[index1];
        let ptr2: *mut _ = &mut self[index2];
        unsafe {
            std_ptr::swap(ptr1, ptr2);
        }
    }

    /// Swap elements *unchecked* at indices `index1` and `index2`.
    ///
    /// Indices may be equal.
    ///
    /// **Note:** only unchecked for non-debug builds of ndarray.<br>
    /// **Note:** (For `RcArray`) The array must be uniquely held.
    pub unsafe fn uswap<I>(&mut self, index1: I, index2: I)
        where S: DataMut,
              I: NdIndex<D>,
    {
        debug_assert!(self.data.is_unique());
        arraytraits::debug_bounds_check(self, &index1);
        arraytraits::debug_bounds_check(self, &index2);
        let off1 = index1.index_unchecked(&self.strides);
        let off2 = index2.index_unchecked(&self.strides);
        std_ptr::swap(self.ptr.offset(off1), self.ptr.offset(off2));
    }

    // `get` for zero-dimensional arrays
    // panics if dimension is not zero. otherwise an element is always present.
    fn get_0d(&self) -> &A {
        assert!(self.ndim() == 0);
        unsafe {
            &*self.as_ptr()
        }
    }

    /// Along `axis`, select the subview `index` and return a
    /// view with that axis removed.
    ///
    /// See [*Subviews*](#subviews) for full documentation.
    ///
    /// **Panics** if `axis` or `index` is out of bounds.
    ///
    /// ```
    /// use ndarray::{arr2, ArrayView, Axis};
    ///
    /// let a = arr2(&[[1., 2. ],    // ... axis 0, row 0
    ///                [3., 4. ],    // --- axis 0, row 1
    ///                [5., 6. ]]);  // ... axis 0, row 2
    /// //               .   \
    /// //                .   axis 1, column 1
    /// //                 axis 1, column 0
    /// assert!(
    ///     a.subview(Axis(0), 1) == ArrayView::from(&[3., 4.]) &&
    ///     a.subview(Axis(1), 1) == ArrayView::from(&[2., 4., 6.])
    /// );
    /// ```
    pub fn subview(&self, axis: Axis, index: Ix) -> ArrayView<A, D::Smaller>
        where D: RemoveAxis,
    {
        self.view().into_subview(axis, index)
    }

    /// Along `axis`, select the subview `index` and return a read-write view
    /// with the axis removed.
    ///
    /// **Panics** if `axis` or `index` is out of bounds.
    ///
    /// ```
    /// use ndarray::{arr2, aview2, Axis};
    ///
    /// let mut a = arr2(&[[1., 2. ],
    ///                    [3., 4. ]]);
    /// //                   .   \
    /// //                    .   axis 1, column 1
    /// //                     axis 1, column 0
    ///
    /// {
    ///     let mut column1 = a.subview_mut(Axis(1), 1);
    ///     column1 += 10.;
    /// }
    ///
    /// assert!(
    ///     a == aview2(&[[1., 12.],
    ///                   [3., 14.]])
    /// );
    /// ```
    pub fn subview_mut(&mut self, axis: Axis, index: Ix)
        -> ArrayViewMut<A, D::Smaller>
        where S: DataMut,
              D: RemoveAxis,
    {
        self.view_mut().into_subview(axis, index)
    }

    /// Collapse dimension `axis` into length one,
    /// and select the subview of `index` along that axis.
    ///
    /// **Panics** if `index` is past the length of the axis.
    pub fn isubview(&mut self, axis: Axis, index: Ix) {
        dimension::do_sub(&mut self.dim, &mut self.ptr, &self.strides,
                          axis.index(), index)
    }

    /// Along `axis`, select the subview `index` and return `self`
    /// with that axis removed.
    ///
    /// See [`.subview()`](#method.subview) and [*Subviews*](#subviews) for full documentation.
    pub fn into_subview(mut self, axis: Axis, index: Ix) -> ArrayBase<S, D::Smaller>
        where D: RemoveAxis,
    {
        self.isubview(axis, index);
        self.remove_axis(axis)
    }

    /// Along `axis`, select arbitrary subviews corresponding to `indices`
    /// and and copy them into a new array.
    ///
    /// **Panics** if `axis` or an element of `indices` is out of bounds.
    ///
    /// ```
    /// use ndarray::{arr2, Axis};
    ///
    /// let x = arr2(&[[0., 1.],
    ///                [2., 3.],
    ///                [4., 5.],
    ///                [6., 7.],
    ///                [8., 9.]]);
    ///
    /// let r = x.select(Axis(0), &[0, 4, 3]);
    /// assert!(
    ///         r == arr2(&[[0., 1.],
    ///                     [8., 9.],
    ///                     [6., 7.]])
    ///);
    /// ```
    pub fn select(&self, axis: Axis, indices: &[Ix]) -> Array<A, D>
        where A: Copy,
              D: RemoveAxis,
    {
        let mut subs = vec![self.view(); indices.len()];
        for (&i, sub) in zip(indices, &mut subs[..]) {
            sub.isubview(axis, i);
        }
        if subs.is_empty() {
            let mut dim = self.raw_dim();
            dim.set_axis(axis, 0);
            unsafe {
                Array::from_shape_vec_unchecked(dim, vec![])
            }
        } else {
            stack(axis, &subs).unwrap()
        }
    }

    /// Return a producer and iterable that traverses over the *generalized*
    /// rows of the array. For a 2D array these are the regular rows.
    ///
    /// This is equivalent to `.lanes(Axis(n - 1))` where *n* is `self.ndim()`.
    ///
    /// For an array of dimensions *a* × *b* × *c* × ... × *l* × *m*
    /// it has *a* × *b* × *c* × ... × *l* rows each of length *m*.
    ///
    /// For example, in a 2 × 2 × 3 array, each row is 3 elements long
    /// and there are 2 × 2 = 4 rows in total.
    ///
    /// Iterator element is `ArrayView1<A>` (1D array view).
    ///
    /// ```
    /// use ndarray::{arr3, Axis, arr1};
    ///
    /// let a = arr3(&[[[ 0,  1,  2],    // -- row 0, 0
    ///                 [ 3,  4,  5]],   // -- row 0, 1
    ///                [[ 6,  7,  8],    // -- row 1, 0
    ///                 [ 9, 10, 11]]]); // -- row 1, 1
    ///
    /// // `genrows` will yield the four generalized rows of the array.
    /// for row in a.genrows() {
    ///     /* loop body */
    /// }
    /// ```
    pub fn genrows(&self) -> Lanes<A, D::Smaller> {
        let mut n = self.ndim();
        if n == 0 { n += 1; }
        new_lanes(self.view(), Axis(n - 1))
    }

    /// Return a producer and iterable that traverses over the *generalized*
    /// rows of the array and yields mutable array views.
    ///
    /// Iterator element is `ArrayView1<A>` (1D read-write array view).
    pub fn genrows_mut(&mut self) -> LanesMut<A, D::Smaller>
        where S: DataMut
    {
        let mut n = self.ndim();
        if n == 0 { n += 1; }
        new_lanes_mut(self.view_mut(), Axis(n - 1))
    }

    /// Return a producer and iterable that traverses over the *generalized*
    /// columns of the array. For a 2D array these are the regular columns.
    ///
    /// This is equivalent to `.lanes(Axis(0))`.
    ///
    /// For an array of dimensions *a* × *b* × *c* × ... × *l* × *m*
    /// it has *b* × *c* × ... × *l* × *m* columns each of length *a*.
    ///
    /// For example, in a 2 × 2 × 3 array, each column is 2 elements long
    /// and there are 2 × 3 = 6 columns in total.
    ///
    /// Iterator element is `ArrayView1<A>` (1D array view).
    ///
    /// ```
    /// use ndarray::{arr3, Axis, arr1};
    ///
    /// // The generalized columns of a 3D array:
    /// // are directed along the 0th axis: 0 and 6, 1 and 7 and so on...
    /// let a = arr3(&[[[ 0,  1,  2], [ 3,  4,  5]],
    ///                [[ 6,  7,  8], [ 9, 10, 11]]]);
    ///
    /// // Here `gencolumns` will yield the six generalized columns of the array.
    /// for row in a.gencolumns() {
    ///     /* loop body */
    /// }
    /// ```
    pub fn gencolumns(&self) -> Lanes<A, D::Smaller> {
        new_lanes(self.view(), Axis(0))
    }

    /// Return a producer and iterable that traverses over the *generalized*
    /// columns of the array and yields mutable array views.
    ///
    /// Iterator element is `ArrayView1<A>` (1D read-write array view).
    pub fn gencolumns_mut(&mut self) -> LanesMut<A, D::Smaller>
        where S: DataMut
    {
        new_lanes_mut(self.view_mut(), Axis(0))
    }

    /// Return a producer and iterable that traverses over all 1D lanes
    /// pointing in the direction of `axis`.
    ///
    /// When the pointing in the direction of the first axis, they are *columns*,
    /// in the direction of the last axis *rows*; in general they are all
    /// *lanes* and are one dimensional.
    ///
    /// Iterator element is `ArrayView1<A>` (1D array view).
    ///
    /// ```
    /// use ndarray::{arr3, aview1, Axis};
    ///
    /// let a = arr3(&[[[ 0,  1,  2],
    ///                 [ 3,  4,  5]],
    ///                [[ 6,  7,  8],
    ///                 [ 9, 10, 11]]]);
    ///
    /// let inner0 = a.lanes(Axis(0));
    /// let inner1 = a.lanes(Axis(1));
    /// let inner2 = a.lanes(Axis(2));
    ///
    /// // The first lane for axis 0 is [0, 6]
    /// assert_eq!(inner0.into_iter().next().unwrap(), aview1(&[0, 6]));
    /// // The first lane for axis 1 is [0, 3]
    /// assert_eq!(inner1.into_iter().next().unwrap(), aview1(&[0, 3]));
    /// // The first lane for axis 2 is [0, 1, 2]
    /// assert_eq!(inner2.into_iter().next().unwrap(), aview1(&[0, 1, 2]));
    /// ```
    pub fn lanes(&self, axis: Axis) -> Lanes<A, D::Smaller> {
        new_lanes(self.view(), axis)
    }

    /// Return a producer and iterable that traverses over all 1D lanes
    /// pointing in the direction of `axis`.
    ///
    /// Iterator element is `ArrayViewMut1<A>` (1D read-write array view).
    pub fn lanes_mut(&mut self, axis: Axis) -> LanesMut<A, D::Smaller>
        where S: DataMut
    {
        new_lanes_mut(self.view_mut(), axis)
    }


    /// Return an iterator that traverses over the outermost dimension
    /// and yields each subview.
    ///
    /// This is equivalent to `.axis_iter(Axis(0))`.
    ///
    /// Iterator element is `ArrayView<A, D::Smaller>` (read-only array view).
    #[allow(deprecated)]
    pub fn outer_iter(&self) -> AxisIter<A, D::Smaller>
        where D: RemoveAxis,
    {
        self.view().into_outer_iter()
    }

    /// Return an iterator that traverses over the outermost dimension
    /// and yields each subview.
    ///
    /// This is equivalent to `.axis_iter_mut(Axis(0))`.
    ///
    /// Iterator element is `ArrayViewMut<A, D::Smaller>` (read-write array view).
    #[allow(deprecated)]
    pub fn outer_iter_mut(&mut self) -> AxisIterMut<A, D::Smaller>
        where S: DataMut,
              D: RemoveAxis,
    {
        self.view_mut().into_outer_iter()
    }

    /// Return an iterator that traverses over `axis`
    /// and yields each subview along it.
    ///
    /// For example, in a 3 × 4 × 5 array, with `axis` equal to `Axis(2)`,
    /// the iterator element
    /// is a 3 × 4 subview (and there are 5 in total), as shown
    /// in the picture below.
    ///
    /// Iterator element is `ArrayView<A, D::Smaller>` (read-only array view).
    ///
    /// See [*Subviews*](#subviews) for full documentation.
    ///
    /// **Panics** if `axis` is out of bounds.
    ///
    /// <img src="https://bluss.github.io/ndarray/images/axis_iter_3_4_5.svg" height="250px">
    pub fn axis_iter(&self, axis: Axis) -> AxisIter<A, D::Smaller>
        where D: RemoveAxis,
    {
        iterators::new_axis_iter(self.view(), axis.index())
    }


    /// Return an iterator that traverses over `axis`
    /// and yields each mutable subview along it.
    ///
    /// Iterator element is `ArrayViewMut<A, D::Smaller>`
    /// (read-write array view).
    ///
    /// **Panics** if `axis` is out of bounds.
    pub fn axis_iter_mut(&mut self, axis: Axis) -> AxisIterMut<A, D::Smaller>
        where S: DataMut,
              D: RemoveAxis,
    {
        iterators::new_axis_iter_mut(self.view_mut(), axis.index())
    }


    /// Return an iterator that traverses over `axis` by chunks of `size`,
    /// yielding non-overlapping views along that axis.
    ///
    /// Iterator element is `ArrayView<A, D>`
    ///
    /// The last view may have less elements if `size` does not divide
    /// the axis' dimension.
    ///
    /// **Panics** if `axis` is out of bounds.
    ///
    /// ```
    /// use ndarray::Array;
    /// use ndarray::{arr3, Axis};
    ///
    /// let a = Array::from_iter(0..28).into_shape((2, 7, 2)).unwrap();
    /// let mut iter = a.axis_chunks_iter(Axis(1), 2);
    ///
    /// // first iteration yields a 2 × 2 × 2 view
    /// assert_eq!(iter.next().unwrap(),
    ///            arr3(&[[[ 0,  1], [ 2, 3]],
    ///                   [[14, 15], [16, 17]]]));
    ///
    /// // however the last element is a 2 × 1 × 2 view since 7 % 2 == 1
    /// assert_eq!(iter.next_back().unwrap(), arr3(&[[[12, 13]],
    ///                                              [[26, 27]]]));
    /// ```
    pub fn axis_chunks_iter(&self, axis: Axis, size: usize) -> AxisChunksIter<A, D> {
        iterators::new_chunk_iter(self.view(), axis.index(), size)
    }

    /// Return an iterator that traverses over `axis` by chunks of `size`,
    /// yielding non-overlapping read-write views along that axis.
    ///
    /// Iterator element is `ArrayViewMut<A, D>`
    ///
    /// **Panics** if `axis` is out of bounds.
    pub fn axis_chunks_iter_mut(&mut self, axis: Axis, size: usize)
        -> AxisChunksIterMut<A, D>
        where S: DataMut
    {
        iterators::new_chunk_iter_mut(self.view_mut(), axis.index(), size)
    }

    /// Return an exact chunks producer (and iterable).
    ///
    /// It produces the whole chunks of a given n-dimensional chunk size,
    /// skipping the remainder along each dimension that doesn't fit evenly.
    ///
    /// The produced element is a `ArrayView<A, D>` with exactly the dimension
    /// `chunk_size`.
    ///
    /// **Panics** if any dimension of `chunk_size` is zero<br>
    /// (**Panics** if `D` is `IxDyn` and `chunk_size` does not match the
    /// number of array axes.)
    pub fn exact_chunks<E>(&self, chunk_size: E) -> ExactChunks<A, D> 
        where E: IntoDimension<Dim=D>,
    {
        exact_chunks_of(self.view(), chunk_size)
    }

    #[doc(hidden)]
    #[deprecated(note="Renamed to exact_chunks")]
    pub fn whole_chunks<E>(&self, chunk_size: E) -> ExactChunks<A, D> 
        where E: IntoDimension<Dim=D>,
    {
        self.exact_chunks(chunk_size)
    }

    /// Return an exact chunks producer (and iterable).
    ///
    /// It produces the whole chunks of a given n-dimensional chunk size,
    /// skipping the remainder along each dimension that doesn't fit evenly.
    ///
    /// The produced element is a `ArrayViewMut<A, D>` with exactly
    /// the dimension `chunk_size`.
    ///
    /// **Panics** if any dimension of `chunk_size` is zero<br>
    /// (**Panics** if `D` is `IxDyn` and `chunk_size` does not match the
    /// number of array axes.)
    ///
    /// ```rust
    /// use ndarray::Array;
    /// use ndarray::arr2;
    /// let mut a = Array::zeros((6, 7));
    ///
    /// // Fill each 2 × 2 chunk with the index of where it appeared in iteration
    /// for (i, mut chunk) in a.exact_chunks_mut((2, 2)).into_iter().enumerate() {
    ///     chunk.fill(i);
    /// }
    ///
    /// // The resulting array is:
    /// assert_eq!(
    ///   a,
    ///   arr2(&[[0, 0, 1, 1, 2, 2, 0],
    ///          [0, 0, 1, 1, 2, 2, 0],
    ///          [3, 3, 4, 4, 5, 5, 0],
    ///          [3, 3, 4, 4, 5, 5, 0],
    ///          [6, 6, 7, 7, 8, 8, 0],
    ///          [6, 6, 7, 7, 8, 8, 0]]));
    /// ```
    pub fn exact_chunks_mut<E>(&mut self, chunk_size: E) -> ExactChunksMut<A, D> 
        where E: IntoDimension<Dim=D>,
              S: DataMut
    {
        exact_chunks_mut_of(self.view_mut(), chunk_size)
    }

    #[doc(hidden)]
    #[deprecated(note="Renamed to exact_chunks_mut")]
    pub fn whole_chunks_mut<E>(&mut self, chunk_size: E) -> ExactChunksMut<A, D> 
        where E: IntoDimension<Dim=D>,
              S: DataMut
    {
        self.exact_chunks_mut(chunk_size)
    }

    /// Return a window producer and iterable.
    ///
    /// The windows are all distinct overlapping views of size `window_size`
    /// that fit into the array's shape.
    /// 
    /// Will yield over no elements if window size is larger
    /// than the actual array size of any dimension.
    ///
    /// The produced element is an `ArrayView<A, D>` with exactly the dimension
    /// `window_size`.
    /// 
    /// **Panics** if any dimension of `window_size` is zero.<br>
    /// (**Panics** if `D` is `IxDyn` and `window_size` does not match the
    /// number of array axes.)
    pub fn windows<E>(&self, window_size: E) -> Windows<A, D>
        where E: IntoDimension<Dim=D>
    {
        windows(self.view(), window_size)
    }

    // Return (length, stride) for diagonal
    fn diag_params(&self) -> (Ix, Ixs) {
        /* empty shape has len 1 */
        let len = self.dim.slice().iter().cloned().min().unwrap_or(1);
        let stride = self.strides()
                         .iter()
                         .fold(0, |sum, s| sum + s);
        (len, stride)
    }

    /// Return an view of the diagonal elements of the array.
    ///
    /// The diagonal is simply the sequence indexed by *(0, 0, .., 0)*,
    /// *(1, 1, ..., 1)* etc as long as all axes have elements.
    pub fn diag(&self) -> ArrayView1<A> {
        self.view().into_diag()
    }

    /// Return a read-write view over the diagonal elements of the array.
    pub fn diag_mut(&mut self) -> ArrayViewMut1<A>
        where S: DataMut,
    {
        self.view_mut().into_diag()
    }

    /// Return the diagonal as a one-dimensional array.
    pub fn into_diag(self) -> ArrayBase<S, Ix1> {
        let (len, stride) = self.diag_params();
        ArrayBase {
            data: self.data,
            ptr: self.ptr,
            dim: Ix1(len),
            strides: Ix1(stride as Ix),
        }
    }

    /// Make the array unshared.
    ///
    /// This method is mostly only useful with unsafe code.
    fn ensure_unique(&mut self)
        where S: DataMut
    {
        debug_assert!(self.pointer_is_inbounds());
        S::ensure_unique(self);
        debug_assert!(self.pointer_is_inbounds());
    }

    /// Return `true` if the array data is laid out in contiguous “C order” in
    /// memory (where the last index is the most rapidly varying).
    ///
    /// Return `false` otherwise, i.e the array is possibly not
    /// contiguous in memory, it has custom strides, etc.
    pub fn is_standard_layout(&self) -> bool {
        fn is_standard_layout<D: Dimension>(dim: &D, strides: &D) -> bool {
            let defaults = dim.default_strides();
            if strides.equal(&defaults) {
                return true;
            }
            if dim.ndim() == 1 { return false; }
            // check all dimensions -- a dimension of length 1 can have unequal strides
            for (&dim, &s, &ds) in izip!(dim.slice(), strides.slice(), defaults.slice()) {
                if dim != 1 && s != ds {
                    return false;
                }
            }
            true
        }
        is_standard_layout(&self.dim, &self.strides)
    }

    fn is_contiguous(&self) -> bool {
        D::is_contiguous(&self.dim, &self.strides)
    }

    /// Return a pointer to the first element in the array.
    ///
    /// Raw access to array elements needs to follow the strided indexing
    /// scheme: an element at multi-index *I* in an array with strides *S* is
    /// located at offset
    ///
    /// *Σ<sub>0 ≤ k < d</sub> I<sub>k</sub> × S<sub>k</sub>*
    ///
    /// where *d* is `self.ndim()`.
    #[inline(always)]
    pub fn as_ptr(&self) -> *const A {
        self.ptr
    }

    /// Return a mutable pointer to the first element in the array.
    #[inline(always)]
    pub fn as_mut_ptr(&mut self) -> *mut A
        where S: DataMut
    {
        self.ensure_unique(); // for RcArray
        self.ptr
    }

    /// Return the array’s data as a slice, if it is contiguous and in standard order.
    /// Return `None` otherwise.
    ///
    /// If this function returns `Some(_)`, then the element order in the slice
    /// corresponds to the logical order of the array’s elements.
    pub fn as_slice(&self) -> Option<&[A]> {
        if self.is_standard_layout() {
            unsafe {
                Some(slice::from_raw_parts(self.ptr, self.len()))
            }
        } else {
            None
        }
    }

    /// Return the array’s data as a slice, if it is contiguous and in standard order.
    /// Return `None` otherwise.
    pub fn as_slice_mut(&mut self) -> Option<&mut [A]>
        where S: DataMut
    {
        if self.is_standard_layout() {
            self.ensure_unique();
            unsafe {
                Some(slice::from_raw_parts_mut(self.ptr, self.len()))
            }
        } else {
            None
        }
    }

    /// Return the array’s data as a slice if it is contiguous,
    /// return `None` otherwise.
    ///
    /// If this function returns `Some(_)`, then the elements in the slice
    /// have whatever order the elements have in memory.
    ///
    /// Implementation notes: Does not yet support negatively strided arrays.
    pub fn as_slice_memory_order(&self) -> Option<&[A]> {
        if self.is_contiguous() {
            unsafe {
                Some(slice::from_raw_parts(self.ptr, self.len()))
            }
        } else {
            None
        }
    }

    /// Return the array’s data as a slice if it is contiguous,
    /// return `None` otherwise.
    pub fn as_slice_memory_order_mut(&mut self) -> Option<&mut [A]>
        where S: DataMut
    {
        if self.is_contiguous() {
            self.ensure_unique();
            unsafe {
                Some(slice::from_raw_parts_mut(self.ptr, self.len()))
            }
        } else {
            None
        }
    }

    /// Transform the array into `shape`; any shape with the same number of
    /// elements is accepted, but the source array or view must be
    /// contiguous, otherwise we cannot rearrange the dimension.
    ///
    /// **Errors** if the shapes don't have the same number of elements.<br>
    /// **Errors** if the input array is not c- or f-contiguous.
    ///
    /// ```
    /// use ndarray::{aview1, aview2};
    ///
    /// assert!(
    ///     aview1(&[1., 2., 3., 4.]).into_shape((2, 2)).unwrap()
    ///     == aview2(&[[1., 2.],
    ///                 [3., 4.]])
    /// );
    /// ```
    pub fn into_shape<E>(self, shape: E) -> Result<ArrayBase<S, E::Dim>, ShapeError>
        where E: IntoDimension,
    {
        let shape = shape.into_dimension();
        if shape.size_checked() != Some(self.dim.size()) {
            return Err(error::incompatible_shapes(&self.dim, &shape));
        }
        // Check if contiguous, if not => copy all, else just adapt strides
        if self.is_standard_layout() {
            Ok(ArrayBase {
                data: self.data,
                ptr: self.ptr,
                strides: shape.default_strides(),
                dim: shape,
            })
        } else if self.ndim() > 1 && self.view().reversed_axes().is_standard_layout() {
            Ok(ArrayBase {
                data: self.data,
                ptr: self.ptr,
                strides: shape.fortran_strides(),
                dim: shape,
            })
        } else {
            Err(error::from_kind(error::ErrorKind::IncompatibleLayout))
        }
    }

    /// *Note: Reshape is for `RcArray` only. Use `.into_shape()` for
    /// other arrays and array views.*
    ///
    /// Transform the array into `shape`; any shape with the same number of
    /// elements is accepted.
    ///
    /// May clone all elements if needed to arrange elements in standard
    /// layout (and break sharing).
    ///
    /// **Panics** if shapes are incompatible.
    ///
    /// ```
    /// use ndarray::{rcarr1, rcarr2};
    ///
    /// assert!(
    ///     rcarr1(&[1., 2., 3., 4.]).reshape((2, 2))
    ///     == rcarr2(&[[1., 2.],
    ///                 [3., 4.]])
    /// );
    /// ```
    pub fn reshape<E>(&self, shape: E) -> ArrayBase<S, E::Dim>
        where S: DataShared + DataOwned,
              A: Clone,
              E: IntoDimension,
    {
        let shape = shape.into_dimension();
        if shape.size_checked() != Some(self.dim.size()) {
            panic!("ndarray: incompatible shapes in reshape, attempted from: {:?}, to: {:?}",
                   self.dim.slice(),
                   shape.slice())
        }
        // Check if contiguous, if not => copy all, else just adapt strides
        if self.is_standard_layout() {
            let cl = self.clone();
            ArrayBase {
                data: cl.data,
                ptr: cl.ptr,
                strides: shape.default_strides(),
                dim: shape,
            }
        } else {
            let v = self.iter().map(|x| x.clone()).collect::<Vec<A>>();
            unsafe {
                ArrayBase::from_shape_vec_unchecked(shape, v)
            }
        }
    }

    /// Convert any array or array view to a dynamic dimensional array or
    /// array view (respectively).
    ///
    /// ```
    /// use ndarray::{arr2, ArrayD};
    ///
    /// let array: ArrayD<i32> = arr2(&[[1, 2],
    ///                                 [3, 4]]).into_dyn();
    /// ```
    pub fn into_dyn(self) -> ArrayBase<S, IxDyn> {
        ArrayBase {
            data: self.data,
            ptr: self.ptr,
            dim: self.dim.into_dyn(),
            strides: self.strides.into_dyn(),
        }
    }

    /// Convert an array or array view to another with the same type, but
    /// different dimensionality type. Errors if the dimensions don't agree.
    ///
    /// ```
    /// use ndarray::{ArrayD, Ix2, IxDyn};
    ///
    /// // Create a dynamic dimensionality array and convert it to an Array2
    /// // (Ix2 dimension type).
    ///
    /// let array = ArrayD::<f64>::zeros(IxDyn(&[10, 10]));
    ///
    /// assert!(array.into_dimensionality::<Ix2>().is_ok());
    /// ```
    pub fn into_dimensionality<D2>(self) -> Result<ArrayBase<S, D2>, ShapeError>
        where D2: Dimension
    {
        if let Some(dim) = D2::from_dimension(&self.dim) {
            if let Some(strides) = D2::from_dimension(&self.strides) {
                return Ok(ArrayBase {
                    data: self.data,
                    ptr: self.ptr,
                    dim: dim,
                    strides: strides,
                });
            }
        }
        Err(ShapeError::from_kind(ErrorKind::IncompatibleShape))
    }

    /// Act like a larger size and/or shape array by *broadcasting*
    /// into a larger shape, if possible.
    ///
    /// Return `None` if shapes can not be broadcast together.
    ///
    /// ***Background***
    ///
    ///  * Two axes are compatible if they are equal, or one of them is 1.
    ///  * In this instance, only the axes of the smaller side (self) can be 1.
    ///
    /// Compare axes beginning with the *last* axis of each shape.
    ///
    /// For example (1, 2, 4) can be broadcast into (7, 6, 2, 4)
    /// because its axes are either equal or 1 (or missing);
    /// while (2, 2) can *not* be broadcast into (2, 4).
    ///
    /// The implementation creates a view with strides set to zero for the
    /// axes that are to be repeated.
    ///
    /// The broadcasting documentation for Numpy has more information.
    ///
    /// ```
    /// use ndarray::{aview1, aview2};
    ///
    /// assert!(
    ///     aview1(&[1., 0.]).broadcast((10, 2)).unwrap()
    ///     == aview2(&[[1., 0.]; 10])
    /// );
    /// ```
    pub fn broadcast<E>(&self, dim: E) -> Option<ArrayView<A, E::Dim>>
        where E: IntoDimension
    {
        /// Return new stride when trying to grow `from` into shape `to`
        ///
        /// Broadcasting works by returning a "fake stride" where elements
        /// to repeat are in axes with 0 stride, so that several indexes point
        /// to the same element.
        ///
        /// **Note:** Cannot be used for mutable iterators, since repeating
        /// elements would create aliasing pointers.
        fn upcast<D: Dimension, E: Dimension>(to: &D, from: &E, stride: &E) -> Option<D> {
            let mut new_stride = to.clone();
            // begin at the back (the least significant dimension)
            // size of the axis has to either agree or `from` has to be 1
            if to.ndim() < from.ndim() {
                return None;
            }

            {
                let mut new_stride_iter = new_stride.slice_mut().iter_mut().rev();
                for ((er, es), dr) in from.slice().iter().rev()
                                        .zip(stride.slice().iter().rev())
                                        .zip(new_stride_iter.by_ref())
                {
                    /* update strides */
                    if *dr == *er {
                        /* keep stride */
                        *dr = *es;
                    } else if *er == 1 {
                        /* dead dimension, zero stride */
                        *dr = 0
                    } else {
                        return None;
                    }
                }

                /* set remaining strides to zero */
                for dr in new_stride_iter {
                    *dr = 0;
                }
            }
            Some(new_stride)
        }
        let dim = dim.into_dimension();

        // Note: zero strides are safe precisely because we return an read-only view
        let broadcast_strides = match upcast(&dim, &self.dim, &self.strides) {
            Some(st) => st,
            None => return None,
        };
        unsafe { Some(ArrayView::new_(self.ptr, dim, broadcast_strides)) }
    }

    /// Swap axes `ax` and `bx`.
    ///
    /// This does not move any data, it just adjusts the array’s dimensions
    /// and strides.
    ///
    /// **Panics** if the axes are out of bounds.
    ///
    /// ```
    /// use ndarray::arr2;
    ///
    /// let mut a = arr2(&[[1., 2., 3.]]);
    /// a.swap_axes(0, 1);
    /// assert!(
    ///     a == arr2(&[[1.], [2.], [3.]])
    /// );
    /// ```
    pub fn swap_axes(&mut self, ax: usize, bx: usize) {
        self.dim.slice_mut().swap(ax, bx);
        self.strides.slice_mut().swap(ax, bx);
    }

    /// Transpose the array by reversing axes.
    ///
    /// Transposition reverses the order of the axes (dimensions and strides)
    /// while retaining the same data.
    pub fn reversed_axes(mut self) -> ArrayBase<S, D> {
        self.dim.slice_mut().reverse();
        self.strides.slice_mut().reverse();
        self
    }

    /// Return a transposed view of the array.
    ///
    /// This is a shorthand for `self.view().reversed_axes()`.
    ///
    /// See also the more general methods `.reversed_axes()` and `.swap_axes()`.
    pub fn t(&self) -> ArrayView<A, D> {
        self.view().reversed_axes()
    }

    /// Return an iterator over the length and stride of each axis.
    pub fn axes(&self) -> Axes<D> {
        axes_of(&self.dim, &self.strides)
    }

    /*
    /// Return the axis with the least stride (by absolute value)
    pub fn min_stride_axis(&self) -> Axis {
        self.dim.min_stride_axis(&self.strides)
    }
    */

    /// Return the axis with the greatest stride (by absolute value),
    /// preferring axes with len > 1.
    pub fn max_stride_axis(&self) -> Axis {
        self.dim.max_stride_axis(&self.strides)
    }

    /// Reverse the stride of `axis`.
    ///
    /// ***Panics*** if the axis is out of bounds.
    pub fn invert_axis(&mut self, axis: Axis) {
        unsafe {
            let s = self.strides.axis(axis) as Ixs;
            let m = self.dim.axis(axis);
            if m != 0 {
                self.ptr = self.ptr.offset(stride_offset(m - 1, s as Ix));
            }
            self.strides.set_axis(axis, (-s) as Ix);
        }
    }

    /// If possible, merge in the axis `take` to `into`.
    ///
    /// ```
    /// use ndarray::Array3;
    /// use ndarray::Axis;
    ///
    /// let mut a = Array3::<f64>::zeros((2, 3, 4));
    /// a.merge_axes(Axis(1), Axis(2));
    /// assert_eq!(a.shape(), &[2, 1, 12]);
    /// ```
    ///
    /// ***Panics*** if an axis is out of bounds.
    pub fn merge_axes(&mut self, take: Axis, into: Axis) -> bool {
        merge_axes(&mut self.dim, &mut self.strides, take, into)
    }

    /// Insert new array axis at `axis` and return the result.
    ///
    /// ```
    /// use ndarray::{Array3, Axis, arr1, arr2};
    ///
    /// // Convert a 1-D array into a row vector (2-D).
    /// let a = arr1(&[1, 2, 3]);
    /// let row = a.insert_axis(Axis(0));
    /// assert_eq!(row, arr2(&[[1, 2, 3]]));
    ///
    /// // Convert a 1-D array into a column vector (2-D).
    /// let b = arr1(&[1, 2, 3]);
    /// let col = b.insert_axis(Axis(1));
    /// assert_eq!(col, arr2(&[[1], [2], [3]]));
    ///
    /// // The new axis always has length 1.
    /// let b = Array3::<f64>::zeros((3, 4, 5));
    /// assert_eq!(b.insert_axis(Axis(2)).shape(), &[3, 4, 1, 5]);
    /// ```
    ///
    /// ***Panics*** if the axis is out of bounds.
    pub fn insert_axis(self, axis: Axis) -> ArrayBase<S, D::Larger>
    {
        assert!(axis.index() <= self.ndim());
        let ArrayBase { ptr, data, dim, strides } = self;
        ArrayBase {
            ptr,
            data,
            dim: dim.insert_axis(axis),
            strides: strides.insert_axis(axis),
        }
    }

    /// Remove array axis `axis` and return the result.
    pub fn remove_axis(self, axis: Axis) -> ArrayBase<S, D::Smaller>
        where D: RemoveAxis,
    {
        assert!(self.ndim() != 0);
        let d = self.dim.remove_axis(axis);
        let s = self.strides.remove_axis(axis);
        ArrayBase {
            ptr: self.ptr,
            data: self.data,
            dim: d,
            strides: s,
        }
    }

    fn pointer_is_inbounds(&self) -> bool {
        let slc = self.data._data_slice();
        if slc.is_empty() {
            // special case for data-less views
            return true;
        }
        let ptr = slc.as_ptr() as *mut _;
        let end =  unsafe {
            ptr.offset(slc.len() as isize)
        };
        self.ptr >= ptr && self.ptr <= end
    }

    /// Perform an elementwise assigment to `self` from `rhs`.
    ///
    /// If their shapes disagree, `rhs` is broadcast to the shape of `self`.
    ///
    /// **Panics** if broadcasting isn’t possible.
    pub fn assign<E: Dimension, S2>(&mut self, rhs: &ArrayBase<S2, E>)
        where S: DataMut,
              A: Clone,
              S2: Data<Elem=A>,
    {
        self.zip_mut_with(rhs, |x, y| *x = y.clone());
    }

    /// Perform an elementwise assigment to `self` from element `x`.
    pub fn fill(&mut self, x: A)
        where S: DataMut, A: Clone,
    {
        self.unordered_foreach_mut(move |elt| *elt = x.clone());
    }

    fn zip_mut_with_same_shape<B, S2, E, F>(&mut self, rhs: &ArrayBase<S2, E>, mut f: F)
        where S: DataMut,
              S2: Data<Elem=B>,
              E: Dimension,
              F: FnMut(&mut A, &B)
    {
        debug_assert_eq!(self.shape(), rhs.shape());
        if let Some(self_s) = self.as_slice_mut() {
            if let Some(rhs_s) = rhs.as_slice() {
                let len = cmp::min(self_s.len(), rhs_s.len());
                let s = &mut self_s[..len];
                let r = &rhs_s[..len];
                for i in 0..len {
                    f(&mut s[i], &r[i]);
                }
                return;
            }
        }
        // otherwise, fall back to the outer iter
        self.zip_mut_with_by_rows(rhs, f);
    }

    // zip two arrays where they have different layout or strides
    #[inline(always)]
    fn zip_mut_with_by_rows<B, S2, E, F>(&mut self, rhs: &ArrayBase<S2, E>, mut f: F)
        where S: DataMut,
              S2: Data<Elem=B>,
              E: Dimension,
              F: FnMut(&mut A, &B)
    {
        debug_assert_eq!(self.shape(), rhs.shape());
        debug_assert_ne!(self.ndim(), 0);

        // break the arrays up into their inner rows
        let n = self.ndim();
        let dim = self.raw_dim();
        Zip::from(new_lanes_mut(self.view_mut(), Axis(n - 1)))
            .and(new_lanes(rhs.broadcast_assume(dim), Axis(n - 1)))
            .apply(move |s_row, r_row| {
                Zip::from(s_row).and(r_row).apply(|a, b| f(a, b))
            });
    }


    fn zip_mut_with_elem<B, F>(&mut self, rhs_elem: &B, mut f: F)
        where S: DataMut,
              F: FnMut(&mut A, &B)
    {
        self.unordered_foreach_mut(move |elt| f(elt, rhs_elem));
    }

    /// Traverse two arrays in unspecified order, in lock step,
    /// calling the closure `f` on each element pair.
    ///
    /// If their shapes disagree, `rhs` is broadcast to the shape of `self`.
    ///
    /// **Panics** if broadcasting isn’t possible.
    #[inline]
    pub fn zip_mut_with<B, S2, E, F>(&mut self, rhs: &ArrayBase<S2, E>, f: F)
        where S: DataMut,
              S2: Data<Elem=B>,
              E: Dimension,
              F: FnMut(&mut A, &B)
    {
        if rhs.dim.ndim() == 0 {
            // Skip broadcast from 0-dim array
            self.zip_mut_with_elem(rhs.get_0d(), f);
        } else if self.dim.ndim() == rhs.dim.ndim() && self.shape() == rhs.shape() {
            self.zip_mut_with_same_shape(rhs, f);
        } else {
            let rhs_broadcast = rhs.broadcast_unwrap(self.raw_dim());
            self.zip_mut_with_by_rows(&rhs_broadcast, f);
        }
    }

    /// Traverse the array elements and apply a fold,
    /// returning the resulting value.
    ///
    /// Elements are visited in arbitrary order.
    pub fn fold<'a, F, B>(&'a self, init: B, f: F) -> B
        where F: FnMut(B, &'a A) -> B, A: 'a
    {
        if let Some(slc) = self.as_slice_memory_order() {
            slc.iter().fold(init, f)
        } else {
            let mut v = self.view();
            // put the narrowest axis at the last position
            if v.ndim() > 1 {
                let last = v.ndim() - 1;
                let narrow_axis = v.axes()
                                   .filter(|ax| ax.len() > 1)
                                   .min_by_key(|ax| ax.stride().abs())
                                   .map_or(last, |ax| ax.axis().index());
                v.swap_axes(last, narrow_axis);
            }
            v.into_elements_base().fold(init, f)
        }
    }

    /// Call `f` by reference on each element and create a new array
    /// with the new values.
    ///
    /// Elements are visited in arbitrary order.
    ///
    /// Return an array with the same shape as `self`.
    ///
    /// ```
    /// use ndarray::arr2;
    ///
    /// let a = arr2(&[[ 0., 1.],
    ///                [-1., 2.]]);
    /// assert!(
    ///     a.map(|x| *x >= 1.0)
    ///     == arr2(&[[false, true],
    ///               [false, true]])
    /// );
    /// ```
    pub fn map<'a, B, F>(&'a self, f: F) -> Array<B, D>
        where F: FnMut(&'a A) -> B,
              A: 'a,
    {
        if let Some(slc) = self.as_slice_memory_order() {
            let v = ::iterators::to_vec_mapped(slc.iter(), f);
            unsafe {
                ArrayBase::from_shape_vec_unchecked(
                    self.dim.clone().strides(self.strides.clone()), v)
            }
        } else {
            let v = ::iterators::to_vec_mapped(self.iter(), f);
            unsafe {
                ArrayBase::from_shape_vec_unchecked(self.dim.clone(), v)
            }
        }
    }

    /// Call `f` by **v**alue on each element and create a new array
    /// with the new values.
    ///
    /// Elements are visited in arbitrary order.
    ///
    /// Return an array with the same shape as `self`.
    ///
    /// ```
    /// use ndarray::arr2;
    ///
    /// let a = arr2(&[[ 0., 1.],
    ///                [-1., 2.]]);
    /// assert!(
    ///     a.mapv(f32::abs) == arr2(&[[0., 1.],
    ///                                [1., 2.]])
    /// );
    /// ```
    pub fn mapv<B, F>(&self, f: F) -> Array<B, D>
        where F: Fn(A) -> B,
              A: Clone,
    {
        self.map(move |x| f(x.clone()))
    }

    /// Call `f` by **v**alue on each element, update the array with the new values
    /// and return it.
    ///
    /// Elements are visited in arbitrary order.
    pub fn mapv_into<F>(mut self, f: F) -> Self
        where S: DataMut,
              F: Fn(A) -> A,
              A: Clone,
    {
        self.mapv_inplace(f);
        self
    }

    /// Modify the array in place by calling `f` by mutable reference on each element.
    ///
    /// Elements are visited in arbitrary order.
    pub fn map_inplace<F>(&mut self, f: F)
        where S: DataMut,
              F: Fn(&mut A),
    {
        self.unordered_foreach_mut(f);
    }

    /// Modify the array in place by calling `f` by **v**alue on each element.
    /// The array is updated with the new values.
    ///
    /// Elements are visited in arbitrary order.
    ///
    /// ```
    /// use ndarray::arr2;
    ///
    /// let mut a = arr2(&[[ 0., 1.],
    ///                    [-1., 2.]]);
    /// a.mapv_inplace(f32::exp);
    /// assert!(
    ///     a.all_close(&arr2(&[[1.00000, 2.71828],
    ///                         [0.36788, 7.38906]]), 1e-5)
    /// );
    /// ```
    pub fn mapv_inplace<F>(&mut self, f: F)
        where S: DataMut,
              F: Fn(A) -> A,
              A: Clone,
    {
        self.unordered_foreach_mut(move |x| *x = f(x.clone()));
    }

    /// Visit each element in the array by calling `f` by reference
    /// on each element.
    ///
    /// Elements are visited in arbitrary order.
    pub fn visit<'a, F>(&'a self, mut f: F)
        where F: FnMut(&'a A),
              A: 'a,
    {
        self.fold((), move |(), elt| f(elt))
    }

    /// Fold along an axis.
    ///
    /// Combine the elements of each subview with the previous using the `fold`
    /// function and initial value `init`.
    ///
    /// Return the result as an `Array`.
    pub fn fold_axis<B, F>(&self, axis: Axis, init: B, mut fold: F)
        -> Array<B, D::Smaller>
        where D: RemoveAxis,
              F: FnMut(&B, &A) -> B,
              B: Clone,
    {
        let mut res = Array::from_elem(self.raw_dim().remove_axis(axis), init);
        for subview in self.axis_iter(axis) {
            res.zip_mut_with(&subview, |x, y| *x = fold(x, y));
        }
        res
    }

    /// Reduce the values along an axis into just one value, producing a new
    /// array with one less dimension.
    ///
    /// Elements are visited in arbitrary order.
    ///
    /// Return the result as an `Array`.
    ///
    /// **Panics** if `axis` is out of bounds.
    pub fn map_axis<'a, B, F>(&'a self, axis: Axis, mut mapping: F)
        -> Array<B, D::Smaller>
        where D: RemoveAxis,
              F: FnMut(ArrayView1<'a, A>) -> B,
              A: 'a,
    {
        let view_len = self.len_of(axis);
        let view_stride = self.strides.axis(axis);
        // use the 0th subview as a map to each 1d array view extended from
        // the 0th element.
        self.subview(axis, 0).map(|first_elt| {
            unsafe {
                mapping(ArrayView::new_(first_elt, Ix1(view_len), Ix1(view_stride)))
            }
        })
    }
}
