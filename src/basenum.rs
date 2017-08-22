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

use std::cmp;
use std::{ f32, f64 };
use std::ops::{ Sub, Div, Rem, Neg };
use rand::Rand;
use num::{ PrimInt, Float, One, Signed, Zero };

/// Marker trait for primitive types.
///
/// # Note
///
/// In `glm`, not all Rust primitive number types are used. Only those types
/// that used in GLSL, i.e., `f32`, `f64`, `i32`, `u32` and `bool`, implement
/// this trait.
pub trait Primitive
: Send + Copy + Sized + Clone + PartialOrd + PartialEq + Rand {}

impl Primitive for bool {}

/// Trait for primitive number type.
pub trait BaseNum
: Primitive
+ Zero
+ One
+ Div<Self, Output = Self>
+ Rem<Self, Output = Self>
{
    /// Returns the smaller one of two numbers.
    ///
    /// # Example
    ///
    /// ```
    /// use glm::BaseNum;
    /// assert_eq!(BaseNum::min(1i32, 2i32), 1i32);
    /// ```
    fn min(self, other: Self) -> Self;

    /// Returns the larger one of two numbers.
    ///
    /// # Example
    ///
    /// ```
    /// use glm::BaseNum;
    /// assert_eq!(BaseNum::max(1i32, 2i32), 2i32);
    /// ```
    fn max(self, other: Self) -> Self;
}

/// Trait for numerical types that have negative values.
pub trait SignedNum
: Sized
+ Neg<Output = Self>
+ Sub<Self, Output = Self>
{
    /// Returns the absolute value of the receiver.
    fn abs(&self) -> Self;

    /// Returns the sign number of the receiver.
    ///
    /// # Example
    ///
    /// ```
    /// use glm::{ SignedNum, dvec3 };
    /// assert_eq!(dvec3(2.718, 0., -0.).sign(), dvec3(1., 0., 0.));
    /// ```
    fn sign(&self) -> Self;
}

/// Marker trait for primitive integer number type.
pub trait BaseInt: PrimInt + BaseNum {}

/// Trait for comparing types that are derived from float numbers.
///
/// # Note
///
/// Comparing float numbers is tricky. This trait is mainly for convenience.
/// See [this article](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/)
/// for the details of comparing float numbers.
pub trait ApproxEq {

    type BaseType: BaseFloat;

    /// Returns `true` if the difference between `x` and `y` is less than
    /// `max_diff`.
    ///
    /// # Note
    ///
    /// The meanings of "difference" and "less than" are up to the
    /// implementations.
    ///
    /// # Example
    ///
    /// use glm::*;
    ///
    /// let v1 = vec2(1000., 2000.);
    /// let v2 = vec2(1010., 1080.);
    /// assert!(v1.is_close_to(&v2, 50.));
    /// assert!(!v1.is_close_to(&v2, 10.));
    fn is_close_to(&self, rhs: &Self, max_diff: Self::BaseType) -> bool;

    /// Returns `true` if the difference between `x` and `y` is less than
    /// [machine epsilon](http://en.wikipedia.org/wiki/Machine_epsilon).
    ///
    /// # Example
    ///
    /// ```
    /// use glm::*;
    ///
    /// let f = 0.1_f32;
    /// let mut sum = 0f32;
    /// for i in 0..10 {
    ///     sum += f;
    /// }
    /// assert_eq!(1.0_f32 == sum, false);
    /// assert_eq!(1f32.is_approx_eq(&sum), true);
    /// ```
    fn is_approx_eq(&self, rhs: &Self) -> bool {
        self.is_close_to(rhs, Self::BaseType::epsilon())
    }
}

/// Returns the result of `x.is_close_to(y, max_diff)`.
#[inline(always)]
pub fn is_close_to<T: ApproxEq>(x: &T, y: &T, max_diff: T::BaseType) -> bool {
    x.is_close_to(y, max_diff)
}

/// Returns the result of `x.is_approx_eq(y)`.
#[inline(always)]
pub fn is_approx_eq<T: ApproxEq>(x: &T, y: &T) -> bool {
    x.is_approx_eq(y)
}

#[macro_export]
macro_rules! assert_approx_eq(
    ($left: expr, $right: expr) => ({
        let lhs = &($left);
        let rhs = &($right);
        if !is_approx_eq(lhs, rhs) {
            panic!(
                "assertion failed: left ≈ right` (left: `{:?}`, right: `{:?}`)",
                *lhs, *rhs,
            )
        }
    })
);

#[macro_export]
macro_rules! assert_close_to(
    ($left: expr, $right: expr, $max_diff: expr) => ({
        let lhs = &($left);
        let rhs = &($right);
        let diff = $max_diff;
        if !is_close_to(lhs, rhs, diff) {
            panic!(
                "assertion failed: left ≈ right` (left: `{:?}`, right: `{:?}`, tolerance: `{:?}`)",
                *lhs, *rhs, diff
            )
        }
    })
);


/// Trait for primitive float number type.
pub trait BaseFloat: Float + BaseNum + SignedNum + ApproxEq<BaseType = Self> {
    fn to_degrees(self) -> Self;
    fn to_radians(self) -> Self;
    fn frexp(self) -> (Self, isize);
    fn ldexp(self, exp: isize) -> Self;
}

impl SignedNum for i32 {
    #[inline(always)]
    fn abs(&self) -> i32 {
        Signed::abs(self)
    }
    #[inline(always)]
    fn sign(&self) -> i32 {
        self.signum()
    }
}

macro_rules! impl_int(
    ($($t: ty), +) => {
        $(
            impl Primitive for $t {}
            impl BaseNum for $t {
                #[inline(always)]
                fn min(self, other: $t) -> $t {
                    cmp::min(self, other)
                }
                #[inline(always)]
                fn max(self, other: $t) -> $t {
                    cmp::max(self, other)
                }
            }
            impl BaseInt for $t {}
        )+
    }
);
impl_int! { i32, u32 }

macro_rules! impl_flt(
    ($t: ident) => {
        impl Primitive for $t {}
        impl SignedNum for $t {
            #[inline(always)]
            fn abs(&self) -> $t {
                Float::abs(*self)
            }
            #[inline(always)]
            fn sign(&self) -> $t {
                let l = $t::zero();
                if self.is_zero() {
                    l
                } else {
                    self.signum()
                }
            }
        }
        impl ApproxEq for $t {
            type BaseType = $t;
            #[inline(always)]
            fn is_close_to(&self, rhs: &$t, max_diff: $t) -> bool {
                (self - *rhs).abs() <= max_diff
            }
        }
        impl BaseNum for $t {
            #[inline(always)]
            fn min(self, other: $t) -> $t {
                Float::min(self, other)
            }
            #[inline(always)]
            fn max(self, other: $t) -> $t {
                Float::max(self, other)
            }
        }
        impl BaseFloat for $t {
            #[inline(always)]
            fn to_degrees(self) -> $t {
                self * (180. / $t::consts::PI)
            }
            #[inline(always)]
            fn to_radians(self) -> $t {
                self * ($t::consts::PI / 180.)
            }
            #[inline(always)]
            fn frexp(self) -> ($t, isize) {
                // CHECK: use impl in `libstd` after it's stable.
                if self.is_zero() || self.is_infinite() || self.is_nan() {
                    (self, 0)
                } else {
                    let lg = self.abs().log2();
                    let x = (lg.fract() - 1.).exp2();
                    let exp = lg.floor() + 1.;
                    (self.signum() * x, exp as isize)
                }
            }
            #[inline(always)]
            fn ldexp(self, exp: isize) -> $t {
                // CHECK: use impl in `libstd` after it's stable.
                let f = exp as $t;
                self * f.exp2()
            }
        }
    }
);

impl_flt! { f32 }
impl_flt! { f64 }
