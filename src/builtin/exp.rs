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
// The GLSL Specification, ch 8.2, Exponential Functions.

use basenum::BaseFloat;
use traits::GenFloat;
use num::Float;

/// Returns `x` raised to the `y` power, i.e., *x<sup>y</sup>*.
///
/// Results are undefined if `x < 0`.
///
/// Results are undefined if `x = 0` and `y ≤ 0`.
///
/// # Example
///
/// ```
/// use glm::{ pow, vec3 };
///
/// assert_eq!(pow(2.0, 3.0), 8.0);
/// let v1 = vec3(1.0, 2.0, 3.0);
/// let v2 = vec3(1.0, 4.0, 27.0);
/// assert_eq!(pow(v1, v1), v2);
/// ```
#[inline(always)]
pub fn pow<F: BaseFloat, T: GenFloat<F>>(x: T, y: T) -> T {
    x.zip(y, Float::powf)
}

/// Returns the natural exponentiation of `x`. i.e., *e<sup>x</sup>*.
///
/// # Example
///
/// ```
/// use glm::{ exp, ApproxEq };
/// use glm::ext::e;
///
/// let e1: f32 = e();
/// let e2 = e1 * e1;
/// assert!(exp(2.).is_close_to(&e2, 0.000001));
/// ```
#[inline(always)]
pub fn exp<F: BaseFloat, T: GenFloat<F>>(x: T) -> T {
    x.map(Float::exp)
}

/// Returns the natural logarithm of `x`. i.e., the value `y` which satisfies
/// *x = e<sup>y</sup>*.
///
/// # Example
///
/// ```
/// use glm::{ log, ApproxEq };
/// use glm::ext::e;
///
/// let e1: f64 = e();
/// let e2 = e1 * e1;
/// assert!(log(e2).is_approx_eq(&2.));
/// ```
#[inline(always)]
pub fn log<F: BaseFloat, T: GenFloat<F>>(x: T) -> T {
    x.map(Float::ln)
}

/// Returns `2` raised to the power of `x`. i.e., *2<sup>x</sup>*.
///
/// # Example
///
/// ```
/// use glm::{ exp2, dvec2 };
///
/// assert_eq!(exp2(10_f32), 1024.);
/// assert_eq!(exp2(dvec2(-1., 5.)), dvec2(0.5, 32.));
/// ```
#[inline(always)]
pub fn exp2<F: BaseFloat, T: GenFloat<F>>(x: T) -> T {
    x.map(Float::exp2)
}

/// Returns the base `2` logarithm of `x`. i.e., the value `y` which satisfies
/// *x = 2<sup>y</sup>*.
///
/// Results are undefined if `x < 0`.
///
/// # Example
///
/// ```
/// use glm::{ log2, vec2 };
/// assert_eq!(log2(vec2(64., 256.)), vec2(6., 8.));
/// ```
#[inline(always)]
pub fn log2<F: BaseFloat, T: GenFloat<F>>(x: T) -> T {
    x.map(Float::log2)
}

/// Returns the square root of `x`. i.e., the value `sqrt(x)`.
///
/// Results are undefined if `x < 0`.
///
/// # Example
///
/// ```
/// use glm::{ sqrt, vec2 };
/// assert_eq!(sqrt(vec2(64., 1.)), vec2(8., 1.));
/// ```
#[inline(always)]
pub fn sqrt<F: BaseFloat, T: GenFloat<F>>(x: T) -> T {
    x.map(Float::sqrt)
}

/// Returns the inverse of the square root of `x`. i.e., the value `1/sqrt(x)`.
///
/// Results are undefined if `x ≤ 0`.
///
/// # Example
///
/// ```
/// use glm::{ inversesqrt, vec2 };
/// assert_eq!(inversesqrt(4_f32), 0.5);
/// assert_eq!(inversesqrt(vec2(64., 1.)), vec2(0.125, 1.));
/// ```
#[inline(always)]
pub fn inversesqrt<F: BaseFloat, T: GenFloat<F>>(x: T) -> T {
    x.map(|f| -> F {
        f.sqrt().recip()
    })
}
