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

use basenum::BaseFloat;
use vec::traits::GenFloatVec;
use mat::traits::GenSquareMat;

pub use self::transform::*;

mod transform;

/// Returns the trace of a square matrix `m`.
///
/// # Example
///
/// ```
/// use glm::mat2;
/// use glm::ext::trace;
///
/// let m2 = mat2(1., 3., 2., 4.);
/// assert_eq!(trace(&m2), 5.);
/// ```
#[inline]
pub fn trace<F: BaseFloat, C: GenFloatVec<F>, M: GenSquareMat<F, C>>(m: &M) -> F {
    let s = C::dim();
    let mut tr = F::zero();
    for i in 0..s {
        tr = tr + m[i][i];
    };
    tr
}

/// Returns `true` if the square matrix `m` is invertible, i.e., its determinant
/// does not close or equal to `0`.
///
/// # Example
///
/// ```
/// use glm::mat2;
/// use glm::ext::is_invertible;
///
/// let m1 = mat2(1., 2., 3., 6.);
/// assert!(!is_invertible(&m1));
/// let m2 = mat2(1., 2., 3., 4.);
/// assert!(is_invertible(&m2));
/// ```
#[inline]
pub fn is_invertible
<
F: BaseFloat, C: GenFloatVec<F>, M: GenSquareMat<F, C>
>(m: &M) -> bool {
    let y = F::zero();
    !m.determinant().is_approx_eq(&y)
}
