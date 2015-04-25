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

/// Returns the reciprocal (inverse) of float number `x`.
///
/// # Example
///
/// ```
/// use glm::ext::recip;
///
/// let x = 2f32;
/// let y = glm::vec3(1., 2., 4.);
/// assert_eq!(recip(x), 0.5);
/// assert_eq!(recip(y), glm::vec3(1., 0.5, 0.25));
/// ```
#[inline(always)]
pub fn recip<F: BaseFloat, T: GenFloat<F>>(x: T) -> T {
    x.map(Float::recip)
}
