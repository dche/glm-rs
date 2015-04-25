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
use num::Float;

/// Simultaneously computes the sine and cosine of `x`, returns
/// `(sin(x), cos(x))`.
///
/// # Example
///
/// ```rust
/// use glm::*;
/// use glm::ext::*;
///
/// let v = vec2(0., half_pi());
/// let (s, c) = sin_cos(v);
/// assert!(is_approx_eq(&s, &vec2(0., 1.)));
/// assert!(is_approx_eq(&c, &vec2(1., 0.)));
/// ```
#[inline(always)]
pub fn sin_cos<F: BaseFloat, T: GenFloat<F>>(x: T) -> (T, T) {
    x.split(Float::sin_cos)
}
