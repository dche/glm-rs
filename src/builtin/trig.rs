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

// The GLSL Specification, ch 8.1, Angle and Trigonometry Functions.

use basenum::BaseFloat;
use traits::GenFloat;
use num::Float;

/// Converts `degrees` to radians, i.e., `π/180 * degrees`.
#[inline(always)]
pub fn radians<F: BaseFloat, T: GenFloat<F>>(degrees: T) -> T {
    degrees.map(BaseFloat::to_radians)
}

/// Converts `radians` to degrees, i.e., `180/π * radians`.
#[inline(always)]
pub fn degrees<F: BaseFloat, T: GenFloat<F>>(radians: T) -> T {
    radians.map(BaseFloat::to_degrees)
}

/// The standard trigonometric sine function.
#[inline(always)]
pub fn sin<F: BaseFloat, T: GenFloat<F>>(angle: T) -> T {
    angle.map(Float::sin)
}

/// The standard trigonometric cosine function.
#[inline(always)]
pub fn cos<F: BaseFloat, T: GenFloat<F>>(angle: T) -> T {
    angle.map(Float::cos)
}

/// The standard trigonometric tangent.
#[inline(always)]
pub fn tan<F: BaseFloat, T: GenFloat<F>>(angle: T) -> T {
    angle.map(Float::tan)
}

/// Returns an angle whose sine is `x`.
///
/// The range of values returned by this function is `[-π/2, π/2]`.
///
/// Results are undefined if `|x| > 1`.
#[inline(always)]
pub fn asin<F: BaseFloat, T: GenFloat<F>>(x: T) -> T {
    x.map(Float::asin)
}

/// Returns an angle whose cosine is `x`.
///
/// The range of values returned by this function is `[0, π]`.
///
/// Results are undefined if `∣x∣ > 1`.
#[inline(always)]
pub fn acos<F: BaseFloat, T: GenFloat<F>>(x: T) -> T {
    x.map(Float::acos)
}

/// Returns an angle whose tangent is `y / x`.
///
/// The signs of `x` and `y` are used to determine what quadrant the angle is
/// in.
///
/// The range of values returned by this function is `[−π, π]`.
///
/// Results are undefined if `x` and `y` are both `0`.
///
/// # Note
///
/// `atan2` is not a GLSL function name.
#[inline(always)]
pub fn atan2<F: BaseFloat, T: GenFloat<F>>(y: T, x: T) -> T {
    x.zip(y, Float::atan2)
}

/// Returns an angle whose tangent is `y_over_x`.
///
/// The range of values returned by this function is `[-π/2, π/2]`.
#[inline(always)]
pub fn atan<F: BaseFloat, T: GenFloat<F>>(y_over_x: T) -> T {
    y_over_x.map(Float::atan)
}

/// Returns the hyperbolic sine function (e<sup>x</sup> - e<sup>-x</sup>) / 2.
#[inline(always)]
pub fn sinh<F: BaseFloat, T: GenFloat<F>>(x: T) -> T {
    x.map(Float::sinh)
}

/// Returns the hyperbolic cosine function (e<sup>x</sup> + e<sup>-x</sup>) / 2.
#[inline(always)]
pub fn cosh<F: BaseFloat, T: GenFloat<F>>(x: T) -> T {
    x.map(Float::cosh)
}

/// Returns the hyperbolic tangent function `sinh(x)/cosh(x)`.
#[inline(always)]
pub fn tanh<F: BaseFloat, T: GenFloat<F>>(x: T) -> T {
    x.map(Float::tanh)
}

/// Arc hyperbolic sine; returns the inverse of **sinh**.
#[inline(always)]
pub fn asinh<F: BaseFloat, T: GenFloat<F>>(x: T) -> T {
    x.map(Float::asinh)
}

/// Arc hyperbolic cosine; returns the non-negative inverse of **cosh**.
///
/// Results are undefined if `x < 1`.
#[inline(always)]
pub fn acosh<F: BaseFloat, T: GenFloat<F>>(x: T) -> T {
    x.map(Float::acosh)
}

/// Arc hyperbolic tangent; returns the inverse of **tanh**.
///
/// Results are undefined if `∣x∣ ≥ 1`.
#[inline(always)]
pub fn atanh<F: BaseFloat, T: GenFloat<F>>(x: T) -> T {
    x.map(Float::atanh)
}
