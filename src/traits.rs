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

use basenum::{ BaseNum, BaseInt, BaseFloat, SignedNum, ApproxEq };
use std::ops::{ Add, Mul, Sub, Div, Rem, Not, BitAnd, BitOr, BitXor, Shl, Shr };
use rand::Rand;
use num::{ Float, One, Zero };

// TODO: use associated types to reduce redundant type parameters.

/// Generic numeric type.
pub trait GenNum<E: BaseNum>
: Copy
+ Sized
+ Clone
+ One
+ Zero
+ Div<Self, Output = Self>
+ Rem<Self, Output = Self>
+ Add<E, Output = Self>
+ Mul<E, Output = Self>
+ Rand
{
    /// Constructs from a scalar number.
    fn from_s(x: E) -> Self;

    fn map<F>(self, f: F) -> Self where F: Fn(E) -> E;

    fn zip<F>(self, y: Self, f: F) -> Self where F: Fn(E, E) -> E;

    fn split<F>(self, f: F) -> (Self, Self) where F: Fn(E) -> (E, E);

    fn map2<F>(self, y: Self, f: F) -> (Self, Self) where F: Fn(E, E) -> (E, E);
}

macro_rules! impl_GenNum_for_scalar(
    ($t: ty) => {
        impl GenNum<$t> for $t {
            #[inline(always)]
            fn from_s(x: $t) -> Self {
                x
            }
            #[inline(always)]
            fn map<F: Fn($t) -> $t>(self, f: F) -> $t {
                f(self)
            }
            #[inline(always)]
            fn zip<F: Fn($t, $t) -> $t>(self, y: $t, f: F) -> $t {
                f(self, y)
            }
            #[inline(always)]
            fn split<F: Fn($t) -> ($t, $t)>(self, f: F) -> ($t, $t) {
                f(self)
            }
            #[inline(always)]
            fn map2<F: Fn($t, $t) -> ($t, $t)>(self, y: $t, f: F) -> ($t, $t) {
                f(self, y)
            }
        }
    }
);

/// Generic interger type.
pub trait GenInt<I: BaseInt>
: GenNum<I>
+ Eq
+ Not<Output = Self>
+ BitAnd<Output = Self>
+ BitOr<Output = Self>
+ BitXor<Output = Self>
+ Shl<usize, Output = Self>
+ Shr<usize, Output = Self>
{}

/// Generic signed integer type.
///
/// # Note
///
/// Only 32-bit integer is used in GLSL.
pub trait GenIType: GenInt<i32> + SignedNum + Sub<i32, Output = Self> {}

impl_GenNum_for_scalar! { i32 }
impl GenInt<i32> for i32 {}
impl GenIType for i32 {}

/// Generic unsigned integer type.
///
/// # Note
///
/// Only 32-bit unsigned integer is used in GLSL.
pub trait GenUType: GenInt<u32> {}

impl_GenNum_for_scalar! { u32 }
impl GenInt<u32> for u32 {}
impl GenUType for u32 {}

/// Generic float number type.
pub trait GenFloat<F: BaseFloat>
: GenNum<F>
+ ApproxEq<BaseType = F>
+ SignedNum
+ Sub<F, Output = Self>
{
    /// Computes and returns `a * b + c`.
    fn fma(&self, b: &Self, c: &Self) -> Self;
}

pub trait GenType: GenFloat<f32> {}
pub trait GenDType: GenFloat<f64> {}

macro_rules! impl_GenFloat_for_scalar(
    ($t: ty, $gt: path) => {
        impl_GenNum_for_scalar! { $t }
        impl GenFloat<$t> for $t {
            fn fma(&self, b: &$t, c: &$t) -> $t {
                Float::mul_add(*self, *b, *c)
            }
        }
        impl $gt for $t {}
    }
);

impl_GenFloat_for_scalar! { f32, GenType }
impl_GenFloat_for_scalar! { f64, GenDType }

/// Generic boolean type.
pub trait GenBType: Eq {}

impl GenBType for bool {}
