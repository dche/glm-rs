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

// The GLSL Specification, ch 8.3, Common Functions.

use basenum::*;
use traits::*;
use vec::traits::{ GenVec, GenFloatVec, GenNumVec };
use vec::vec::{ Vector2, Vector3, Vector4 };
use std::mem;
use std::ops::Rem;
use num::{ Float, Zero };

pub trait FloatIntRel<E: BaseFloat, I: BaseInt, GI: GenInt<I>>: GenFloat<E> {
    // float -> int
    fn map_int<F: Fn(E) -> I>(&self, fun: F) -> GI;
    // (float) -> (float, int)
    fn split_int<F: Fn(E) -> (E, I)>(&self, fun: F) -> (Self, GI);
}

pub trait IntFloatRel<I: BaseInt, E: BaseFloat, GF: GenFloat<E>>: GenInt<I> {
    // int -> float
    fn map_flt<F: Fn(I) -> E>(&self, fun: F) -> GF;
    // (int, float) -> float
    fn zip_flt<F: Fn(I, E) -> E>(&self, f: &GF, fun: F) -> GF;
}

pub trait NumBoolRel<N: BaseNum, B: GenBType>: GenNum<N> {
    // num -> bool
    fn map_bool<F: Fn(N) -> bool>(&self, fun: F) -> B;
    // (num, bool) -> num
    fn zip_bool<F: Fn(N, bool) -> N>(&self, b: &B, fun: F) -> Self;
}

macro_rules! impl_scalar_FloatIntRel {
    ($($t: ident),+) => {
        $(
            impl<I: BaseInt + GenInt<I>> FloatIntRel<$t, I, I> for $t {
                #[inline(always)]
                fn map_int<F: Fn($t) -> I>(&self, fun: F) -> I {
                    fun(*self)
                }
                #[inline(always)]
                fn split_int<F: Fn($t) -> ($t, I)>(&self, fun: F) -> ($t, I) {
                    fun(*self)
                }
            }
        )+
    }
}

impl_scalar_FloatIntRel! { f32, f64 }

macro_rules! impl_vec_FloatIntRel {
    ($({ $t: ident, $($field: ident),+ }),+) => {
        $(
            impl<E: BaseFloat, I: BaseInt> FloatIntRel<E, I, $t<I>> for $t<E> {
                #[inline]
                fn map_int<F: Fn(E) -> I>(&self, fun: F) -> $t<I> {
                    $t::new($(fun(self.$field)),+)
                }
                #[inline]
                fn split_int<F: Fn(E) -> (E, I)>(&self, fun: F) -> ($t<E>, $t<I>) {
                    let mut f = $t::<E>::zero();
                    let mut i = $t::<I>::zero();
                    let dim = $t::<E>::dim();
                    for j in 0..dim {
                        let (a, b) = fun(self[j]);
                        f[j] = a;
                        i[j] = b;
                    }
                    (f, i)
                }
            }
        )+
    }
}

impl_vec_FloatIntRel! {
    { Vector2, x, y },
    { Vector3, x, y, z },
    { Vector4, x, y, z, w }
}

macro_rules! impl_scalar_IntFloatRel {
    ($($t: ident),+) => {
        $(
            impl<E: BaseFloat + GenFloat<E>> IntFloatRel<$t, E, E> for $t {
                #[inline]
                fn map_flt<F: Fn($t) -> E>(&self, fun: F) -> E {
                    fun(*self)
                }
                #[inline]
                fn zip_flt<F: Fn($t, E) -> E>(&self, f: &E, fun: F) -> E {
                    fun(*self, *f)
                }
            }
        )+
    }
}

impl_scalar_IntFloatRel! { i32, u32 }

macro_rules! impl_vec_IntFloatRel {
    ($({ $t: ident, $($field: ident),+ }),+) => {
        $(
            impl<I: BaseInt, E: BaseFloat> IntFloatRel<I, E, $t<E>> for $t<I> {
                #[inline]
                fn map_flt<F: Fn(I) -> E>(&self, fun: F) -> $t<E> {
                    $t::new($(fun(self.$field)),+)
                }
                #[inline]
                fn zip_flt<F: Fn(I, E) -> E>(&self, f: &$t<E>, fun: F) -> $t<E> {
                    $t::new($(fun(self.$field, f.$field)),+)
                }
            }
        )+
    }
}

impl_vec_IntFloatRel! {
    { Vector2, x, y },
    { Vector3, x, y, z },
    { Vector4, x, y, z, w }
}

macro_rules! impl_scalar_NumBoolRel {
    ($($t: ident),+) => {
        $(
            impl NumBoolRel<$t, bool> for $t {
                #[inline]
                fn map_bool<F: Fn($t) -> bool>(&self, fun: F) -> bool {
                    fun(*self)
                }
                #[inline]
                fn zip_bool<F: Fn($t, bool) -> $t>(&self, b: &bool, fun: F) -> $t {
                    fun(*self, *b)
                }
            }
        )+
    }
}

impl_scalar_NumBoolRel! { i32, u32, f32, f64 }

macro_rules! impl_vec_NumBoolRel {
    ($({ $t: ident, $($field: ident),+ }),+) => {
        $(
            impl<N: BaseNum> NumBoolRel<N, $t<bool>> for $t<N> {
                #[inline]
                fn map_bool<F: Fn(N) -> bool>(&self, fun: F) -> $t<bool> {
                    $t::new($(fun(self.$field)),+)
                }
                #[inline]
                fn zip_bool<F: Fn(N, bool) -> N>(&self, b: &$t<bool>, fun: F) -> $t<N> {
                    $t::new($(fun(self.$field, b.$field)),+)
                }
            }
        )+
    }
}

impl_vec_NumBoolRel! {
    { Vector2, x, y },
    { Vector3, x, y, z },
    { Vector4, x, y, z, w }
}

/// Returns `x` if `x ≥ 0`, otherwise it returns `–x`.
///
/// # Example
///
/// ```
/// use glm::{ abs, dvec4, SignedNum };
///
/// assert_eq!(abs(-1_f32), 1.);
/// let v4 = dvec4(0., 100., -2., -3.);
/// assert_eq!(abs(v4), v4.abs());
/// assert_eq!(abs(v4), dvec4(0., 100., 2., 3.));
/// ```
#[inline(always)]
pub fn abs<S: SignedNum + BaseNum, T: GenNum<S>>(x: T) -> T {
    x.map(|s: S| {
        SignedNum::abs(&s)
    })
}

/// Returns `1.0` if `x > 0`, `0.0` if `x = 0`, or `–1.0` if `x < 0`.
///
/// # Example
///
/// ```
/// use glm::{ sign, vec3 };
///
/// assert_eq!(sign(-0_f32), 0.);
/// assert_eq!(sign(vec3(-100., 2., 0.)), vec3(-1., 1., 0.));
/// ```
#[inline(always)]
pub fn sign<S: SignedNum + BaseNum, T: GenNum<S>>(x: T) -> T {
    x.map(|s: S| {
        SignedNum::sign(&s)
    })
}

/// Returns a value equal to the nearest integer that is less than or
/// equal to `x`.
///
/// # Example
///
/// ```
/// assert_eq!(glm::floor(-3.14_f32), -4.);
/// assert_eq!(glm::floor(glm::vec3(-1.7, 4., 4.9)), glm::vec3(-2., 4., 4.));
/// ```
#[inline(always)]
pub fn floor<F: BaseFloat, T: GenFloat<F>>(x: T) -> T {
    x.map(Float::floor)
}

/// Returns a value equal to the nearest integer to `x` whose absolute value
/// is not larger than the absolute value of `x`.
///
/// # Example
///
/// ```
/// assert_eq!(glm::trunc(-3.14_f32), -3.);
/// assert_eq!(glm::trunc(glm::vec3(-1.7, 4., 4.9)), glm::vec3(-1., 4., 4.));
/// ```
#[inline(always)]
pub fn trunc<F: BaseFloat, T: GenFloat<F>>(x: T) -> T {
    x.map(Float::trunc)
}

/// Returns a value equal to the nearest integer to `x`.
///
/// The fraction `0.5` will round in a direction chosen by the implementation,
/// presumably the direction that is fastest. This includes the possibility
/// that `round(x)` returns the same value as `roundEven(x)` for all values of
/// `x`.
///
/// # Example
///
/// ```
/// use glm::{ round, vec3 };
///
/// assert_eq!(round(-3.14_f32), -3.);
/// assert_eq!(round(vec3(-1.7, 4., 4.9)), vec3(-2., 4., 5.));
/// ```
#[inline(always)]
pub fn round<F: BaseFloat, T: GenFloat<F>>(x: T) -> T {
    x.map(Float::round)
}

/// Returns a value equal to the nearest integer to `x`.
///
/// A fractional part of `0.5` will round toward the nearest even integer.
/// (Both `3.5` and `4.5` for x will return `4.0`.)
///
/// # Example
///
/// ```
/// use glm::{ roundEven, vec4 };
///
/// assert_eq!(roundEven(2.5_f32), 2.);
/// assert_eq!(roundEven(1.5_f32), 2.);
/// assert_eq!(roundEven(vec4(3.14, -3.14, -1.5, -2.5)), vec4(3., -3., -2., -2.));
/// ```
#[inline(always)]
#[allow(non_snake_case)]
pub fn roundEven<F: BaseFloat, T: GenFloat<F>>(x: T) -> T {
    x.map(|f| -> F {
        let ling = F::zero();
        let yi = F::one();
        let er = yi + yi;

        let int = f.trunc();
        if f.fract().abs() != F::from(0.5).unwrap() {
            f.round()
        } else if int % er == ling {
            int
        } else if int < ling {
            int - yi
        } else {
            int + yi
        }
    })
}

/// Returns a value equal to the nearest integer that is greater than or
/// equal to `x`.
///
/// # Example
///
/// ```
/// use glm::{ ceil, dvec3 };
///
/// assert_eq!(ceil(-3.14_f32), -3.);
/// assert_eq!(ceil(3.14_f32), 4.);
/// assert_eq!(ceil(dvec3(-1.8, 1., 1.8)), dvec3(-1., 1., 2.));
/// ```
#[inline(always)]
pub fn ceil<F: BaseFloat, T: GenFloat<F>>(x: T) -> T {
    x.map(Float::ceil)
}

/// Returns `x – floor(x)`.
///
/// # Example
///
/// ```
/// use glm::{ fract, vec2 };
///
/// assert_eq!(fract(3.25_f32), 0.25);
/// assert_eq!(fract(vec2(-1.5, 1.)), vec2(-0.5, 0.));
/// ```
#[inline(always)]
pub fn fract<F: BaseFloat, T: GenFloat<F>>(x: T) -> T {
    x.map(Float::fract)
}

/// Modulus. Returns `x – y ∗ floor(x/y)`.
///
/// # Note
///
/// Original function name `mod` is renamed to `fmod` because **mod** is
/// a keyword in Rust.
///
/// # Example
///
/// ```
/// assert_eq!(glm::fmod(3.5_f32, 3.), 0.5);
/// ```
#[inline(always)]
pub fn fmod<F: BaseFloat, T: GenFloat<F>>(x: T, y: T) -> T {
    x.zip(y, Rem::rem)
}

/// Modulus with a scalar number.
///
/// # Note
///
/// `mod_s` is not a GLSL function name. It is a variant of original `mod`
/// function and is introduced because Rust does not support function name
/// overloading.
///
/// # Example
///
/// ```
/// use glm::{ mod_s, dvec3 };
///
/// let v = dvec3(-1.5, 1.5, 10.5);
/// assert_eq!(mod_s(v, 2.), dvec3(-1.5, 1.5, 0.5));
/// ```
#[inline(always)]
pub fn mod_s<F: BaseFloat, T: GenFloatVec<F>>(x: T, y: F) -> T {
    x.map(|f| -> F {
        f.rem(y)
    })
}

/// Returns the fractional and integer parts of `x`.
///
/// Both parts will have the same sign as `x`.
///
/// # Note
///
/// In GLSL, the integer part is returned via a output parameter `i`.
/// In Rust we can return both parts using a tuple *(interger part, fractional part)*.
///
/// # Example
///
/// ```
/// use glm::{ modf, vec3 };
///
/// assert_eq!(modf(1.5_f32), (1., 0.5));
/// assert_eq!(modf(vec3(0., -1.25, 3.75)), (vec3(0., -1., 3.), vec3(0., -0.25, 0.75)));
/// ```
#[inline(always)]
pub fn modf<F: BaseFloat, T: GenFloat<F>>(x: T) -> (T, T) {
    (trunc(x), fract(x))
}

/// Returns `y` if `y < x`, otherwise it returns `x`.
///
/// # Example
///
/// ```
/// let v1 = glm::vec2(1., 4.);
/// let v2 = glm::vec2(2., 3.);
/// assert_eq!(glm::min(v1, v2), glm::vec2(1., 3.));
/// ```
#[inline(always)]
pub fn min<S: BaseNum, T: GenNum<S>>(x: T, y: T) -> T {
    x.zip(y, BaseNum::min)
}

/// A variant of function `min` that uses a scalar value as comparator.
///
/// # Note
///
/// `mod_s` is not a GLSL function name.
///
/// # Example
///
/// ```
/// let v = glm::vec2(1., 3.);
/// let y = 2_f32;
/// assert_eq!(glm::min_s(v, y), glm::vec2(1., 2.));
/// ```
#[inline(always)]
pub fn min_s<S: BaseNum, T: GenNumVec<S>>(x: T, y: S) -> T {
    x.map(|c| -> S {
        BaseNum::min(c, y)
    })
}

/// Returns `y` if `x < y`, otherwise it returns `x`.
///
/// # Example
///
/// ```
/// let v1 = glm::vec2(1., 4.);
/// let v2 = glm::vec2(2., 3.);
/// assert_eq!(glm::max(v1, v2), glm::vec2(2., 4.));
/// ```
#[inline(always)]
pub fn max<S: BaseNum, T: GenNum<S>>(x: T, y: T) -> T {
    x.zip(y, BaseNum::max)
}

/// A variant of `max` that always uses a scalar value as the comparator.
///
/// # Note
///
/// `max_s` is not a GLSL function name. It is introduced because Rust does
/// not support function name overloading.
///
/// # Example
///
/// ```
/// let v = glm::vec2(1., 3.);
/// let y = 2_f32;
/// assert_eq!(glm::max_s(v, y), glm::vec2(2., 3.));
/// ```
#[inline(always)]
pub fn max_s<S: BaseNum, T: GenNumVec<S>>(x: T, y: S) -> T {
    x.map(|s| -> S { BaseNum::max(s, y) })
}

/// Returns `min (max (x, min_val), max_val)`.
///
/// # Example
///
/// ```
/// use glm::{ clamp, vec3 };
///
/// assert_eq!(clamp(3.14_f32, 0., 1.), 1.);
/// let v = vec3(-1., 0., 100.);
/// let min = vec3(0., 0., 0.);
/// let max = vec3(1., 1., 1.);
/// assert_eq!(clamp(v, min, max), vec3(0., 0., 1.));
/// ```
#[inline(always)]
pub fn clamp<S: BaseNum, T: GenNum<S>>(x: T, min_val: T, max_val: T) -> T {
    min(max(x, min_val), max_val)
}

/// A variant of function `clamp` that uses scalar values as thresholds.
///
/// # Note
///
/// `clamp_s` is not a GLSL function name.
///
/// # Example
///
/// ```
/// use glm::{ clamp_s, dvec2 };
///
/// let dv2 = dvec2(-1., 3.14);
/// assert_eq!(clamp_s(dv2, 0., 1.), dvec2(0., 1.));
/// ```
#[inline(always)]
pub fn clamp_s<S: BaseNum, T: GenNumVec<S>>(x: T, min_val: S, max_val: S) -> T {
    min_s(max_s(x, min_val), max_val)
}

/// Returns the linear blend of `x` and `y`, i.e., `x⋅(1−a)+y⋅a`.
///
/// # Example
///
/// ```
/// use glm::{ mix, vec2 };
///
/// assert_eq!(mix(0_f32, 1_f32, 0.5), 0.5);
/// let x = vec2(1., 2.);
/// let y = vec2(3., 4.);
/// let a = vec2(0.5, 2.);
/// let r = vec2(2., 6.);
/// assert_eq!(mix(x, y, a), r);
/// ```
#[inline(always)]
pub fn mix<F: BaseFloat, T: GenFloat<F>>(x: T, y: T, a: T) -> T {
    let yi = T::one();
    x * (yi - a) + y * a
}

/// A variant of function `mix` that parameter `a` is a scalar.
///
/// # Note
///
/// `mix_s` is not a GLSL function name. It is introduced because Rust does
/// not support function name overloading.
///
/// # Example
///
/// ```
/// use glm::{ mix_s, dvec3 };
///
/// let x = dvec3(10., 20., 30.);
/// let y = dvec3(100., 200., 300.);
/// assert_eq!(mix_s(x, y, 0.5), dvec3(55., 110., 165.));
/// ```
#[inline(always)]
pub fn mix_s<F: BaseFloat, T: GenFloatVec<F>>(x: T, y: T, a: F) -> T {
    let yi = F::one();
    x * (yi - a) + y * a
}

/// Selects which vector each returned component comes from.
///
/// For a component of `a` that is `false`, the corresponding component of `x`
/// is returned. For a component of `a` that is `true`, the corresponding
/// component of `y` is returned. Components of `x` and `y` that are not
/// selected are allowed to be invalid floating point values and will have no
/// effect on the results.
///
/// # Note
///
/// 1. `mix_bool` is not a GLSL function name. It is a variant of `mix`
///    function and is introduced becasue Rust does not support function name
///    overloading.
/// 1. This function works for scalar types too.
///
/// # Example
///
/// ```
/// use glm::{ bvec4, dvec4, mix_bool };
///
/// let a = bvec4(true, false, false, true);
/// let x = dvec4(1., 1., 1., 1.);
/// let y = dvec4(2., 2., 2., 2.);
/// assert_eq!(mix_bool(x, y, a), dvec4(2., 1., 1., 2.));
/// // works for scalars too.
/// assert_eq!(mix_bool(1_f32, 2., false), 1.);
/// ```
#[inline(always)]
pub fn mix_bool
<
F: BaseFloat,
B: GenBType,
T: NumBoolRel<F, B>
>(x: T, y: T, a: B) -> T {
    let ling = F::zero();
    x.zip_bool(&a, |f, b| -> F {
        if b {
            ling
        } else {
            f
        }
    }) +
    y.zip_bool(&a, |f, b| -> F {
        if b {
            f
        } else {
            ling
        }
    })
}

/// Returns `0.0` if `x` < `edge`, otherwise it returns `1.0`.
///
/// # Example
///
/// ```
/// use glm::{ step, dvec2 };
/// assert_eq!(step(1f32, 1.), 1.);
/// assert_eq!(step(dvec2(1., 2.), dvec2(2., 1.)), dvec2(1., 0.));
/// ```
#[inline(always)]
pub fn step<F: BaseFloat, T: GenFloat<F>>(edge: T, x: T) -> T {
    x.zip(edge, |f, e| -> F {
        if f < e {
            F::zero()
        } else {
            F::one()
        }
    })
}

/// A variant of `step` function that use scalar as the edge.
///
/// # Note
///
/// `step_s` is not a GLSL function name.
///
/// # Example
///
/// ```
/// use glm::{ step_s, vec3 };
///
/// assert_eq!(step_s(0_f32, vec3(-1., 0., 1.)), vec3(0., 1., 1.));
/// ```
#[inline(always)]
pub fn step_s<F: BaseFloat, T: GenFloatVec<F>>(edge: F, x: T) -> T {
    x.map(|f| -> F {
        if f < edge {
            F::zero()
        } else {
            F::one()
        }
    })
}

/// Returns `0.0` if `x ≤ edge0` and `1.0` if `x ≥ edge1` and performs
/// smooth Hermite interpolation between 0 and 1 when `edge0 < x < edge1`.
///
/// This is useful in cases where you would want a threshold function with
/// a smooth transition.
///
/// Results are undefined if `edge0 ≥ edge1`.
///
/// # Example
///
/// ```
/// use glm::{ smoothstep, dvec2 };
///
/// assert_eq!(smoothstep(0f32, 1., -1.), 0.);
/// let e0 = dvec2(0., -100.);
/// let e1 = dvec2(1., 100.);
/// let v = dvec2(1., 50.);
/// assert_eq!(smoothstep(e0, e1, v), dvec2(1., 0.84375));
/// ```
#[inline]
pub fn smoothstep<F: BaseFloat, T: GenFloat<F>>(edge0: T, edge1: T, x: T) -> T {
    let ling = T::zero();
    let yi = T::one();
    let er = yi + yi;
    let san = er + yi;

    let t = clamp((x - edge0) / (edge1 - edge0), ling, yi);
    t * t * (t * -er + san)
}

/// A variant of `smoothstep` function that use scalar as edges.
///
/// # Note
///
/// `smoothstep_s` is not a GLSL function name.
#[inline]
pub fn smoothstep_s
<
F: BaseFloat + GenNum<F>,
T: GenFloatVec<F>
>(edge0: F, edge1: F, x: T) -> T {
    let ling = F::zero();
    let yi = F::one();
    let er = yi + yi;
    let san = er + yi;

    x.map(|f| -> F {
        let t = clamp((f - edge0) / (edge1 - edge0), ling, yi);
        t * t * (san - er * t)
    })
}

/// Returns `true` if `x` holds a *NaN*. Returns `false` otherwise.
///
/// # Example
///
/// ```
/// # extern crate glm;
/// # extern crate num;
/// # fn main() {
/// use glm::{ bvec3, dvec3, isnan };
/// use num::Float;
///
/// let nan: f64 = Float::nan();
/// assert!(isnan(nan));
/// assert_eq!(isnan(dvec3(nan, 1., -0.)), bvec3(true, false, false));
/// # }
/// ```
#[inline(always)]
pub fn isnan<F: BaseFloat, B: GenBType, T: NumBoolRel<F, B>>(x: T) -> B {
    x.map_bool(Float::is_nan)
}

/// Returns true if x holds a positive infinity or negative infinity.
/// Returns false otherwise.
///
/// # Example
///
/// ```
/// # extern crate glm;
/// # extern crate num;
/// # fn main() {
/// use num::Float;
///
/// let inf: f32 = Float::infinity();
/// assert!(glm::isinf(inf));
/// assert_eq!(glm::isinf(glm::vec2(inf, 0.)), glm::bvec2(true, false));
/// # }
/// ```
#[inline(always)]
pub fn isinf<F: BaseFloat, B: GenBType, T: NumBoolRel<F, B>>(x: T) -> B {
    x.map_bool(Float::is_infinite)
}

/// Returns a signed integer value representing the encoding of
/// a floating-point value.
///
/// The floating-point value's bit-level representation is preserved.
///
/// # Example
///
/// ```
/// # extern crate glm;
/// # extern crate num;
/// # fn main() {
/// use glm::*;
/// use num::Float;
///
/// let f = 1_f32;
/// let i = floatBitsToInt(f);
/// assert_eq!(i, 0x3F800000);
/// let inf: f32 = Float::infinity();
/// let v = vec3(0.2, 0., inf);
/// assert_eq!(floatBitsToInt(v), ivec3(0x3E4CCCCD, 0, 0x7f800000));
/// # }
/// ```
#[inline(always)]
#[allow(non_snake_case)]
pub fn floatBitsToInt<G: GenIType, T: FloatIntRel<f32, i32, G>>(value: T) -> G {
    value.map_int(|f| -> i32 {
        let i: i32 = unsafe { mem::transmute(f) };
        i
    })
}

/// Returns a unsigned integer value representing the encoding of
/// a floating-point value.
///
/// The floating- point value's bit-level representation is preserved.
///
/// # Example
///
/// ```
/// # extern crate glm;
/// # extern crate num;
/// # fn main() {
/// use glm::{ floatBitsToUint, vec3, uvec3 };
/// use num::Float;
///
/// let f = 1_f32;
/// let u = floatBitsToUint(f);
/// assert_eq!(u, 0x3F800000);
/// let inf: f32 = Float::infinity();
/// let v = vec3(0.2, 0., inf);
/// assert_eq!(floatBitsToUint(v), uvec3(0x3E4CCCCD, 0, 0x7f800000));
/// # }
/// ```
#[inline(always)]
#[allow(non_snake_case)]
pub fn floatBitsToUint<G: GenUType, T: FloatIntRel<f32, u32, G>>(value: T) -> G {
    value.map_int(|f| -> u32 {
        let u: u32 = unsafe { mem::transmute(f) };
        u
    })
}

/// Returns a floating-point value corresponding to a signed integer encoding
/// of a floating-point value.
///
/// # Example
///
/// ```
/// # extern crate glm;
/// # extern crate num;
/// # fn main() {
/// use glm::{ intBitsToFloat, vec3, ivec3 };
/// use num::Float;
///
/// let i: i32 = 0x3F800000;
/// let f = intBitsToFloat(i);
/// assert_eq!(f, 1.);
/// let inf: f32 = Float::infinity();
/// let vi = ivec3(0x3E4CCCCD, 0, 0x7f800000);
/// let vf = vec3(0.2, 0., inf);
/// assert_eq!(intBitsToFloat(vi), vf);
/// # }
/// ```
#[inline(always)]
#[allow(non_snake_case)]
pub fn intBitsToFloat<G: GenType, T: IntFloatRel<i32, f32, G>>(value: T) -> G {
    value.map_flt(|i| -> f32 {
        let f: f32 = unsafe { mem::transmute(i) };
        f
    })
}

/// Returns a floating-point value corresponding to a unsigned integer encoding
/// of a floating-point value.
///
/// # Example
///
/// ```
/// # extern crate glm;
/// # extern crate num;
/// # fn main() {
/// use glm::{ uintBitsToFloat, vec3, uvec3 };
/// use num::Float;
///
/// let i: u32 = 0x3F800000;
/// let f = uintBitsToFloat(i);
/// assert_eq!(f, 1.);
/// let inf: f32 = Float::infinity();
/// let vu = uvec3(0x3E4CCCCD, 0, 0x7f800000);
/// let vf = vec3(0.2, 0., inf);
/// assert_eq!(uintBitsToFloat(vu), vf);
/// # }
/// ```
#[inline(always)]
#[allow(non_snake_case)]
pub fn uintBitsToFloat<G: GenType, T: IntFloatRel<u32, f32, G>>(value: T) -> G {
    value.map_flt(|u| -> f32 {
        let f: f32 = unsafe { mem::transmute(u) };
        f
    })
}

/// Computes and returns `a * b + c`.
///
/// # Example
///
/// ```
/// use glm::{ fma, vec3 };
/// assert_eq!(fma(1.5_f32, 2.25, 3.125), 6.5);
/// let a = vec3(-1., 2., 3.);
/// let b = vec3(4., 5., 6.);
/// let c = vec3(7., 8., 0.);
/// assert_eq!(fma(a, b, c), vec3(3., 18., 18.));
/// ```
#[inline(always)]
pub fn fma<F: BaseFloat, T: GenFloat<F>>(a: T, b: T, c: T) -> T {
    a.fma(&b, &c)
}

/// Splits `x` into a floating-point significand in the range [0.5, 1.0) and
/// an integral exponent of two, such that:
/// *x = significand⋅2<sup>exponent</sup>*.
///
/// For a floating-point value of zero, the significant and exponent are both
/// zero.
///
/// For a floating-point value that is an infinity or is not a number,
/// the results are undefined.
///
/// # Note
///
/// In GLSL, the significand is returned by the function and the exponent is
/// returned in the output parameter `exp`. In Rust, we have the luxury to
/// return both of them very naturally via a tuple.
///
/// # Example
///
/// ```
/// use glm::{ frexp, dvec3, ivec3 };
///
/// assert_eq!(frexp(0_f32), (0., 0));
/// let v3 = dvec3(1024., 1., 3.);
/// let s = dvec3(0.5, 0.5, glm::exp2(glm::log2(3.) - 2.));
/// let e = ivec3(11, 1, 2);
/// assert_eq!((s, e), frexp(v3));
/// ```
#[inline(always)]
pub fn frexp
<
F: BaseFloat,
I: GenIType,
T: FloatIntRel<F, i32, I>
>(x: T) -> (T, I) {
    x.split_int(|f| -> (F, i32) {
        let (s, e) = BaseFloat::frexp(f);
        (s, e as i32)
    })
}

/// Builds a floating-point number from `x` and the corresponding integral
/// exponent of two in `exp`, returning:
/// *significand ⋅ 2<sup>exponent</sup>*.
///
/// If this product is too large to be represented in the floating-point type,
/// the result is undefined.
///
/// # Example
///
/// ```
/// use glm::{ ldexp, vec3, ivec3 };
///
/// assert_eq!(ldexp(2_f32, 2), 8.);
/// let vf = vec3(1., 2., 3.);
/// let vi = ivec3(-1, 1, 2);
/// assert_eq!(ldexp(vf, vi), vec3(0.5, 4., 12.));
/// ```
#[inline(always)]
pub fn ldexp
<
F: BaseFloat,
G: GenFloat<F>,
T: IntFloatRel<i32, F, G>
>(x: G, exp: T) -> G {
    exp.zip_flt(&x, |i, f| -> F {
        BaseFloat::ldexp(f, i as isize)
    })
}
