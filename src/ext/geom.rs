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
use traits::GenFloat;
use vec::traits::GenFloatVec;
use builtin as bif;

/// Returns the squre of the length of vector `x`.
///
/// # Example
///
/// ```rust
/// use glm::*;
/// use glm::ext::*;
///
/// assert_eq!(sqlength(vec2(1., 2.)), 5.);
/// ```
#[inline(always)]
pub fn sqlength<F: BaseFloat, T: GenFloatVec<F>>(x: T) -> F {
    bif::dot(x, x)
}

/// Returns the reciprocal (inverse) of the length of vector `x`.
///
/// # Example
///
/// ```rust
/// use glm::*;
/// use glm::ext::*;
///
/// let v = vec2(3., 4.);
/// assert_eq!(recip_length(v), 0.2);
/// ```
#[inline(always)]
pub fn recip_length<F: BaseFloat + GenFloat<F>, T: GenFloatVec<F>>(x: T) -> F {
    bif::inversesqrt(sqlength(x))
}

/// Normalizes vector `x` of specific length `len`.
///
/// # Example
///
/// ```
/// use glm::length;
/// use glm::ext::normalize_to;
///
/// let v = glm::vec2(3., 4.);
/// assert_eq!(length(normalize_to(v, 2.)), 2.);
/// ```
#[inline(always)]
pub fn normalize_to<F: BaseFloat + GenFloat<F>, T: GenFloatVec<F>>(x: T, len: F) -> T {
    bif::normalize(x) * len
}

/// Projects `x` on `y`.
///
/// # Example
///
/// ```
/// use glm::vec2;
/// use glm::ext::projection;
///
/// assert_eq!(projection(vec2(1., 0.), vec2(1., 1.)), vec2(0.5, 0.5));
/// ```
#[inline]
pub fn projection<F: BaseFloat, T: GenFloatVec<F>>(x: T, y: T) -> T {
    let ling = F::zero();
    let sqlen = sqlength(y);
    if sqlen.is_approx_eq(&ling) {
        T::zero()
    } else {
        y * bif::dot(x, y) * sqlen.recip()
    }
}

/// Returns `true` if vector `x` is perpendicular to `y`, i.e., angle between
/// `x` and `y` is π/2.
///
/// # Example
///
/// ```rust
/// use glm::ext::is_perpendicular;
///
/// let x = glm::vec2(1., 0.);
/// let y = glm::vec2(0., 1.);
/// assert!(is_perpendicular(x, y));
/// ```
#[inline(always)]
pub fn is_perpendicular<F: BaseFloat, T: GenFloatVec<F>>(x: T, y: T) -> bool {
    bif::dot(x, y).is_approx_eq(&F::zero())
}

/// Returns angle between vectors `x` and `y`.
///
/// The return value is in radian unit and in the interval [0, π].
///
/// # Note
///
/// - `x` and `y` need be normalized to get meaningful result.
/// - If either `x` or `y` is zero, the angle is undefined, and `0` is returned.
///
/// # Example
///
/// ```
/// use glm::*;
/// use glm::ext::*;
///
/// let vx = vec2(1., 0.);
/// let vy = vec2(0., 1.);
/// assert!(is_approx_eq(&angle(vx, vy), &half_pi()));
/// assert!(is_approx_eq(&angle(vy, vx), &half_pi()));
/// ```
#[inline]
pub fn angle<F: BaseFloat + GenFloat<F>, T: GenFloatVec<F>>(x: T, y: T) -> F {
    let ling = F::zero();
    let sqmag = bif::dot(x, x) * bif::dot(y, y);
    if sqmag.is_approx_eq(&ling) {
        ling
    } else {
        (bif::dot(x, y) * bif::inversesqrt(sqmag)).acos()
    }
}
