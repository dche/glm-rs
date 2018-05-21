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

// The GLSL Specification, ch 8.5, Geometric Functions.
//
// NOTE:
// - `ftransform` is not implemented.

use basenum::BaseFloat;
use traits::GenFloat;
use vec::traits::GenFloatVec;
use vec::vec::Vector3;
use super::exp::inversesqrt;

/// Returns the dot product of `x` and `y`, i.e.,
/// `x[0] * y[0] + x[1] * y[1] + ...`.
///
/// # Example
///
/// ```
/// use glm::{ dot, vec2 };
///
/// let v1 = vec2(1., 2.);
/// let v2 = vec2(3., 4.);
/// assert_eq!(dot(v1, v2), 11.);
/// ```
#[inline(always)]
pub fn dot<S: BaseFloat, T: GenFloatVec<S>>(x: T, y: T) -> S {
    (x * y).sum()
}

/// Returns the length of vector `x`, i.e., `sqrt(x[0]^2 + x[1]^2 + ...)`.
///
/// # Example
///
/// ```
/// assert_eq!(glm::length(glm::vec2(3., 4.)), 5.);
/// ```
#[inline(always)]
pub fn length<S: BaseFloat, T: GenFloatVec<S>>(x: T) -> S {
    dot(x, x).sqrt()
}

/// Returns a vector in the same direction as `x` but with a length of `1`.
///
/// # Example
///
/// ```
/// use glm::{ normalize, dvec2, ApproxEq };
///
/// assert!(normalize(dvec2(3., 4.)).is_approx_eq(&dvec2(0.6, 0.8)));
/// ```
#[inline(always)]
pub fn normalize<S: BaseFloat + GenFloat<S>, T: GenFloatVec<S>>(x: T) -> T {
    x * inversesqrt(dot(x, x))
}

/// Returns the distance between `p0` and `p1`, i.e., `length(p0 – p1)`.
///
/// # Example
///
/// ```
/// use glm::{ distance, vec2 };
///
/// let v1 = vec2(1., 2.);
/// let v2 = vec2(4., 6.);
/// assert_eq!(distance(v1, v2), 5.);
/// ```
#[inline(always)]
pub fn distance<S: BaseFloat, T: GenFloatVec<S>>(p0: T, p1: T) -> S {
    length(p0 - p1)
}

/// If `dot(Nref, I) < 0` return *N*, otherwise return *-N*.
#[inline]
#[allow(non_snake_case)]
pub fn faceforward<S: BaseFloat, T: GenFloatVec<S>>(N: T, I: T, Nref: T) -> T {
    let ling = S::zero();
    if dot(Nref, I) < ling {
        N
    } else {
        -N
    }
}

/// For the incident vector *I* and surface orientation *N*,
/// returns the reflection direction: `I - 2 ∗ dot(N, I) ∗ N`.
///
/// *N* must already be normalized in order to achieve the desired result.
#[inline]
#[allow(non_snake_case)]
pub fn reflect<S: BaseFloat, T: GenFloatVec<S>>(I: T, N: T) -> T {
    let d = dot(N, I);
    I - N * (d + d)
}

/// For the incident vector *I* and surface normal *N*, and the ratio of
/// indices of refraction `eta`, return the refraction vector.
///
/// The result is computed by,
/// ```ignore
/// k = 1.0 - eta * eta * (1.0 - dot(N, I) * dot(N, I))
/// if (k < 0.0)
///     return genType(0.0) // or genDType(0.0)
/// else
///     return eta * I - (eta * dot(N, I) + sqrt(k)) * N
/// ```
///
/// The input parameters for the incident vector *I* and the surface normal *N*
/// must already be normalized to get the desired results.
#[inline]
#[allow(non_snake_case)]
pub fn refract<S: BaseFloat, T: GenFloatVec<S>>(I: T, N: T, eta: S) -> T {
    let dot_ni = dot(I, N);
    let yi = S::one();
    let ling = S::zero();

    let k = yi - eta * eta * (yi - dot_ni) * dot_ni;
    if k < ling {
        T::zero()
    } else {
        I * eta - N * (eta * dot_ni + k.sqrt())
    }
}

/// Returns the cross product of `x` and `y`.
///
/// # Example
///
/// ```
/// use glm::vec3;
///
/// let x = vec3(1.0, 0.0, 0.0);
/// let y = vec3(0.0, 1.0, 0.0);
/// let z = vec3(0.0, 0.0, 1.0);
/// assert_eq!(glm::cross(x, y), z);
/// ```
#[inline]
pub fn cross<F: BaseFloat>(x: Vector3<F>, y: Vector3<F>) -> Vector3<F> {
    Vector3::new(
        x.y * y.z - y.y * x.z,
        x.z * y.x - y.z * x.x,
        x.x * y.y - y.x * x.y
    )
}
