// Copyright 2014-2016 bluss and ndarray developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Constructor methods for ndarray
//!
//!

use libnum::{Zero, One, Float};

use imp_prelude::*;
use StrideShape;
use dimension;
use linspace;
use error::{self, ShapeError, ErrorKind};
use indices;
use indexes;
use iterators::{to_vec, to_vec_mapped};

/// # Constructor Methods for Owned Arrays
///
/// Note that the constructor methods apply to `Array` and `RcArray`,
/// the two array types that have owned storage.
///
/// ## Constructor methods for one-dimensional arrays.
impl<S, A> ArrayBase<S, Ix1>
    where S: DataOwned<Elem=A>,
{
    /// Create a one-dimensional array from a vector (no copying needed).
    ///
    /// ```rust
    /// use ndarray::Array;
    ///
    /// let array = Array::from_vec(vec![1., 2., 3., 4.]);
    /// ```
    pub fn from_vec(v: Vec<A>) -> Self {
        unsafe { Self::from_shape_vec_unchecked(v.len() as Ix, v) }
    }

    /// Create a one-dimensional array from an iterable.
    ///
    /// ```rust
    /// use ndarray::{Array, arr1};
    ///
    /// let array = Array::from_iter((0..5).map(|x| x * x));
    /// assert!(array == arr1(&[0, 1, 4, 9, 16]))
    /// ```
    pub fn from_iter<I>(iterable: I) -> Self
        where I: IntoIterator<Item=A>
    {
        Self::from_vec(iterable.into_iter().collect())
    }

    /// Create a one-dimensional array from the inclusive interval
    /// `[start, end]` with `n` elements. `A` must be a floating point type.
    ///
    /// ```rust
    /// use ndarray::{Array, arr1};
    ///
    /// let array = Array::linspace(0., 1., 5);
    /// assert!(array == arr1(&[0.0, 0.25, 0.5, 0.75, 1.0]))
    /// ```
    pub fn linspace(start: A, end: A, n: usize) -> Self
        where A: Float,
    {
        Self::from_vec(to_vec(linspace::linspace(start, end, n)))
    }

    /// Create a one-dimensional array from the half-open interval
    /// `[start, end)` with elements spaced by `step`. `A` must be a floating
    /// point type.
    ///
    /// ```rust
    /// use ndarray::{Array, arr1};
    ///
    /// let array = Array::range(0., 5., 1.);
    /// assert!(array == arr1(&[0., 1., 2., 3., 4.]))
    /// ```
    pub fn range(start: A, end: A, step: A) -> Self
        where A: Float,
    {
        Self::from_vec(to_vec(linspace::range(start, end, step)))
    }
}

/// ## Constructor methods for two-dimensional arrays.
impl<S, A> ArrayBase<S, Ix2>
    where S: DataOwned<Elem=A>,
{
    /// Create an identity matrix of size `n` (square 2D array).
    ///
    /// **Panics** if `n * n` would overflow usize.
    pub fn eye(n: Ix) -> Self
        where S: DataMut,
              A: Clone + Zero + One,
    {
        let mut eye = Self::zeros((n, n));
        for a_ii in eye.diag_mut() {
            *a_ii = A::one();
        }
        eye
    }
}

#[cfg(not(debug_assertions))]
macro_rules! size_checked_unwrap {
    ($dim:expr) => {
        match $dim.size_checked() {
            Some(sz) => sz,
            None => panic!("ndarray: Shape too large, number of elements overflows usize"),
        }
    }
}

#[cfg(debug_assertions)]
macro_rules! size_checked_unwrap {
    ($dim:expr) => {
        match $dim.size_checked() {
            Some(sz) => sz,
            None => panic!("ndarray: Shape too large, number of elements overflows usize in shape {:?}",
                           $dim),
        }
    }
}

/// ## Constructor methods for n-dimensional arrays.
///
/// The `shape` argument can be an integer or a tuple of integers to specify
/// a static size. For example `10` makes a length 10 one-dimensional array
/// (dimension type `Ix1`) and `(5, 6)` a 5 × 6 array (dimension type `Ix2`).
///
/// With the trait `ShapeBuilder` in scope, there is the method `.f()` to select
/// column major (“f” order) memory layout instead of the default row major.
/// For example `Array::zeros((5, 6).f())` makes a column major 5 × 6 array.
///
/// Use [`IxDyn`](type.IxDyn.html) for the shape to create an array with dynamic
/// number of axes.
///
/// Finally, the few constructors that take a completely general
/// `Into<StrideShape>` argument *optionally* support custom strides, for
/// example a shape given like `(10, 2, 2).strides((1, 10, 20))` is valid.
impl<S, A, D> ArrayBase<S, D>
    where S: DataOwned<Elem=A>,
          D: Dimension,
{
    /// Create an array with copies of `elem`, shape `shape`.
    ///
    /// **Panics** if the number of elements in `shape` would overflow usize.
    ///
    /// ```
    /// use ndarray::{Array, arr3, ShapeBuilder};
    ///
    /// let a = Array::from_elem((2, 2, 2), 1.);
    ///
    /// assert!(
    ///     a == arr3(&[[[1., 1.],
    ///                  [1., 1.]],
    ///                 [[1., 1.],
    ///                  [1., 1.]]])
    /// );
    /// assert!(a.strides() == &[4, 2, 1]);
    ///
    /// let b = Array::from_elem((2, 2, 2).f(), 1.);
    /// assert!(b.strides() == &[1, 2, 4]);
    /// ```
    pub fn from_elem<Sh>(shape: Sh, elem: A) -> Self
        where A: Clone,
              Sh: ShapeBuilder<Dim=D>,
    {
        // Note: We don't need to check the case of a size between
        // isize::MAX -> usize::MAX; in this case, the vec constructor itself
        // panics.
        let shape = shape.into_shape();
        let size = size_checked_unwrap!(shape.dim);
        let v = vec![elem; size];
        unsafe { Self::from_shape_vec_unchecked(shape, v) }
    }

    /// Create an array with zeros, shape `shape`.
    ///
    /// **Panics** if the number of elements in `shape` would overflow usize.
    pub fn zeros<Sh>(shape: Sh) -> Self
        where A: Clone + Zero,
              Sh: ShapeBuilder<Dim=D>,
    {
        Self::from_elem(shape, A::zero())
    }

    /// Create an array with default values, shape `shape`
    ///
    /// **Panics** if the number of elements in `shape` would overflow usize.
    pub fn default<Sh>(shape: Sh) -> Self
        where A: Default,
              Sh: ShapeBuilder<Dim=D>,
    {
        let shape = shape.into_shape();
        let size = size_checked_unwrap!(shape.dim);
        let v = to_vec((0..size).map(|_| A::default()));
        unsafe { Self::from_shape_vec_unchecked(shape, v) }
    }

    /// Create an array with values created by the function `f`.
    ///
    /// `f` is called with the index of the element to create; the elements are
    /// visited in arbitirary order.
    ///
    /// **Panics** if the number of elements in `shape` would overflow usize.
    pub fn from_shape_fn<Sh, F>(shape: Sh, f: F) -> Self
        where Sh: ShapeBuilder<Dim=D>,
              F: FnMut(D::Pattern) -> A,
    {
        let shape = shape.into_shape();
        let _ = size_checked_unwrap!(shape.dim);
        if shape.is_c {
            let v = to_vec_mapped(indices(shape.dim.clone()).into_iter(), f);
            unsafe { Self::from_shape_vec_unchecked(shape, v) }
        } else {
            let dim = shape.dim.clone();
            let v = to_vec_mapped(indexes::indices_iter_f(dim).into_iter(), f);
            unsafe { Self::from_shape_vec_unchecked(shape, v) }
        }
    }

    /// Create an array with the given shape from a vector. (No cloning of
    /// elements needed.)
    ///
    /// ---- 
    ///
    /// For a contiguous c- or f-order shape, the following applies:
    ///
    /// **Errors** if `shape` does not correspond to the number of elements in `v`.
    ///
    /// ---- 
    ///
    /// For custom strides, the following applies:
    ///
    /// **Errors** if strides and dimensions can point out of bounds of `v`.<br>
    /// **Errors** if strides allow multiple indices to point to the same element.
    ///
    /// ```
    /// use ndarray::Array;
    /// use ndarray::ShapeBuilder; // Needed for .strides() method
    /// use ndarray::arr2;
    ///
    /// let a = Array::from_shape_vec((2, 2), vec![1., 2., 3., 4.]);
    /// assert!(a.is_ok());
    ///
    /// let b = Array::from_shape_vec((2, 2).strides((1, 2)),
    ///                               vec![1., 2., 3., 4.]).unwrap();
    /// assert!(
    ///     b == arr2(&[[1., 3.],
    ///                 [2., 4.]])
    /// );
    /// ```
    pub fn from_shape_vec<Sh>(shape: Sh, v: Vec<A>) -> Result<Self, ShapeError>
        where Sh: Into<StrideShape<D>>,
    {
        // eliminate the type parameter Sh as soon as possible
        Self::from_shape_vec_impl(shape.into(), v)
    }

    fn from_shape_vec_impl(shape: StrideShape<D>, v: Vec<A>) -> Result<Self, ShapeError>
    {
        if shape.custom {
            Self::from_vec_dim_stride(shape.dim, shape.strides, v)
        } else {
            let dim = shape.dim;
            let strides = shape.strides;
            if dim.size_checked() != Some(v.len()) {
                return Err(error::incompatible_shapes(&Ix1(v.len()), &dim));
            }
            unsafe { Ok(Self::from_vec_dim_stride_unchecked(dim, strides, v)) }
        }
    }

    /// Create an array from a vector and interpret it according to the
    /// provided dimensions and strides. (No cloning of elements needed.)
    ///
    /// Unsafe because dimension and strides are unchecked.
    pub unsafe fn from_shape_vec_unchecked<Sh>(shape: Sh, v: Vec<A>) -> Self
        where Sh: Into<StrideShape<D>>,
    {
        let shape = shape.into();
        Self::from_vec_dim_stride_unchecked(shape.dim, shape.strides, v)
    }

    fn from_vec_dim_stride(dim: D, strides: D, v: Vec<A>)
        -> Result<Self, ShapeError>
    {
        dimension::can_index_slice(&v, &dim, &strides).map(|_| {
            unsafe {
                Self::from_vec_dim_stride_unchecked(dim, strides, v)
            }
        })
    }

    unsafe fn from_vec_dim_stride_unchecked(dim: D, strides: D, mut v: Vec<A>)
        -> Self
    {
        // debug check for issues that indicates wrong use of this constructor
        debug_assert!(match dimension::can_index_slice(&v, &dim, &strides) {
            Ok(_) => true,
            Err(ref e) => match e.kind() {
                ErrorKind::OutOfBounds => false,
                ErrorKind::RangeLimited => false,
                _ => true,
            }
        });
        ArrayBase {
            ptr: v.as_mut_ptr(),
            data: DataOwned::new(v),
            strides: strides,
            dim: dim
        }
    }

    /// Create an array with uninitalized elements, shape `shape`.
    ///
    /// **Panics** if the number of elements in `shape` would overflow usize.
    ///
    /// ### Safety
    ///
    /// Accessing uninitalized values is undefined behaviour. You must
    /// overwrite *all* the elements in the array after it is created; for
    /// example using the methods `.fill()` or `.assign()`.
    ///
    /// The contents of the array is indeterminate before initialization and it
    /// is an error to perform operations that use the previous values. For
    /// example it would not be legal to use `a += 1.;` on such an array.
    ///
    /// This constructor is limited to elements where `A: Copy` (no destructors)
    /// to avoid users shooting themselves too hard in the foot; it is not
    /// a problem to drop an array created with this method even before elements
    /// are initialized. (Note that constructors `from_shape_vec` and
    /// `from_shape_vec_unchecked` allow the user yet more control).
    ///
    /// ### Examples
    ///
    /// ```
    /// #[macro_use(s)]
    /// extern crate ndarray;
    ///
    /// use ndarray::Array2;
    ///
    /// // Example Task: Let's create a column shifted copy of a in b
    ///
    /// fn shift_by_two(a: &Array2<f32>) -> Array2<f32> {
    ///     let mut b = unsafe { Array2::uninitialized(a.dim()) };
    ///
    ///     // two first columns in b are two last in a
    ///     // rest of columns in b are the initial columns in a
    ///     b.slice_mut(s![.., ..2]).assign(&a.slice(s![.., -2..]));
    ///     b.slice_mut(s![.., 2..]).assign(&a.slice(s![.., ..-2]));
    ///
    ///     // `b` is safe to use with all operations at this point
    ///     b
    /// }
    ///
    /// # fn main() {
    /// #   shift_by_two(&Array2::zeros((8, 8)));
    /// # }
    /// ```
    pub unsafe fn uninitialized<Sh>(shape: Sh) -> Self
        where A: Copy,
              Sh: ShapeBuilder<Dim=D>,
    {
        let shape = shape.into_shape();
        let size = size_checked_unwrap!(shape.dim);
        let mut v = Vec::with_capacity(size);
        v.set_len(size);
        Self::from_shape_vec_unchecked(shape, v)
    }

}
