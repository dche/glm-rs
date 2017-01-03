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

// The GLSL Specification, ch 8.6, Matrix Functions.

use basenum::BaseFloat;
use mat::traits::{ GenMat, GenSquareMat };
use vec::traits::GenFloatVec;

/// Multiply matrix `x` by matrix `y` component-wise, i.e., `result[i][j]` is
/// the scalar product of `x[i][j]` and `y[i][j]`.
///
/// # Note
///
/// To get linear algebraic matrix multiplication, use the multiply operator
/// `*`.
///
/// # Example
///
/// ```
/// use glm::{ matrixCompMult, mat3x2 };
///
/// let m1 = mat3x2(1., 4., 2., 5., 3., 6.);
/// let m2 = mat3x2(2., 3., 2., 3., 2., 3.);
/// let me = mat3x2(2., 12., 4., 15., 6., 18.);
/// assert_eq!(matrixCompMult(&m1, &m2), me);
/// ```
#[inline(always)]
#[allow(non_snake_case)]
pub fn matrixCompMult<
T: BaseFloat,
C: GenFloatVec<T>,
M: GenMat<T, C>
>(x: &M, y: &M) -> M {
    x.mul_c(y)
}

/// Treats the first parameter `c` as a column vector (matrix with one column)
/// and the second parameter `r` as a row vector (matrix with one row)
/// and does a linear algebraic matrix multiply `c * r`,
/// yielding a matrix whose number of rows is the number of components in `c`
/// and whose number of columns is the number of components in `r`.
///
/// # Example
///
/// ```
/// # use glm::*;
/// let v2 = vec2(1., 2.);
/// let v3 = vec3(4., 0., -1.);
/// let e = mat3x2(4., 8., 0., 0., -1., -2.);
/// let op: Mat3x2 = outerProduct(v2, v3);
/// assert_eq!(op, e);
/// ```
#[inline]
#[allow(non_snake_case)]
pub fn outerProduct<
T: BaseFloat,
C: GenFloatVec<T>,
R: GenFloatVec<T>,
/*
 * NOTE:
 * I can't believe Rust allows this! But Rust is wrong at the first place.
 * Associated types (e.g., `R` and `Transpose` of `GenMat` here) are not type
 * parameters, and should not be mandatorily required when specifying a type,
 * and if `Transpose` is ommitted (as we did before), that does not mean
 * `GenMat` is not implemented for `Tanspose` of `M` (E0277). How could that
 * be possible?
 */
N: GenMat<T, R, R = C, Transpose = M>,
M: GenMat<T, C, R = R, Transpose = N>
>(c: C, r: R) -> M {
    let mut z = M::zero();
    let dim = R::dim();
    for i in 0..dim {
        z[i] = c * r[i];
    };
    z
}

/// Returns a matrix that is the transpose of `m`.
///
/// The input matrix `m` is not modified.
#[inline(always)]
pub fn transpose<
T: BaseFloat,
C: GenFloatVec<T>,
M: GenMat<T, C>
>(m: &M) -> M::Transpose {
    m.transpose()
}

/// Returns the determinant of `m`.
#[inline(always)]
pub fn determinant<
T: BaseFloat,
C: GenFloatVec<T>,
M: GenSquareMat<T, C>
>(m: &M) -> T {
    m.determinant()
}

/// Returns a matrix that is the inverse of `m`.
///
/// The input matrix `m` is not modified.
///
/// # Panic
///
/// It is a panic if `m` is singular or poorly-conditioned (nearly singular).
#[inline]
pub fn inverse<
T: BaseFloat,
C: GenFloatVec<T>,
M: GenSquareMat<T, C>
>(m: &M) -> M {
    let inv = m.inverse();
    match inv {
        Some(im) => im,
        _ => panic!("inverse a matrix that is not invertible.")
    }
}
