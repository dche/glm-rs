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

use basenum::Primitive;
use vec::traits::GenVec;
use vec::vec::*;
use std::default::Default;
use num::{ ToPrimitive, Zero };

/// This trait is like the `std::num::ToPrimitive`, but function `to_bool()`
/// is added.
pub trait ToPrim: Primitive {

    fn to_i32(&self) -> Option<i32>;

    fn to_u32(&self) -> Option<u32>;

    fn to_f32(&self) -> Option<f32>;

    fn to_f64(&self) -> Option<f64>;

    fn to_bool(&self) -> Option<bool>;
}

macro_rules! impl_ToPrim_for {
    ($($t: ident),+) => {
        $(
            impl ToPrim for $t {
                #[inline]
                fn to_i32(&self) -> Option<i32> {
                    ToPrimitive::to_i32(self)
                }
                #[inline]
                fn to_u32(&self) -> Option<u32> {
                    ToPrimitive::to_u32(self)
                }
                #[inline]
                fn to_f32(&self) -> Option<f32> {
                    ToPrimitive::to_f32(self)
                }
                #[inline]
                fn to_f64(&self) -> Option<f64> {
                    ToPrimitive::to_f64(self)
                }
                #[inline]
                fn to_bool(&self) -> Option<bool> {
                    let b = if self.is_zero() { false } else { true };
                    Some(b)
                }
            }
        )+
    };
}

impl_ToPrim_for! { i32, u32, f32, f64 }

impl ToPrim for bool {
    #[inline]
    fn to_i32(&self) -> Option<i32> {
        let i = if *self { 1 } else { 0 };
        Some(i)
    }
    #[inline]
    fn to_u32(&self) -> Option<u32> {
        let i = if *self { 1 } else { 0 };
        Some(i)
    }
    #[inline]
    fn to_f32(&self) -> Option<f32> {
        let i = if *self { 1. } else { 0. };
        Some(i)
    }
    #[inline]
    fn to_f64(&self) -> Option<f64> {
        let i = if *self { 1. } else { 0. };
        Some(i)
    }
    #[inline]
    fn to_bool(&self) -> Option<bool> {
        Some(*self)
    }
}

/// This trait provides parameterized function `from`.
pub trait PrimCast: ToPrim {

    /// Converts from a value with primitive type `p`.
    fn from<F: ToPrim>(p: F) -> Option<Self>;
}

macro_rules! impl_PrimCast_for {
    ($({ $t: ident, $cf: ident }),+) => {
        $(
            impl PrimCast for $t {
                #[inline]
                fn from<F: ToPrim>(p: F) -> Option<$t> {
                    p.$cf()
                }
            }
        )+
    }
}

impl_PrimCast_for! {
    { i32, to_i32 },
    { u32, to_u32 },
    { f32, to_f32 },
    { f64, to_f64 },
    { bool, to_bool }
}

/// This trait unifies all scalar and vector types, so we can convert between
/// any two of them.
// TODO: move GenPrimitive to `traits.rs`?
//       and move functions defined in `GenNum` (map, zip, etc.) here?
pub trait GenPrimitive {
    type BaseType: PrimCast;
}

macro_rules! impl_GenPrimitive_for_scalar {
    ($($t: ident),+) => {
        $(
            impl GenPrimitive for $t {
                type BaseType = $t;
            }
        )+
    }
}

impl_GenPrimitive_for_scalar! { i32, u32, f32, f64, bool }

macro_rules! impl_GenPrimitive_for_vector {
    ($($t: ident),+) => {
        $(
            impl<T: PrimCast> GenPrimitive for $t<T> {
                type BaseType = T;
            }
        )+
    }
}

impl_GenPrimitive_for_vector! {
    Vector2, Vector3, Vector4
}

/// Traits for converting any scalar/vector value to a scalar.
pub trait ToScalar<F: PrimCast, T: PrimCast>: GenPrimitive<BaseType = F> {
    /// Casts _self_ to a value of type `T`.
    ///
    /// According to the GLSL spec, if _self_ is a vector, casts the first
    /// component only.
    ///
    /// # Example
    ///
    /// ```rust
    /// use glm::*;
    ///
    /// assert_eq!(int(bvec2(true, false)), 1);
    /// assert_eq!(int(3.14_f32), 3);
    /// ```
    fn to(self) -> Option<T>;
}

macro_rules! impl_ToScalar_for_scalar {
    ($($t: ident),+) => {
        $(
            impl<T: PrimCast> ToScalar<$t, T> for $t {
                #[inline(always)]
                fn to(self) -> Option<T> {
                    T::from(self)
                }
            }
        )+
    }
}

impl_ToScalar_for_scalar! { i32, u32, f32, f64, bool }

macro_rules! impl_ToScalar_for_vector {
    ($($t: ident),+) => {
        $(
            impl<F: PrimCast, T: PrimCast> ToScalar<F, T> for $t<F> {
                #[inline(always)]
                fn to(self) -> Option<T> {
                    T::from(self[0])
                }
            }
        )+
    }
}

impl_ToScalar_for_vector! { Vector2, Vector3, Vector4 }

macro_rules! def_cast_scalar_fun {
    ($({ $nm: ident, $t: ty }),+) => {
        $(
            pub fn $nm<B: PrimCast, F: ToScalar<B, $t>>(from: F) -> $t {
                from.to().unwrap()
            }
        )+
    }
}

def_cast_scalar_fun! {
    { int,      i32 },
    { uint,     u32 },
    { float,    f32 },
    { double,   f64 },
    { boolean,  bool }
}

/// Traits for converting any scalar/vector values to a vector.
pub trait ToVector
<
F: PrimCast,
T: PrimCast,
GT: GenPrimitive<BaseType = T> + GenVec<T>
>: GenPrimitive<BaseType = F> {

    /// Convertes _self_ to a value of Vector type.
    ///
    /// # Example
    ///
    /// ```rust
    /// use glm::*;
    ///
    /// assert_eq!(to_bvec2(0_i32), bvec2(false, false));
    /// assert_eq!(to_dvec4(bvec4(true, true, false, true)), dvec4(1., 1., 0., 1.));
    /// ```
    fn to(self) -> Option<GT>;
}

macro_rules! impl_ToVector_for_scalar {
    ($t: ident, $v: ident, $($field: ident),+) => {
        impl<T: PrimCast> ToVector<$t, T, $v<T>> for $t {
            #[inline]
            fn to(self) -> Option<$v<T>> {
                PrimCast::from(self).map(|t| -> $v<T> {
                    $v { $($field: t),+ }
                })
            }
        }
    }
}

macro_rules! impl_ToVectors_for_scalar {
    ($($t: ident),+) => {
        $(
            impl_ToVector_for_scalar! { $t, Vector2, x, y }
            impl_ToVector_for_scalar! { $t, Vector3, x, y, z }
            impl_ToVector_for_scalar! { $t, Vector4, x, y, z, w }
        )+
    }
}

impl_ToVectors_for_scalar! { i32, u32, f32, f64, bool }

// TODO: to support convertion between vectors with different dimensions.
//       by introducing something like `DimCast`.

macro_rules! impl_ToVector_for_vector {
    ($({ $v: ident, $($field: ident),+ }),+) => {
        $(
            impl<F: PrimCast, T: PrimCast + Default> ToVector<F, T, $v<T>> for $v<F> {
                #[inline]
                fn to(self) -> Option<$v<T>> {
                    let os = [$(T::from(self.$field)),+];
                    if os.iter().any(|&o| -> bool { o.is_none() }) {
                        None
                    } else {
                        // all Primitives implement `Default`.
                        let mut zero: $v<T> = $v { $($field: Default::default()),+ };
                        os.iter().fold(0, |i, &o| -> usize {
                            zero[i] = o.unwrap();
                            i + 1
                        });
                        Some(zero)
                    }
                }
            }
        )+
    }
}

impl_ToVector_for_vector! {
    { Vector2, x, y },
    { Vector3, x, y, z },
    { Vector4, x, y, z, w }
}

macro_rules! def_cast_vector_fun {
    ($({ $nm: ident, $s: ty, $v: ty }),+) => {
        $(
            #[inline]
            pub fn $nm<B: PrimCast, F: ToVector<B, $s, $v>>(gp: F) -> $v {
                gp.to().unwrap()
            }
        )+
    }
}

def_cast_vector_fun! {
    { to_ivec2, i32, IVec2 },
    { to_ivec3, i32, IVec3 },
    { to_ivec4, i32, IVec4 },
    { to_uvec2, u32, UVec2 },
    { to_uvec3, u32, UVec3 },
    { to_uvec4, u32, UVec4 },
    { to_vec2,  f32, Vec2 },
    { to_vec3,  f32, Vec3 },
    { to_vec4,  f32, Vec4 },
    { to_dvec2, f64, DVec2 },
    { to_dvec3, f64, DVec3 },
    { to_dvec4, f64, DVec4 },
    { to_bvec2, bool, BVec2 },
    { to_bvec3, bool, BVec3 },
    { to_bvec4, bool, BVec4 }
}

// TODO: support casting matrices to vectors. Just returns the first column.

#[cfg(test)]
mod test {

    use vec::vec::*;
    use super::*;

    #[test]
    fn test_int() {
        assert_eq!(int(1_f32), 1_i32);
        assert_eq!(int(true), 1_i32);
        assert_eq!(int(false), 0_i32);
    }

    #[test]
    fn test_boolean() {
        assert_eq!(boolean(uvec2(0, 1)), false);
        assert_eq!(boolean(vec3(1., -1., 0.)), true);
    }

    #[test]
    fn test_float() {
        assert_eq!(float(bvec2(true, false)), 1.);
        assert_eq!(float(123_u32), 123.);
        assert_eq!(float(0_f64), 0.);
    }

    #[test]
    fn test_to_vec() {
        assert_eq!(to_vec2(bvec2(true, false)), vec2(1., 0.));
        assert_eq!(to_bvec3(ivec3(0, 1, -1)), bvec3(false, true, true));
    }
}
