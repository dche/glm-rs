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

// The GLSL Specification, ch 8.7, Vector Relational Functions

use basenum::Primitive;
use vec::traits::{ GenVec, GenBVec };
use vec::vec::{ Vector2, Vector3, Vector4 };
use std::cmp::{ PartialEq, PartialOrd };

pub trait VecRel<T: Primitive, B: GenBVec>: GenVec<T> {
    fn zip_bool(&self, rhs: &Self, fn(&T, &T) -> bool) -> B;
}

macro_rules! impl_vecrel_for(
    ($t: ident, $($field: ident), +) => {
        impl<T: Primitive> VecRel<T, $t<bool>> for $t<T> {
            #[inline(always)]
            fn zip_bool(&self, rhs: &$t<T>, oper: fn(&T, &T) -> bool) -> $t<bool> {
                $t::new($(oper(&(self.$field), &(rhs.$field))), +)
            }
        }
    }
);

impl_vecrel_for! { Vector2, x, y }
impl_vecrel_for! { Vector3, x, y, z }
impl_vecrel_for! { Vector4, x, y, z, w }

/// Returns the component-wise compare of `x < y`.
///
/// # Example
///
/// ```
/// use glm::*;
///
/// let a = ivec4(1, 2, 3, 4);
/// let b = ivec4(2, 3, 4, 5);
/// assert_eq!(lessThan(b, a), bvec4(false, false, false, false));
/// assert!(all(lessThan(a, b)));
/// ```
#[inline(always)]
#[allow(non_snake_case)]
pub fn lessThan<T: Primitive, B: GenBVec, V: VecRel<T, B>>(x: V, y: V) -> B {
    x.zip_bool(&y, PartialOrd::lt)
}

/// Returns the component-wise compare of `x ≤ y`.
///
/// # Example
///
/// ```
/// use glm::*;
///
/// let a = ivec4(1, 2, 3, 4);
/// let b = ivec4(2, 2, 3, 3);
/// assert_eq!(lessThanEqual(a, b), bvec4(true, true, true, false));
/// assert_eq!(lessThanEqual(a, b), not(greaterThan(a, b)));
/// ```
#[inline(always)]
#[allow(non_snake_case)]
pub fn lessThanEqual
<
T: Primitive, B: GenBVec, V: VecRel<T, B>
>(x: V, y: V) -> B {
    x.zip_bool(&y, PartialOrd::le)
}

/// Returns the component-wise compare of `x > y`.
///
/// # Example
///
/// ```
/// use glm::*;
///
/// let a = ivec4(1, 2, 3, 4);
/// let b = ivec4(2, 2, 3, 3);
/// assert_eq!(greaterThan(a, b), bvec4(false, false, false, true));
/// assert_eq!(greaterThan(a, b), lessThan(b, a));
/// ```
#[inline(always)]
#[allow(non_snake_case)]
pub fn greaterThan
<
T: Primitive, B: GenBVec, V: VecRel<T, B>
>(x: V, y: V) -> B {
    x.zip_bool(&y, PartialOrd::gt)
}

/// Returns the component-wise compare of `x ≥ y`.
///
/// # Example
///
/// ```
/// use glm::*;
///
/// let a = ivec4(1, 2, 3, 4);
/// let b = ivec4(2, 2, 3, 3);
/// assert_eq!(greaterThanEqual(a, b), bvec4(false, true, true, true));
/// assert_eq!(greaterThanEqual(a, b), lessThanEqual(b, a));
/// assert_eq!(greaterThanEqual(a, b), not(lessThan(a, b)));
/// ```
#[inline(always)]
#[allow(non_snake_case)]
pub fn greaterThanEqual
<
T: Primitive, B: GenBVec, V: VecRel<T, B>
>(x: V, y: V) -> B {
    x.zip_bool(&y, PartialOrd::ge)
}

/// Returns the component-wise compare of `x == y`.
///
/// # Example
///
/// ```
/// use glm::*;
///
/// let a = ivec4(1, 2, 3, 4);
/// let b = ivec4(2, 2, 3, 3);
/// assert_eq!(equal(a, b), bvec4(false, true, true, false));
/// assert_eq!(equal(a, b), equal(b, a));
/// ```
#[inline(always)]
pub fn equal<T: Primitive, B: GenBVec, V: VecRel<T, B>>(x: V, y: V) -> B {
    x.zip_bool(&y, PartialEq::eq)
}

/// Returns the component-wise compare of `x ≠ y`.
///
/// # Example
///
/// ```
/// use glm::*;
///
/// let a = ivec4(1, 2, 3, 4);
/// let b = ivec4(2, 2, 3, 3);
/// assert_eq!(notEqual(a, b), bvec4(true, false, false, true));
/// assert_eq!(notEqual(a, b), not(equal(b, a)));
/// ```
#[inline(always)]
#[allow(non_snake_case)]
pub fn notEqual<T: Primitive, B: GenBVec, V: VecRel<T, B>>(x: V, y: V) -> B {
    x.zip_bool(&y, PartialEq::ne)
}

/// Returns `true` if any component of `x` is **true**.
#[inline(always)]
pub fn any<T: GenBVec>(bvec: T) -> bool {
    bvec.any()
}

/// Returns `true` only if all components of `x` are **true**.
#[inline(always)]
pub fn all<T: GenBVec>(bvec: T) -> bool {
    bvec.all()
}

/// Returns the component-wise logical complement of `x`.
#[inline(always)]
pub fn not<T: GenBVec>(bvec: T) -> T {
    bvec.not()
}
