//
// GLSL Mathematics for Rust.
//
// Copyright (c) 2015 The glm-rs authors.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

use basenum::{ BaseFloat, ApproxEq };
use vec::traits::GenFloatVec;
use std::ops::{ Add, Sub, Neg, Mul, Div, Rem, Index, IndexMut };
use num::{ One, Zero };

/// Generic Matrix type.
pub trait GenMat
<
T: BaseFloat,
C: GenFloatVec<T>
>
: Sized
+ Zero
+ Add<Output = Self>
+ Sub<Output = Self>
+ Div<Output = Self>
+ Rem<Output = Self>
+ Neg<Output = Self>
+ Mul<T, Output = Self>
+ Index<usize, Output = C>
+ IndexMut<usize>
+ ApproxEq<BaseType = T>
{
    /// Type of row vectors.
    type R: GenFloatVec<T>;

    /// Type of transpose matrix.
    type Transpose: GenMat<T, Self::R, R = C, Transpose = Self>;

    /// Returns the transpose matrix.
    ///
    /// # Example
    ///
    /// ```
    /// use glm::GenMat;    // bing the method into scope.
    ///
    /// let m = glm::mat3x2(1., 2., 3., 4., 5., 6.);
    /// let tm = glm::mat2x3(1., 3., 5., 2., 4., 6.);
    /// assert_eq!(tm, m.transpose());
    /// ```
    fn transpose(&self) -> Self::Transpose;

    /// Component-wise multiplication.
    ///
    /// # Example
    ///
    /// ```
    /// use glm::GenMat;    // bing the method into scope.
    ///
    /// let m1 = glm::mat2(1., 2., 3., 4.);
    /// let m2 = glm::mat2(0., 0., -7., 0.5);
    /// assert_eq!(m1.mul_c(&m2), glm::mat2(0., 0., -21., 2.));
    /// ```
    fn mul_c(&self, rhs: &Self) -> Self;
}

/// Generic type of square matrix.
pub trait GenSquareMat
<
T: BaseFloat,
C: GenFloatVec<T>,
>
: GenMat<T, C, R = C, Transpose = Self>
+ One
{
    /// Returns the determinant of a square matrix.
    ///
    /// # Example
    ///
    /// ```
    /// use glm::GenSquareMat;
    ///
    /// let mat = glm::mat2(1., 3., 2., 4.);
    /// assert_eq!(mat.determinant(), -2.);
    /// ```
    fn determinant(&self) -> T;

    /// Returns the inverse matrix of a square matrix, or `None` if the
    /// matrix is not invertible.
    fn inverse(&self) -> Option<Self>;
}
