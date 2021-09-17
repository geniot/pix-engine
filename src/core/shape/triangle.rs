//! A shape type representing triangles used for drawing.
//!
//! # Examples
//!
//! You can create a [Triangle][Tri] using [Tri::new]:
//!
//! ```
//! use pix_engine::prelude::*;
//!
//! // 2D
//! let tri: TriI2 = Tri::new([10, 20], [30, 10], [20, 25]);
//!
//! let p1 = point!(10, 20);
//! let p2 = point!(30, 10);
//! let p3 = point!(20, 25);
//! let tri: TriI2 = Tri::new(p1, p2, p3);
//! ```

use crate::prelude::*;
use num_traits::AsPrimitive;
// #[cfg(feature = "serde")]
// use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut, Index, IndexMut};

/// A `Triangle` with three [Point]s.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
// #[cfg_ater(feature = "serde", derive(Serialize, Deserialize))]
pub struct Tri<T, const N: usize>([Point<T, N>; 3]);

impl<T, const N: usize> Tri<T, N> {
    /// Constructs a `Triangle` with the given [Point]s.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let tri: TriI2 = Tri::new([10, 20], [30, 10], [20, 25]);
    /// assert_eq!(tri.p1().values(), [10, 20]);
    /// assert_eq!(tri.p2().values(), [30, 10]);
    /// assert_eq!(tri.p3().values(), [20, 25]);
    /// ```
    pub fn new<P>(p1: P, p2: P, p3: P) -> Self
    where
        P: Into<Point<T, N>>,
    {
        Self([p1.into(), p2.into(), p3.into()])
    }
}

/// A 2D `Triangle` represented by integers.
pub type TriI2 = Tri<i32, 2>;

/// A 3D `Triangle` represented by integers.
pub type TriI3 = Tri<i32, 3>;

/// A 2D `Triangle` represented by floating point numbers.
pub type TriF2 = Tri<Scalar, 2>;

/// A 3D `Triangle` represented by floating point numbers.
pub type TriF3 = Tri<Scalar, 3>;

impl<T, const N: usize> Tri<T, N>
where
    T: Copy,
{
    /// Returns the first point of the triangle.
    #[inline(always)]
    pub fn p1(&self) -> Point<T, N> {
        self.0[0]
    }

    /// Sets the first point of the triangle.
    #[inline(always)]
    pub fn set_p1<P>(&mut self, p: P)
    where
        P: Into<Point<T, N>>,
    {
        self.0[0] = p.into();
    }

    /// Returns the second point of the triangle.
    #[inline(always)]
    pub fn p2(&self) -> Point<T, N> {
        self.0[1]
    }

    /// Sets the second point of the triangle.
    #[inline(always)]
    pub fn set_p2<P>(&mut self, p: P)
    where
        P: Into<Point<T, N>>,
    {
        self.0[1] = p.into();
    }

    /// Returns the third point of the triangle.
    #[inline(always)]
    pub fn p3(&self) -> Point<T, N> {
        self.0[2]
    }

    /// Sets the third point of the triangle.
    #[inline(always)]
    pub fn set_p3<P>(&mut self, p: P)
    where
        P: Into<Point<T, N>>,
    {
        self.0[2] = p.into();
    }

    /// Convert `Tri<T>` to to `Tri<U>` using the `as` operator.
    #[inline]
    pub fn as_<U>(self) -> Tri<U, N>
    where
        T: AsPrimitive<U>,
        U: 'static + Copy + Default,
    {
        Tri::new(self.p1().as_(), self.p2().as_(), self.p3().as_())
    }

    /// Returns `Triangle` points as `[Point<T, N>; 3]`.
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let tri: TriI2 = Tri::new([10, 20], [30, 10], [20, 25]);
    /// assert_eq!(tri.values(), [
    ///     point!(10, 20),
    ///     point!(30, 10),
    ///     point!(20, 25),
    /// ]);
    /// ```
    pub fn values(&self) -> [Point<T, N>; 3] {
        self.0
    }

    /// Returns `Triangle` as a [Vec].
    ///
    /// # Example
    ///
    /// ```
    /// # use pix_engine::prelude::*;
    /// let tri: TriI2 = Tri::new([10, 20], [30, 10], [20, 25]);
    /// assert_eq!(
    ///   tri.to_vec(),
    ///   vec![
    ///     point!(10, 20),
    ///     point!(30, 10),
    ///     point!(20, 25),
    ///   ]
    /// );
    /// ```
    pub fn to_vec(self) -> Vec<Point<T, N>> {
        self.0.to_vec()
    }
}

impl<T, const N: usize> Draw for Tri<T, N>
where
    Self: Into<TriI2>,
    T: Copy,
{
    /// Draw `Triangle` to the current [PixState] canvas.
    fn draw(&self, s: &mut PixState) -> PixResult<()> {
        s.triangle(*self)
    }
}

impl<T, const N: usize> Deref for Tri<T, N> {
    type Target = [Point<T, N>; 3];
    /// Deref `Tri` to `&[Point; 2]`.
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T, const N: usize> DerefMut for Tri<T, N> {
    /// Deref `Tri` to `&mut [Point; 2]`.
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, const N: usize> Index<usize> for Tri<T, N>
where
    T: Copy,
{
    type Output = Point<T, N>;
    /// Return `&T` by indexing `Tri` with `usize`.
    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl<T, const N: usize> IndexMut<usize> for Tri<T, N>
where
    T: Copy,
{
    /// Return `&mut T` by indexing `Tri` with `usize`.
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.0[idx]
    }
}

impl<T, const N: usize> From<&Tri<T, N>> for Tri<T, N>
where
    T: Copy,
{
    /// Convert &[Tri] to [Tri].
    fn from(tri: &Tri<T, N>) -> Self {
        *tri
    }
}

impl<T, const N: usize> From<&mut Tri<T, N>> for Tri<T, N>
where
    T: Copy,
{
    /// Convert &mut [Tri] to [Tri].
    fn from(tri: &mut Tri<T, N>) -> Self {
        *tri
    }
}

impl<T, const N: usize> From<Tri<T, N>> for [Point<T, N>; 3]
where
    T: Copy,
{
    /// Convert [Tri] to `[Point; 3]`.
    fn from(tri: Tri<T, N>) -> Self {
        tri.values()
    }
}

impl<T, const N: usize> From<&Tri<T, N>> for [Point<T, N>; 3]
where
    T: Copy,
{
    /// Convert &[Tri] to `[Point; 3]`.
    fn from(tri: &Tri<T, N>) -> Self {
        tri.values()
    }
}

impl<T, U, const N: usize> From<[Point<U, N>; 3]> for Tri<T, N>
where
    Point<U, N>: Into<Point<T, N>>,
{
    /// Convert `[Point<U, N>; 3]` to [`Tri<T, N>`].
    fn from([p1, p2, p3]: [Point<U, N>; 3]) -> Self {
        Self::new(p1, p2, p3)
    }
}

impl<T, U, const N: usize> From<&[Point<U, N>; 3]> for Tri<T, N>
where
    U: Copy,
    Point<U, N>: Into<Point<T, N>>,
{
    /// Convert `&[<Point<U, N>; 3]` to [`Tri<T, N>`].
    fn from(&[p1, p2, p3]: &[Point<U, N>; 3]) -> Self {
        Self::new(p1, p2, p3)
    }
}
